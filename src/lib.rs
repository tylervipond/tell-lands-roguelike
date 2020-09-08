use rltk::{GameState, Point, RandomNumberGenerator, Rltk, RltkBuilder};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate specs_derive;
extern crate serde;
mod ai;
mod artwork;
mod components;
mod dungeon;
mod entity_option;
mod entity_set;
mod inventory;
mod menu_option;
mod persistence;
mod player;
mod ranged;
mod run_state;
mod screens;
mod services;
mod spawner;
mod systems;
mod types;
mod ui_components;
mod user_actions;
mod utils;
use components::{
    AreaOfEffect, BlocksTile, Blood, CausesFire, CombatStats, Confused, Confusion, Consumable,
    Contained, Container, DungeonLevel, EntityMoved, EntryTrigger, Flammable, Furniture, Grabbable,
    Grabbing, Hidden, InBackpack, InflictsDamage, Item, Memory, Monster, Name, Objective, OnFire,
    ParticleLifetime, Player, Position, Potion, ProvidesHealing, Ranged, Renderable, Saveable,
    SerializationHelper, SingleActivation, SufferDamage, Trap, Triggered, Viewshed,
    WantsToDisarmTrap, WantsToDropItem, WantsToGrab, WantsToMelee, WantsToMove, WantsToOpenDoor,
    WantsToPickUpItem, WantsToReleaseGrabbed, WantsToSearchHidden, WantsToTrap, WantsToUse,
};

use dungeon::{dungeon::Dungeon, level_builders, level_utils, tile_type::TileType};
use menu_option::{MenuOption, MenuOptionState};
use player::player_action;
use run_state::RunState;
use screens::{
    ScreenCredits, ScreenDeath, ScreenFailure, ScreenIntro, ScreenMainMenu, ScreenMapGeneric,
    ScreenMapMenu, ScreenMapTargeting, ScreenSuccess, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use services::{
    BloodSpawner, DebrisSpawner, GameLog, ItemSpawner, ParticleEffectSpawner, TrapSpawner,
};
use systems::{
    BloodSpawnSystem, DamageSystem, DebrisSpawnSystem, DisarmTrapSystem, FireBurnSystem,
    FireDieSystem, FireSpreadSystem, GrabSystem, ItemCollectionSystem, ItemDropSystem,
    ItemSpawnSystem, MapIndexingSystem, MeleeCombatSystem, MonsterAI, MoveSystem, OpenDoorSystem,
    ParticleSpawnSystem, ReleaseSystem, RemoveParticleEffectsSystem, RemoveTriggeredTrapsSystem,
    RevealTrapsSystem, SearchForHiddenSystem, SetTrapSystem, TrapSpawnSystem, TriggerSystem,
    UpdateMemoriesSystem, UpdateParticleEffectsSystem, UseItemSystem, VisibilitySystem,
};
use user_actions::{
    map_input_to_horizontal_menu_action, map_input_to_map_action, map_input_to_menu_action,
    map_input_to_static_action, map_input_to_targeting_action, MapAction, MenuAction, StaticAction,
    TargetingAction,
};

fn player_can_leave_dungeon(world: &mut World) -> bool {
    let player_level = utils::get_current_level_from_world(world);
    let dungeon = world.fetch::<Dungeon>();
    let level = dungeon.get_level(player_level).unwrap();
    if let Some(exit_point) = level.exit {
        let player_point = world.fetch::<Point>();
        return player_point.x == exit_point.x && player_point.y == exit_point.y;
    }
    false
}

fn has_objective_in_backpack(world: &World) -> bool {
    let player_ent = world.fetch::<Entity>();
    let backpacks = world.read_storage::<InBackpack>();
    let objectives = world.read_storage::<Objective>();
    for (_objective, backpack) in (&objectives, &backpacks).join() {
        if backpack.owner == *player_ent {
            return true;
        }
    }
    false
}

fn get_entity_at_point_on_level(
    world: &World,
    point: &Point,
    filter: impl Fn(Entity) -> bool,
) -> Option<Entity> {
    let player_level = utils::get_current_level_from_world(world);
    let dungeon = world.fetch::<Dungeon>();
    let level = dungeon.get_level(player_level).unwrap();
    let entities = level_utils::entities_at_xy(level, point.x, point.y);
    entities
        .iter()
        .filter(|e| filter(**e))
        .map(|e| e.to_owned())
        .next()
}

fn get_entity_with_component_at_point<T: Component>(
    world: &mut World,
    point: &Point,
) -> Option<Entity> {
    let storage = world.read_storage::<T>();
    let filter = |e| match storage.get(e) {
        Some(_) => true,
        _ => false,
    };
    get_entity_at_point_on_level(world, point, filter)
}

#[cfg(debug_assertions)]
fn kill_all_monsters(world: &mut World) {
    let monster_ents: Vec<Entity> = {
        let entities = world.entities();
        let monsters = world.read_storage::<Monster>();
        (&entities, &monsters).join().map(|(e, _)| e).collect()
    };
    world
        .delete_entities(&monster_ents)
        .expect("couldn't delete ents");
}

#[cfg(debug_assertions)]
fn reveal_map(world: &mut World) {
    use dungeon::constants::MAP_COUNT;
    let player_level = utils::get_current_level_from_world(world);
    let mut dungeon = world.fetch_mut::<Dungeon>();
    let mut level = dungeon.get_level_mut(player_level).unwrap();
    level.revealed_tiles = vec![true; MAP_COUNT]
}

fn generate_dungeon(world: &mut World, levels: u8) -> Dungeon {
    let levels = (0..levels).fold(HashMap::new(), |mut acc, floor_number| {
        let mut level = level_builders::build(floor_number);
        {
            let mut rng = world.fetch_mut::<RandomNumberGenerator>();
            level_builders::update_level_from_room_features(&mut level, &mut rng);
            if floor_number != levels - 1 {
                level_builders::add_up_stairs(&mut level, &mut rng);
            } else {
                level_builders::add_exit(&mut level, &mut rng);
            }
            if floor_number != 0 {
                level_builders::add_down_stairs(&mut level, &mut rng);
            }
            level_builders::update_room_stamps_from_level(&mut level);
            level_builders::decorate_level(&mut level, &mut rng);
            level_builders::update_level_from_room_stamps(&mut level);
            // refactor the above, it should really just be "decorate level, update level from room stamps"
            // basically this would involve moving the column generation into decorate level
        }
        spawner::spawn_entities_for_level(world, &mut level);
        acc.insert(floor_number, level);
        return acc;
    });
    Dungeon { levels }
}

fn initialize_new_game(world: &mut World) {
    world.write_storage::<Position>().clear();
    world.write_storage::<Renderable>().clear();
    world.write_storage::<Player>().clear();
    world.write_storage::<Viewshed>().clear();
    world.write_storage::<Monster>().clear();
    world.write_storage::<Name>().clear();
    world.write_storage::<BlocksTile>().clear();
    world.write_storage::<WantsToMelee>().clear();
    world.write_storage::<SufferDamage>().clear();
    world.write_storage::<CombatStats>().clear();
    world.write_storage::<Item>().clear();
    world.write_storage::<Potion>().clear();
    world.write_storage::<InBackpack>().clear();
    world.write_storage::<WantsToPickUpItem>().clear();
    world.write_storage::<WantsToUse>().clear();
    world.write_storage::<WantsToDropItem>().clear();
    world.write_storage::<ProvidesHealing>().clear();
    world.write_storage::<Consumable>().clear();
    world.write_storage::<Ranged>().clear();
    world.write_storage::<InflictsDamage>().clear();
    world.write_storage::<AreaOfEffect>().clear();
    world.write_storage::<Confusion>().clear();
    world.write_storage::<Confused>().clear();
    world.write_storage::<SimpleMarker<Saveable>>().clear();
    world.write_storage::<SerializationHelper>().clear();
    world.write_storage::<DungeonLevel>().clear();
    world.write_storage::<Blood>().clear();
    world.write_storage::<ParticleLifetime>().clear();
    world.write_storage::<Hidden>().clear();
    world.write_storage::<EntryTrigger>().clear();
    world.write_storage::<EntityMoved>().clear();
    world.write_storage::<SingleActivation>().clear();
    world.write_storage::<Triggered>().clear();
    world.write_storage::<Objective>().clear();
    world.write_storage::<Contained>().clear();
    world.write_storage::<Container>().clear();
    world.write_storage::<Flammable>().clear();
    world.write_storage::<OnFire>().clear();
    world.write_storage::<CausesFire>().clear();
    world.write_storage::<WantsToSearchHidden>().clear();
    world.write_storage::<Trap>().clear();
    world.write_storage::<WantsToTrap>().clear();
    world.write_storage::<WantsToDisarmTrap>().clear();
    world.write_storage::<WantsToGrab>().clear();
    world.write_storage::<Grabbable>().clear();
    world.write_storage::<Grabbing>().clear();
    world.write_storage::<WantsToMove>().clear();
    world.write_storage::<WantsToReleaseGrabbed>();
    world.write_storage::<Memory>().clear();
    world.write_storage::<WantsToOpenDoor>().clear();
    world.write_storage::<Furniture>().clear();
    world.remove::<SimpleMarkerAllocator<Saveable>>();
    world.insert(SimpleMarkerAllocator::<Saveable>::new());
    let dungeon = generate_dungeon(world, 10);
    let level = dungeon.get_level(9).unwrap();
    let (player_idx, _) = level
        .tiles
        .iter()
        .enumerate()
        .find(|(_, tile_type)| **tile_type == TileType::Exit)
        .unwrap();
    let (player_x, player_y) = level_utils::idx_xy(level, player_idx as i32);
    world.remove::<Point>();
    world.insert(Point::new(player_x, player_y));
    world.remove::<Entity>();
    let player_entity = spawner::spawn_player(world, player_x as i32, player_y as i32, 9);
    world.insert(player_entity);
    let rng = world.get_mut::<RandomNumberGenerator>().unwrap();
    let objective_floor = utils::get_random_between_numbers(rng, 1, 9) as u8;
    let level = dungeon.get_level(objective_floor).unwrap();
    let room_idx = utils::get_random_between_numbers(rng, 0, (level.rooms.len() - 1) as i32);
    let room = level.rooms.get(room_idx as usize).unwrap();
    spawner::spawn_objective_for_room(world, &room.rect, &level);
    world.remove::<Dungeon>();
    world.insert(dungeon);
    world.remove::<GameLog>();
    world.insert(GameLog {
        entries: vec!["Enter the dungeon apprentice! Bring back the Talisman!".to_owned()],
    });
}

fn select_next_menu_page(page: usize, total_pages: usize) -> usize {
    if page < total_pages {
        return page + 1;
    }
    page
}

fn select_prev_menu_page(page: usize) -> usize {
    if page > 0 {
        return page - 1;
    }
    page
}

pub struct State {
    world: World,
    run_state: RunState,
}

impl State {
    fn run_systems(&mut self, ctx: &mut Rltk) {
        let mut update_particles = UpdateParticleEffectsSystem {
            elapsed_time: ctx.frame_time_ms,
        };
        update_particles.run_now(&self.world);
        let mut remove_particles = RemoveParticleEffectsSystem {};
        remove_particles.run_now(&self.world);
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.world);
        let mut update_memories_system = UpdateMemoriesSystem {};
        update_memories_system.run_now(&self.world);
        if self.run_state == RunState::MonsterTurn {
            let mut mob = MonsterAI {};
            mob.run_now(&self.world);
        }
        let mut move_system = MoveSystem {};
        move_system.run_now(&self.world);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.world);
        let mut melee_combat = MeleeCombatSystem {};
        melee_combat.run_now(&self.world);
        let mut triggers = TriggerSystem {};
        triggers.run_now(&self.world);
        if self.run_state == RunState::MonsterTurn {
            let mut fire_burn_system = FireBurnSystem {};
            fire_burn_system.run_now(&self.world);
            let mut fire_spread_system = FireSpreadSystem {};
            fire_spread_system.run_now(&self.world);
            let mut fire_die_system = FireDieSystem {};
            fire_die_system.run_now(&self.world);
        }
        let mut damage = DamageSystem {};
        damage.run_now(&self.world);
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.world);
        let mut to_use = UseItemSystem {};
        to_use.run_now(&self.world);
        let mut drop = ItemDropSystem {};
        drop.run_now(&self.world);
        let mut remove_triggered_single_activation_traps_system = RemoveTriggeredTrapsSystem {};
        remove_triggered_single_activation_traps_system.run_now(&self.world);
        if self.run_state == RunState::PlayerTurn {
            let mut reveal_traps = RevealTrapsSystem {};
            reveal_traps.run_now(&self.world);
        }
        let mut release_system = ReleaseSystem {};
        release_system.run_now(&self.world);
        if self.run_state == RunState::PlayerTurn || self.run_state == RunState::MonsterTurn {
            let mut search_for_hidden_system = SearchForHiddenSystem {};
            search_for_hidden_system.run_now(&self.world);
            let mut set_trap_system = SetTrapSystem {};
            set_trap_system.run_now(&self.world);
            let mut disarm_trap_system = DisarmTrapSystem {};
            disarm_trap_system.run_now(&self.world);
            let mut grab_system = GrabSystem {};
            grab_system.run_now(&self.world);
            let mut open_door_system = OpenDoorSystem {};
            open_door_system.run_now(&self.world);
        }
        let mut blood_spawn_system = BloodSpawnSystem {};
        blood_spawn_system.run_now(&self.world);
        let mut particle_spawn_system = ParticleSpawnSystem {};
        particle_spawn_system.run_now(&self.world);
        let mut trap_spawn_system = TrapSpawnSystem {};
        trap_spawn_system.run_now(&self.world);
        let mut item_spawn_system = ItemSpawnSystem {};
        item_spawn_system.run_now(&self.world);
        let mut debris_spawn_system = DebrisSpawnSystem {};
        debris_spawn_system.run_now(&self.world);
        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.run_state = match self.run_state {
            RunState::PreRun => {
                ScreenMapGeneric::new().draw(ctx, &mut self.world);
                self.run_systems(ctx);
                DamageSystem::delete_the_dead(&mut self.world);
                RunState::AwaitingInput
            }
            RunState::AwaitingInput => {
                ScreenMapGeneric::new().draw(ctx, &mut self.world);
                self.run_systems(ctx);
                let action = map_input_to_map_action(ctx);
                match action {
                    MapAction::Exit => {
                        persistence::save_game(&mut self.world);
                        RunState::MainMenu { highlighted: 0 }
                    }
                    MapAction::NoAction => RunState::AwaitingInput,
                    MapAction::ShowInventoryMenu => RunState::InventoryMenu {
                        highlighted: 0,
                        page: 0,
                    },
                    MapAction::ShowDropMenu => RunState::DropItemMenu {
                        highlighted: 0,
                        page: 0,
                    },
                    MapAction::LeaveDungeon => match player_can_leave_dungeon(&mut self.world) {
                        true => RunState::ExitGameMenu { highlighted: 0 },
                        false => {
                            let mut log = self.world.fetch_mut::<GameLog>();
                            log.add(
                                "You must first locate the exit to leave the dungeon".to_string(),
                            );
                            RunState::AwaitingInput
                        }
                    },
                    MapAction::ShowActionMenu => RunState::ActionMenu { highlighted: 0 },
                    #[cfg(debug_assertions)]
                    MapAction::ShowDebugMenu => RunState::DebugMenu { highlighted: 0 },
                    MapAction::SearchContainer => RunState::ShowTargetingOpenContainer,
                    MapAction::GrabFurniture => RunState::ShowTargetingGrabFurniture,
                    MapAction::DisarmTrap => RunState::ShowTargetingDisarmTrap,
                    _ => {
                        player_action(&mut self.world, action);
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::PlayerTurn => {
                ScreenMapGeneric::new().draw(ctx, &mut self.world);
                self.run_systems(ctx);
                DamageSystem::delete_the_dead(&mut self.world);
                RunState::MonsterTurn
            }
            RunState::MonsterTurn => {
                ScreenMapGeneric::new().draw(ctx, &mut self.world);
                self.run_systems(ctx);
                DamageSystem::delete_the_dead(&mut self.world);
                let combat_stats = self.world.read_storage::<CombatStats>();
                let player_ent = self.world.fetch::<Entity>();
                let player_stats = combat_stats.get(*player_ent).unwrap();
                match player_stats.hp < 1 {
                    true => {
                        persistence::delete_save();
                        RunState::DeathScreen
                    }
                    _ => RunState::AwaitingInput,
                }
            }
            RunState::InventoryMenu { highlighted, page } => {
                let inventory = inventory::get_player_inventory_list(&mut self.world);
                let item_count = inventory.len();
                let items_per_page: usize = 10;
                let total_pages = item_count / items_per_page;
                let (inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                let menu: Vec<MenuOption> = inventory_names
                    .iter()
                    .skip(items_per_page * page as usize)
                    .take(items_per_page)
                    .enumerate()
                    .map(|(index, text)| {
                        let state = match highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();
                ScreenMapMenu::new(
                    &menu,
                    &format!("Use Item  < {}/{} >", page + 1, total_pages + 1),
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                let action = map_input_to_menu_action(ctx, highlighted);
                match action {
                    MenuAction::NoAction => RunState::InventoryMenu { highlighted, page },
                    MenuAction::Exit => RunState::AwaitingInput,
                    MenuAction::MoveHighlightNext => RunState::InventoryMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                        page,
                    },
                    MenuAction::MoveHighlightPrev => RunState::InventoryMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                        page,
                    },
                    MenuAction::NextPage => RunState::InventoryMenu {
                        highlighted,
                        page: select_next_menu_page(page, total_pages),
                    },
                    MenuAction::PreviousPage => RunState::InventoryMenu {
                        highlighted,
                        page: select_prev_menu_page(page),
                    },
                    MenuAction::Select { option } => {
                        match inventory_entities.get(page * items_per_page + option) {
                            Some(ent) => {
                                let is_ranged = {
                                    let ranged = self.world.read_storage::<Ranged>();
                                    match ranged.get(*ent) {
                                        Some(ranged_props) => Some(ranged_props.range),
                                        _ => None,
                                    }
                                };
                                match is_ranged {
                                    Some(range) => RunState::ShowTargeting { range, item: *ent },
                                    None => {
                                        player::use_item(&mut self.world, *ent, None);
                                        RunState::PlayerTurn
                                    }
                                }
                            }
                            None => RunState::InventoryMenu { highlighted, page },
                        }
                    }
                }
            }
            RunState::DropItemMenu { highlighted, page } => {
                let inventory = inventory::get_player_inventory_list(&mut self.world);
                let item_count = inventory.len();
                let items_per_page: usize = 10;
                let total_pages = item_count / items_per_page;
                let (inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                let menu: Vec<MenuOption> = inventory_names
                    .iter()
                    .skip(items_per_page * page as usize)
                    .take(items_per_page)
                    .enumerate()
                    .map(|(index, text)| {
                        let state = match highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();
                ScreenMapMenu::new(
                    &menu,
                    &format!("Drop Item  < {}/{} >", page + 1, total_pages + 1),
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                let action = map_input_to_menu_action(ctx, highlighted);
                match action {
                    MenuAction::NoAction => RunState::DropItemMenu { highlighted, page },
                    MenuAction::Exit => RunState::AwaitingInput,
                    MenuAction::MoveHighlightNext => RunState::DropItemMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                        page,
                    },
                    MenuAction::MoveHighlightPrev => RunState::DropItemMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                        page,
                    },
                    MenuAction::NextPage => RunState::DropItemMenu {
                        highlighted,
                        page: select_next_menu_page(page, total_pages),
                    },
                    MenuAction::PreviousPage => RunState::DropItemMenu {
                        highlighted,
                        page: select_prev_menu_page(page),
                    },
                    MenuAction::Select { option } => {
                        match inventory_entities.get(page * items_per_page + option) {
                            Some(ent) => {
                                let mut intent = self.world.write_storage::<WantsToDropItem>();
                                intent
                                    .insert(
                                        *self.world.fetch::<Entity>(),
                                        WantsToDropItem { item: *ent },
                                    )
                                    .expect("Unable To Insert Drop Item Intent");
                                RunState::PlayerTurn
                            }
                            None => RunState::DropItemMenu { highlighted, page },
                        }
                    }
                }
            }
            RunState::ExitGameMenu { highlighted } => {
                let menu: Vec<MenuOption> = ["Yes, exit the dungeon", "No, remain in the dungeon"]
                    .iter()
                    .enumerate()
                    .map(|(index, text)| {
                        let state = match highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();
                ScreenMapMenu::new(&menu, "Exit Dungeon?", "Escape to Cancel")
                    .draw(ctx, &mut self.world);
                let action = map_input_to_menu_action(ctx, highlighted);
                match action {
                    MenuAction::Exit => RunState::AwaitingInput,
                    MenuAction::MoveHighlightNext => RunState::ExitGameMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::ExitGameMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                    },
                    MenuAction::Select { option } => match option {
                        0 => {
                            persistence::delete_save();
                            match has_objective_in_backpack(&self.world) {
                                true => RunState::SuccessScreen,
                                false => RunState::FailureScreen,
                            }
                        }
                        _ => RunState::AwaitingInput,
                    },
                    _ => RunState::ExitGameMenu { highlighted },
                }
            }
            RunState::ShowTargeting { range, item } => {
                let visible_tiles = ranged::get_visible_tiles_in_range(&self.world, range);
                let target = ranged::get_target(&self.world, ctx, &visible_tiles);
                ScreenMapTargeting::new(range, target, Some("Select Target".to_string()))
                    .draw(ctx, &mut self.world);
                let action = map_input_to_targeting_action(ctx, target);
                match action {
                    TargetingAction::NoAction => RunState::ShowTargeting { range, item },
                    TargetingAction::Exit => RunState::AwaitingInput,
                    TargetingAction::Selected(target) => {
                        player::use_item(&mut self.world, item, Some(target));
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::ShowTargetingOpenContainer => {
                let visible_tiles = ranged::get_visible_tiles_in_range(&self.world, 1);
                let target = ranged::get_target(&self.world, ctx, &visible_tiles);
                ScreenMapTargeting::new(1, target, Some("Select Container to Open".to_string()))
                    .draw(ctx, &mut self.world);
                let action = map_input_to_targeting_action(ctx, target);
                match action {
                    TargetingAction::NoAction => RunState::ShowTargetingOpenContainer,
                    TargetingAction::Exit => RunState::AwaitingInput,
                    TargetingAction::Selected(target) => {
                        let container_entity = get_entity_with_component_at_point::<Container>(
                            &mut self.world,
                            &target,
                        );
                        match container_entity {
                            Some(e) => RunState::OpenContainerMenu {
                                page: 0,
                                highlighted: 0,
                                container: e,
                            },
                            None => {
                                let mut game_log = self.world.fetch_mut::<GameLog>();
                                game_log.add(
                                    "There's no container to open at this location.".to_string(),
                                );
                                RunState::PlayerTurn
                            }
                        }
                    }
                }
            }
            RunState::ShowTargetingDisarmTrap => {
                let visible_tiles = ranged::get_visible_tiles_in_range(&self.world, 1);
                let target = ranged::get_target(&self.world, ctx, &visible_tiles);
                ScreenMapTargeting::new(1, target, Some("Select Trap to Disarm".to_string()))
                    .draw(ctx, &mut self.world);
                let action = map_input_to_targeting_action(ctx, target);
                match action {
                    TargetingAction::NoAction => RunState::ShowTargetingDisarmTrap,
                    TargetingAction::Exit => RunState::AwaitingInput,
                    TargetingAction::Selected(target) => {
                        let trap_entity =
                            get_entity_with_component_at_point::<Trap>(&mut self.world, &target);
                        match trap_entity {
                            Some(e) => player::disarm_trap(&mut self.world, e),
                            None => {
                                let mut game_log = self.world.fetch_mut::<GameLog>();
                                game_log
                                    .add("There are no armed traps at this location.".to_string());
                            }
                        }
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::ShowTargetingAttack => {
                let visible_tiles = ranged::get_visible_tiles_in_range(&self.world, 1);
                let target = ranged::get_target(&self.world, ctx, &visible_tiles);
                ScreenMapTargeting::new(1, target, Some("Select thing to Attack".to_string()))
                    .draw(ctx, &mut self.world);
                let action = map_input_to_targeting_action(ctx, target);
                match action {
                    TargetingAction::NoAction => RunState::ShowTargetingAttack,
                    TargetingAction::Exit => RunState::AwaitingInput,
                    TargetingAction::Selected(target) => {
                        let attack_entity = get_entity_with_component_at_point::<CombatStats>(
                            &mut self.world,
                            &target,
                        );
                        match attack_entity {
                            Some(e) => player::attack_entity(&mut self.world, e),
                            None => {
                                let mut game_log = self.world.fetch_mut::<GameLog>();
                                game_log
                                    .add("There is nothing to attack at this location".to_string());
                            }
                        }
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::ShowTargetingGrabFurniture => {
                let visible_tiles = ranged::get_visible_tiles_in_range(&self.world, 1);
                let target = ranged::get_target(&self.world, ctx, &visible_tiles);
                ScreenMapTargeting::new(1, target, Some("Select Furniture to Grab".to_string()))
                    .draw(ctx, &mut self.world);
                let action = map_input_to_targeting_action(ctx, target);
                match action {
                    TargetingAction::NoAction => RunState::ShowTargetingGrabFurniture,
                    TargetingAction::Exit => RunState::AwaitingInput,
                    TargetingAction::Selected(target) => {
                        let grabbable_entity = get_entity_with_component_at_point::<Grabbable>(
                            &mut self.world,
                            &target,
                        );
                        match grabbable_entity {
                            Some(e) => player::grab_entity(&mut self.world, e),
                            None => {
                                let mut game_log = self.world.fetch_mut::<GameLog>();
                                game_log.add(
                                    "There are no grabbable pieces of furniture at this location."
                                        .to_string(),
                                );
                            }
                        }
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::OpenContainerMenu {
                page,
                highlighted,
                container,
            } => {
                let inventory =
                    inventory::get_container_inventory_list(&mut self.world, &container);
                let item_count = inventory.len();
                let items_per_page: usize = 10;
                let total_pages = item_count / items_per_page;
                let (inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                let menu: Vec<MenuOption> = inventory_names
                    .iter()
                    .skip(items_per_page * page as usize)
                    .take(items_per_page)
                    .enumerate()
                    .map(|(index, text)| {
                        let state = match highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();
                ScreenMapMenu::new(
                    &menu,
                    &format!("Take Item  < {}/{} >", page + 1, total_pages + 1),
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                let action = map_input_to_menu_action(ctx, highlighted);
                match action {
                    MenuAction::NoAction => RunState::OpenContainerMenu {
                        highlighted,
                        page,
                        container,
                    },
                    MenuAction::Exit => RunState::AwaitingInput,
                    MenuAction::MoveHighlightNext => RunState::OpenContainerMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                        page,
                        container,
                    },
                    MenuAction::MoveHighlightPrev => RunState::OpenContainerMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                        page,
                        container,
                    },
                    MenuAction::NextPage => RunState::OpenContainerMenu {
                        highlighted,
                        page: select_next_menu_page(page, total_pages),
                        container,
                    },
                    MenuAction::PreviousPage => RunState::OpenContainerMenu {
                        highlighted,
                        page: select_prev_menu_page(page),
                        container,
                    },
                    MenuAction::Select { option } => {
                        match inventory_entities.get(page * items_per_page + option) {
                            Some(ent) => {
                                let mut intent = self.world.write_storage::<WantsToPickUpItem>();
                                let collected_by = self.world.fetch::<Entity>();
                                intent
                                    .insert(
                                        *self.world.fetch::<Entity>(),
                                        WantsToPickUpItem {
                                            item: *ent,
                                            collected_by: *collected_by,
                                        },
                                    )
                                    .expect("Unable To Insert Pick Up Item Intent");
                                RunState::PlayerTurn
                            }
                            None => RunState::OpenContainerMenu {
                                page,
                                highlighted,
                                container,
                            },
                        }
                    }
                }
            }
            RunState::ActionMenu { highlighted } => {
                let menu: Vec<MenuOption> = vec![
                    "Use Item",
                    "Drop Item",
                    "Open Container",
                    "Search Area",
                    "Disarm Trap",
                    "Grab Furniture",
                    "Release Furniture",
                    "Attack",
                ]
                .iter()
                .enumerate()
                .map(|(index, text)| {
                    let state = match highlighted == index {
                        true => MenuOptionState::Highlighted,
                        false => MenuOptionState::Normal,
                    };
                    MenuOption::new(text, state)
                })
                .collect();

                ScreenMapMenu::new(&menu, "Choose an action", "Escape to Cancel")
                    .draw(ctx, &mut self.world);
                let action = map_input_to_menu_action(ctx, highlighted);
                match action {
                    MenuAction::MoveHighlightNext => RunState::ActionMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::ActionMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                    },
                    MenuAction::NoAction => RunState::ActionMenu { highlighted },
                    MenuAction::Exit => RunState::AwaitingInput,
                    MenuAction::Select { option } => match option {
                        0 => RunState::InventoryMenu {
                            highlighted: 0,
                            page: 0,
                        },
                        1 => RunState::DropItemMenu {
                            highlighted: 0,
                            page: 0,
                        },
                        2 => RunState::ShowTargetingOpenContainer,
                        3 => {
                            player_action(&mut self.world, MapAction::SearchHidden);
                            RunState::PlayerTurn
                        }
                        4 => RunState::ShowTargetingDisarmTrap,
                        5 => RunState::ShowTargetingGrabFurniture,
                        6 => {
                            player::release_entity(&mut self.world);
                            RunState::AwaitingInput
                        }
                        7 => RunState::ShowTargetingAttack,
                        _ => RunState::ActionMenu { highlighted },
                    },
                    _ => RunState::ActionMenu { highlighted },
                }
            }
            RunState::IntroScreen => {
                ScreenIntro::new().draw(ctx);
                let action = map_input_to_static_action(ctx);
                match action {
                    StaticAction::Exit => RunState::MainMenu { highlighted: 0 },
                    StaticAction::Continue => RunState::PreRun,
                    StaticAction::NoAction => RunState::IntroScreen,
                }
            }
            RunState::DeathScreen => {
                ScreenDeath::new().draw(ctx);
                let action = map_input_to_static_action(ctx);
                match action {
                    StaticAction::NoAction => RunState::DeathScreen,
                    _ => RunState::MainMenu { highlighted: 0 },
                }
            }
            RunState::SuccessScreen => {
                ScreenSuccess::new().draw(ctx);
                let action = map_input_to_static_action(ctx);
                match action {
                    StaticAction::NoAction => RunState::SuccessScreen,
                    _ => RunState::CreditsScreen,
                }
            }
            RunState::FailureScreen => {
                ScreenFailure::new().draw(ctx);
                let action = map_input_to_static_action(ctx);
                match action {
                    StaticAction::NoAction => RunState::FailureScreen,
                    _ => RunState::MainMenu { highlighted: 0 },
                }
            }
            RunState::CreditsScreen => {
                ScreenCredits::new().draw(ctx);
                let action = map_input_to_static_action(ctx);
                match action {
                    StaticAction::NoAction => RunState::CreditsScreen,
                    _ => RunState::MainMenu { highlighted: 0 },
                }
            }
            RunState::MainMenu { highlighted } => {
                let has_save_game = persistence::has_save_game();
                let new_game_state = match highlighted == 0 {
                    true => MenuOptionState::Highlighted,
                    false => MenuOptionState::Normal,
                };
                let continue_state = match has_save_game {
                    true => match highlighted == 1 {
                        true => MenuOptionState::Highlighted,
                        false => MenuOptionState::Normal,
                    },
                    false => MenuOptionState::Disabled,
                };
                let credits_state = match highlighted == 2 {
                    true => MenuOptionState::Highlighted,
                    false => MenuOptionState::Normal,
                };
                let menu = if cfg!(target_arch = "wasm32") {
                    vec![
                        MenuOption::new("New Game", new_game_state),
                        MenuOption::new("Continue", continue_state),
                        MenuOption::new("Credits", credits_state),
                    ]
                } else {
                    let quit_state = match highlighted == 3 {
                        true => MenuOptionState::Highlighted,
                        false => MenuOptionState::Normal,
                    };
                    vec![
                        MenuOption::new("New Game", new_game_state),
                        MenuOption::new("Continue", continue_state),
                        MenuOption::new("Credits", credits_state),
                        MenuOption::new("Quit", quit_state),
                    ]
                };

                ScreenMainMenu::new(&menu).draw(ctx);
                let action = map_input_to_horizontal_menu_action(ctx, highlighted);
                match action {
                    MenuAction::Exit => RunState::MainMenu { highlighted },
                    MenuAction::MoveHighlightNext => RunState::MainMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::MainMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                    },
                    MenuAction::Select { option } => match option {
                        0 => {
                            initialize_new_game(&mut self.world);
                            RunState::IntroScreen
                        }
                        1 => {
                            persistence::load_game(&mut self.world);
                            persistence::delete_save();
                            RunState::AwaitingInput
                        }
                        2 => RunState::CreditsScreen,
                        3 => std::process::exit(0),
                        _ => RunState::MainMenu { highlighted },
                    },
                    _ => RunState::MainMenu { highlighted },
                }
            }
            #[cfg(debug_assertions)]
            RunState::DebugMenu { highlighted } => {
                let menu = [
                    MenuOption::new(
                        "Wrath of God",
                        match highlighted == 0 {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        },
                    ),
                    MenuOption::new(
                        "Gitaxian Probe",
                        match highlighted == 1 {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        },
                    ),
                ]
                .to_vec();
                ScreenMapMenu::new(&menu, "Debug Menu", "Escape to Cancel")
                    .draw(ctx, &mut self.world);
                let action = map_input_to_menu_action(ctx, highlighted);
                match action {
                    MenuAction::Exit => RunState::DebugMenu { highlighted },
                    MenuAction::MoveHighlightNext => RunState::DebugMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::DebugMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                    },
                    MenuAction::Select { option } => match option {
                        0 => {
                            kill_all_monsters(&mut self.world);
                            self.world
                                .fetch_mut::<GameLog>()
                                .entries
                                .insert(0, "all monsters removed".to_owned());
                            RunState::AwaitingInput
                        }
                        1 => {
                            reveal_map(&mut self.world);
                            self.world
                                .fetch_mut::<GameLog>()
                                .entries
                                .insert(0, "map revealed".to_owned());
                            RunState::AwaitingInput
                        }
                        _ => RunState::DebugMenu { highlighted },
                    },
                    _ => RunState::DebugMenu { highlighted },
                }
            }
        };
    }
}

#[wasm_bindgen]
pub fn start() {
    let mut gs = State {
        world: World::new(),
        run_state: RunState::MainMenu { highlighted: 0 },
    };
    gs.world.register::<Memory>();
    gs.world.register::<Position>();
    gs.world.register::<Renderable>();
    gs.world.register::<Player>();
    gs.world.register::<Viewshed>();
    gs.world.register::<Monster>();
    gs.world.register::<Name>();
    gs.world.register::<BlocksTile>();
    gs.world.register::<WantsToMelee>();
    gs.world.register::<SufferDamage>();
    gs.world.register::<CombatStats>();
    gs.world.register::<Item>();
    gs.world.register::<Potion>();
    gs.world.register::<InBackpack>();
    gs.world.register::<WantsToPickUpItem>();
    gs.world.register::<WantsToUse>();
    gs.world.register::<WantsToDropItem>();
    gs.world.register::<ProvidesHealing>();
    gs.world.register::<Consumable>();
    gs.world.register::<Ranged>();
    gs.world.register::<InflictsDamage>();
    gs.world.register::<AreaOfEffect>();
    gs.world.register::<Confusion>();
    gs.world.register::<Confused>();
    gs.world.register::<SimpleMarker<Saveable>>();
    gs.world.register::<SerializationHelper>();
    gs.world.register::<DungeonLevel>();
    gs.world.register::<Blood>();
    gs.world.register::<ParticleLifetime>();
    gs.world.register::<Hidden>();
    gs.world.register::<EntryTrigger>();
    gs.world.register::<EntityMoved>();
    gs.world.register::<SingleActivation>();
    gs.world.register::<Triggered>();
    gs.world.register::<Objective>();
    gs.world.register::<Contained>();
    gs.world.register::<Container>();
    gs.world.register::<Flammable>();
    gs.world.register::<OnFire>();
    gs.world.register::<CausesFire>();
    gs.world.register::<WantsToSearchHidden>();
    gs.world.register::<Trap>();
    gs.world.register::<WantsToTrap>();
    gs.world.register::<WantsToDisarmTrap>();
    gs.world.register::<WantsToGrab>();
    gs.world.register::<Grabbable>();
    gs.world.register::<Grabbing>();
    gs.world.register::<WantsToMove>();
    gs.world.register::<WantsToReleaseGrabbed>();
    gs.world.register::<WantsToOpenDoor>();
    gs.world.register::<Furniture>();
    gs.world.insert(SimpleMarkerAllocator::<Saveable>::new());
    gs.world.insert(GameLog {
        entries: vec!["Enter the dungeon apprentice! Bring back the Talisman!".to_owned()],
    }); // This needs to get moved to a continue game function I think...
    let rng = RandomNumberGenerator::new();
    gs.world.insert(rng);
    gs.world.insert(ParticleEffectSpawner::new());
    gs.world.insert(BloodSpawner::new());
    gs.world.insert(DebrisSpawner::new());
    gs.world.insert(TrapSpawner::new());
    gs.world.insert(ItemSpawner::new());
    let context = RltkBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Apprentice")
        .build()
        .expect("failed to create context");
    rltk::main_loop(context, gs).expect("failed to start apprentice");
}
