use menu::Menu;
use rltk::{a_star_search, GameState, RandomNumberGenerator, Rltk, RltkBuilder};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use std::collections::HashMap;
use std::iter;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate specs_derive;
extern crate serde;
mod ai;
mod artwork;
mod components;
mod copy;
#[cfg(debug_assertions)]
mod debug;
mod dungeon;
mod entity_option;
mod entity_set;
mod inventory;
mod menu;
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
    equipable::EquipmentPositions, AreaOfEffect, Armable, BlocksTile, Blood, CausesDamage,
    CausesFire, CausesLight, CombatStats, Confused, Confusion, Consumable, Container,
    DamageHistory, Disarmable, Dousable, EntityMoved, EntryTrigger, Equipable, Equipment,
    Flammable, Furniture, Grabbable, Grabbing, Hidden, Hiding, HidingSpot, Info, Inventory, Item,
    Lightable, Memory, Monster, Name, Objective, OnFire, ParticleLifetime, Player, Position,
    Potion, ProvidesHealing, Ranged, Renderable, Saveable, SerializationHelper, SingleActivation,
    SufferDamage, Trap, Triggered, Viewshed, WantsToDisarmTrap, WantsToDouse, WantsToDropItem,
    WantsToEquip, WantsToGrab, WantsToHide, WantsToLight, WantsToMelee, WantsToMove,
    WantsToOpenDoor, WantsToPickUpItem, WantsToReleaseGrabbed, WantsToSearchHidden, WantsToTrap,
    WantsToUse,
};
use types::EquipMenuType;

use dungeon::{dungeon::Dungeon, level_builders, tile_type::TileType};
use menu_option::{MenuOption, MenuOptionState};
use player::{player_action, InteractionType};
use run_state::RunState;
use screens::{
    ScreenCredits, ScreenDeath, ScreenFailure, ScreenIntro, ScreenMainMenu, ScreenMapGeneric,
    ScreenMapInteractMenu, ScreenMapInteractTarget, ScreenMapItemMenu, ScreenMapMenu,
    ScreenMapTargeting, ScreenSuccess, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use services::{
    BloodSpawner, CorpseSpawner, DebrisSpawner, GameLog, ItemSpawner, ParticleEffectSpawner,
    TrapSpawner,
};
use systems::{
    BloodSpawnSystem, CorpseSpawnSystem, DamageSystem, DebrisSpawnSystem, DisarmTrapSystem,
    DouseItemSystem, EquipSystem, FireBurnSystem, FireDieSystem, FireSpreadSystem, GrabSystem,
    HideSystem, ItemCollectionSystem, ItemDropSystem, ItemSpawnSystem, LightItemSystem,
    LightSystem, MapIndexingSystem, MeleeCombatSystem, MonsterAI, MoveSystem, OpenDoorSystem,
    ParticleSpawnSystem, ReleaseSystem, RemoveParticleEffectsSystem, RemoveTriggeredTrapsSystem,
    RevealTrapsSystem, SearchForHiddenSystem, SetTrapSystem, TrapSpawnSystem, TriggerSystem,
    UpdateMemoriesSystem, UpdateParticleEffectsSystem, UseItemSystem, VisibilitySystem,
};
use user_actions::{
    map_input_to_horizontal_menu_action, map_input_to_interaction_targeting_action,
    map_input_to_map_action, map_input_to_menu_action, map_input_to_static_action,
    map_input_to_targeting_action, InteractionTargetingAction, MapAction, MenuAction, StaticAction,
    TargetingAction,
};

fn player_can_leave_dungeon(world: &mut World) -> bool {
    let player_level = utils::get_current_level_from_world(world);
    let dungeon = world.fetch::<Dungeon>();
    let level = dungeon.get_level(player_level).unwrap();
    match level.exit {
        Some(exit_idx) => {
            let player_entity = world.fetch::<Entity>();
            let player_position = world
                .read_storage::<Position>()
                .get(*player_entity)
                .unwrap()
                .idx;
            player_position == exit_idx
        }
        None => false,
    }
}

fn has_objective_in_backpack(world: &World) -> bool {
    let player_ent = world.fetch::<Entity>();
    let inventories = world.read_storage::<Inventory>();
    let player_inventory = inventories.get(*player_ent).unwrap();
    let entities = world.entities();
    let objectives = world.read_storage::<Objective>();
    for (entity, _objective) in (&entities, &objectives).join() {
        if player_inventory.items.contains(&entity) {
            return true;
        }
    }
    false
}

fn get_visible_entities(world: &World) -> Box<[Entity]> {
    let player_ent = world.fetch::<Entity>();
    let viewsheds = world.read_storage::<Viewshed>();
    let player_viewshed = viewsheds.get(*player_ent).unwrap();
    let positions = world.read_storage::<Position>();
    let player_position = positions.get(*player_ent).unwrap();
    let dungeon = world.fetch::<Dungeon>();
    let level = dungeon.get_level(player_position.level).unwrap();
    let hiddens = world.read_storage::<Hidden>();
    player_viewshed
        .visible_tiles
        .iter()
        .map(|idx| {
            level
                .tile_content
                .get(*idx as usize)
                .unwrap()
                .iter()
                .filter(|e| match hiddens.get(**e) {
                    Some(hidden) => hidden.found_by.contains(&*player_ent),
                    None => true,
                })
        })
        .flatten()
        .map(|e| *e)
        .collect()
}

fn filter_for_component<T: Component>(world: &World, entities: Box<[Entity]>) -> Box<[Entity]> {
    let interactables = world.read_storage::<T>();
    entities
        .iter()
        .filter(|e| interactables.get(**e).is_some())
        .map(|e| *e)
        .collect()
}

fn get_interaction_type_targets<T: Component>(world: &World) -> Box<[Entity]> {
    filter_for_component::<T>(world, get_visible_entities(world))
}

fn get_interaction_targets(world: &World) -> Box<[Entity]> {
    let visible_entities = get_visible_entities(world);
    let grabbables = world.read_storage::<Grabbable>();
    let lightables = world.read_storage::<Lightable>();
    let dousables = world.read_storage::<Dousable>();
    let disarmables = world.read_storage::<Trap>();
    let hideables = world.read_storage::<HidingSpot>();
    let combat_stats = world.read_storage::<CombatStats>();
    let containers = world.read_storage::<Container>();
    let items = world.read_storage::<Item>();
    visible_entities
        .iter()
        .filter(|e| {
            grabbables.get(**e).is_some()
                || lightables.get(**e).is_some()
                || dousables.get(**e).is_some()
                || disarmables.get(**e).is_some()
                || hideables.get(**e).is_some()
                || combat_stats.get(**e).is_some()
                || items.get(**e).is_some()
                || containers.get(**e).is_some()
        })
        .map(|e| *e)
        .collect()
}

fn get_interaction_options_for_target(world: &World, target: Entity) -> Vec<InteractionType> {
    let mut interactions = vec![];
    if world.read_storage::<Disarmable>().get(target).is_some() {
        interactions.push(InteractionType::Disarm);
    }
    if world.read_storage::<Armable>().get(target).is_some() {
        interactions.push(InteractionType::Arm);
    }
    if world.read_storage::<Dousable>().get(target).is_some() {
        interactions.push(InteractionType::Douse);
    }
    if world.read_storage::<Lightable>().get(target).is_some() {
        interactions.push(InteractionType::Light);
    }
    if world.read_storage::<Grabbable>().get(target).is_some() {
        interactions.push(InteractionType::Grab);
    }
    if world.read_storage::<HidingSpot>().get(target).is_some() {
        interactions.push(InteractionType::HideIn);
    }
    if world.read_storage::<CombatStats>().get(target).is_some() {
        interactions.push(InteractionType::Attack);
    }
    if world.read_storage::<Item>().get(target).is_some() {
        interactions.push(InteractionType::Pickup);
    }
    if world.read_storage::<Container>().get(target).is_some() {
        interactions.push(InteractionType::OpenContainer);
    }
    interactions
}

fn get_menu_from_interaction_options(highlighted: usize, options: &Vec<InteractionType>) -> Menu {
    let options = options
        .iter()
        .enumerate()
        .map(|(idx, interaction_type)| {
            let name = match interaction_type {
                InteractionType::Disarm => copy::MENU_OPTION_DISARM,
                InteractionType::Arm => copy::MENU_OPTION_ARM,
                InteractionType::Douse => copy::MENU_OPTION_DOUSE,
                InteractionType::Light => copy::MENU_OPTION_LIGHT,
                InteractionType::Grab => copy::MENU_OPTION_GRAB,
                InteractionType::HideIn => copy::MENU_OPTION_HIDE,
                InteractionType::Attack => copy::MENU_OPTION_ATTACK,
                InteractionType::Pickup => copy::MENU_OPTION_PICKUP,
                InteractionType::OpenContainer => copy::MENU_OPTION_OPEN,
            };
            let state = match idx == highlighted {
                true => MenuOptionState::Highlighted,
                false => MenuOptionState::Normal,
            };
            MenuOption::new(name, state)
        })
        .collect();
    Menu::new(options, 10)
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
    world.write_storage::<WantsToPickUpItem>().clear();
    world.write_storage::<WantsToUse>().clear();
    world.write_storage::<WantsToDropItem>().clear();
    world.write_storage::<ProvidesHealing>().clear();
    world.write_storage::<Consumable>().clear();
    world.write_storage::<Ranged>().clear();
    world.write_storage::<AreaOfEffect>().clear();
    world.write_storage::<Confusion>().clear();
    world.write_storage::<Confused>().clear();
    world.write_storage::<SimpleMarker<Saveable>>().clear();
    world.write_storage::<SerializationHelper>().clear();
    world.write_storage::<Blood>().clear();
    world.write_storage::<ParticleLifetime>().clear();
    world.write_storage::<Hidden>().clear();
    world.write_storage::<EntryTrigger>().clear();
    world.write_storage::<EntityMoved>().clear();
    world.write_storage::<SingleActivation>().clear();
    world.write_storage::<Triggered>().clear();
    world.write_storage::<Objective>().clear();
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
    world.write_storage::<Hiding>().clear();
    world.write_storage::<HidingSpot>().clear();
    world.write_storage::<WantsToHide>().clear();
    world.write_storage::<Equipment>().clear();
    world.write_storage::<Equipable>().clear();
    world.write_storage::<WantsToEquip>().clear();
    world.write_storage::<CausesDamage>().clear();
    world.write_storage::<CausesLight>().clear();
    world.write_storage::<Info>().clear();
    world.write_storage::<Lightable>().clear();
    world.write_storage::<Dousable>().clear();
    world.write_storage::<WantsToLight>().clear();
    world.write_storage::<WantsToDouse>().clear();
    world.write_storage::<Armable>().clear();
    world.write_storage::<Disarmable>().clear();
    world.write_storage::<DamageHistory>().clear();
    world.write_storage::<Inventory>().clear();
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
    world.remove::<Entity>();
    let player_entity = spawner::spawn_player(world, player_idx, level);
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

pub struct State {
    world: World,
    run_state: RunState,
    queued_action: Option<(Entity, InteractionType)>,
}

impl State {
    fn run_systems(&mut self, ctx: &mut Rltk) {
        let mut update_particles = UpdateParticleEffectsSystem {
            elapsed_time: ctx.frame_time_ms,
        };
        update_particles.run_now(&self.world);
        let mut remove_particles = RemoveParticleEffectsSystem {};
        remove_particles.run_now(&self.world);
        if self.run_state == RunState::PreRun
            || self.run_state == RunState::PlayerTurn
            || self.run_state == RunState::MonsterTurn
        {
            let mut equip_system = EquipSystem {};
            equip_system.run_now(&self.world);
            let mut light = LightSystem {};
            light.run_now(&self.world);
            let mut vis = VisibilitySystem {
                queued_action: &mut self.queued_action,
            };
            vis.run_now(&self.world);
            let mut update_memories_system = UpdateMemoriesSystem {};
            update_memories_system.run_now(&self.world);
        }
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
        let mut damage = DamageSystem {
            queued_action: &mut self.queued_action,
        };
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
            let mut reveal_traps = RevealTrapsSystem {
                queued_action: &mut self.queued_action,
            };
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
            let mut hide_system = HideSystem {};
            hide_system.run_now(&self.world);
            let mut light_item_system = LightItemSystem {};
            light_item_system.run_now(&self.world);
            let mut douse_item_system = DouseItemSystem {};
            douse_item_system.run_now(&self.world);
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
        let mut corpse_spawn_system = CorpseSpawnSystem {};
        corpse_spawn_system.run_now(&self.world);
        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        match self.run_state {
            RunState::PreRun
            | RunState::AwaitingInput { .. }
            | RunState::PlayerTurn
            | RunState::MonsterTurn => self.run_systems(ctx),
            _ => (),
        }
        self.run_state = match &mut self.run_state {
            RunState::PreRun => {
                ScreenMapGeneric::new(0, 0).draw(ctx, &mut self.world);
                DamageSystem::delete_the_dead(&mut self.world);
                RunState::AwaitingInput {
                    offset_x: 0,
                    offset_y: 0,
                }
            }
            RunState::AwaitingInput { offset_x, offset_y } => {
                ScreenMapGeneric::new(*offset_x, *offset_y).draw(ctx, &mut self.world);
                let action = map_input_to_map_action(ctx);
                if action != MapAction::NoAction {
                    self.queued_action = None
                }
                match action {
                    #[cfg(debug_assertions)]
                    MapAction::ShowDebugMenu => RunState::DebugMenu { highlighted: 0 },
                    MapAction::Exit => {
                        persistence::save_game(&mut self.world);
                        RunState::MainMenu { highlighted: 0 }
                    }
                    MapAction::NoAction => {
                        let mut next_state = RunState::AwaitingInput {
                            offset_x: *offset_x,
                            offset_y: *offset_y,
                        };
                        if let Some((ent, interaction)) = self.queued_action {
                            let path = {
                                let player_entity = self.world.fetch::<Entity>();
                                let positions = self.world.read_storage::<Position>();
                                let player_position = positions.get(*player_entity).unwrap();
                                let target_position = positions.get(ent).unwrap();
                                let dungeon = self.world.fetch::<Dungeon>();
                                let level = dungeon.get_level(player_position.level).unwrap();
                                // this should be updated to only search tiles that the player has seen.
                                a_star_search(
                                    player_position.idx as i32,
                                    target_position.idx as i32,
                                    level,
                                )
                            };
                            let step_count = path.steps.len();
                            if path.success {
                                if step_count > 2 {
                                    let next_index = path.steps[1];
                                    player::move_to_position(&mut self.world, next_index);
                                    next_state = RunState::PlayerTurn;
                                } else {
                                    self.queued_action = None;
                                    match interaction {
                                        InteractionType::OpenContainer => {
                                            next_state = RunState::OpenContainerMenu {
                                                highlighted: 0,
                                                container: ent,
                                            }
                                        }
                                        _ => {
                                            player::interact(&mut self.world, ent, interaction);
                                            next_state = RunState::PlayerTurn;
                                        }
                                    }
                                }
                            } else {
                                self.queued_action = None
                            }
                        }
                        next_state
                    }
                    MapAction::ShowInventoryMenu => RunState::InventoryMenu { highlighted: 0 },
                    MapAction::ShowDropMenu => RunState::DropItemMenu { highlighted: 0 },
                    MapAction::ShowEquipmentMenu => RunState::EquipmentMenu {
                        highlighted: 0,
                        action_highlighted: 0,
                        action_menu: false,
                    },
                    MapAction::LeaveDungeon => match player_can_leave_dungeon(&mut self.world) {
                        true => RunState::ExitGameMenu { highlighted: 0 },
                        false => {
                            let mut log = self.world.fetch_mut::<GameLog>();
                            log.add(
                                "You must first locate the exit to leave the dungeon".to_string(),
                            );
                            RunState::AwaitingInput {
                                offset_x: *offset_x,
                                offset_y: *offset_y,
                            }
                        }
                    },
                    MapAction::ShowActionMenu => RunState::ActionMenu { highlighted: 0 },

                    MapAction::SearchContainer => RunState::InteractionTypeEntityTargeting {
                        target_idx: 0,
                        targets: get_interaction_type_targets::<Container>(&self.world),
                        interaction_type: InteractionType::OpenContainer,
                        cta: Some(copy::CTA_INTERACT_OPEN_CONTAINER),
                    },
                    MapAction::GrabFurniture => RunState::InteractionTypeEntityTargeting {
                        target_idx: 0,
                        targets: get_interaction_type_targets::<Grabbable>(&self.world),
                        interaction_type: InteractionType::Grab,
                        cta: Some(copy::CTA_INTERACT_GRAB),
                    },
                    MapAction::DisarmTrap => RunState::InteractionTypeEntityTargeting {
                        target_idx: 0,
                        targets: get_interaction_type_targets::<Disarmable>(&self.world),
                        interaction_type: InteractionType::Disarm,
                        cta: Some(copy::CTA_INTERACT_DISARM),
                    },
                    MapAction::ArmTrap => RunState::InteractionTypeEntityTargeting {
                        target_idx: 0,
                        targets: get_interaction_type_targets::<Armable>(&self.world),
                        interaction_type: InteractionType::Arm,
                        cta: Some(copy::CTA_INTERACT_ARM),
                    },
                    MapAction::Hide => RunState::InteractionTypeEntityTargeting {
                        target_idx: 0,
                        targets: get_interaction_type_targets::<HidingSpot>(&self.world),
                        interaction_type: InteractionType::HideIn,
                        cta: Some(copy::CTA_INTERACT_HIDE),
                    },
                    MapAction::PickupItem => RunState::InteractionTypeEntityTargeting {
                        target_idx: 0,
                        targets: get_interaction_type_targets::<Item>(&self.world),
                        interaction_type: InteractionType::Pickup,
                        cta: Some(copy::CTA_INTERACT_PICKUP),
                    },
                    MapAction::Attack => RunState::InteractionTypeEntityTargeting {
                        target_idx: 0,
                        targets: get_interaction_type_targets::<CombatStats>(&self.world),
                        interaction_type: InteractionType::Attack,
                        cta: Some(copy::CTA_INTERACT_ATTACK),
                    },
                    MapAction::ScrollDown => RunState::AwaitingInput {
                        offset_x: *offset_x,
                        offset_y: *offset_y + 1,
                    },
                    MapAction::ScrollUp => RunState::AwaitingInput {
                        offset_x: *offset_x,
                        offset_y: *offset_y - 1,
                    },
                    MapAction::ScrollLeft => RunState::AwaitingInput {
                        offset_x: *offset_x - 1,
                        offset_y: *offset_y,
                    },
                    MapAction::ScrollRight => RunState::AwaitingInput {
                        offset_x: *offset_x + 1,
                        offset_y: *offset_y,
                    },
                    MapAction::Interact => RunState::InteractiveEntityTargeting { target_idx: 0 },
                    _ => {
                        player_action(&mut self.world, action);
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::PlayerTurn => {
                ScreenMapGeneric::new(0, 0).draw(ctx, &mut self.world);
                DamageSystem::delete_the_dead(&mut self.world);
                RunState::MonsterTurn
            }
            RunState::MonsterTurn => {
                ScreenMapGeneric::new(0, 0).draw(ctx, &mut self.world);
                DamageSystem::delete_the_dead(&mut self.world);
                let combat_stats = self.world.read_storage::<CombatStats>();
                let player_ent = self.world.fetch::<Entity>();
                let player_stats = combat_stats.get(*player_ent).unwrap();
                match player_stats.hp < 1 {
                    true => {
                        persistence::delete_save();
                        RunState::DeathScreen
                    }
                    _ => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                }
            }
            RunState::InventoryMenu { highlighted } => {
                let inventory = inventory::get_player_inventory_list(&mut self.world);
                let (inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                let menu_options: Box<[MenuOption]> = inventory_names
                    .iter()
                    .enumerate()
                    .map(|(index, text)| {
                        let state = match *highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();
                let menu = Menu::new(menu_options, 10);
                ScreenMapMenu::new(
                    menu.get_page_at_index(*highlighted),
                    &format!(
                        "Use Item  < {}/{} >",
                        menu.page_number_at_index(*highlighted) + 1,
                        menu.page_count() + 1
                    ),
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    MenuAction::MoveHighlightNext => RunState::InventoryMenu {
                        highlighted: menu.get_next_index(*highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::InventoryMenu {
                        highlighted: menu.get_previous_index(*highlighted),
                    },
                    MenuAction::NextPage => RunState::InventoryMenu {
                        highlighted: menu.get_next_page_index(*highlighted),
                    },
                    MenuAction::PreviousPage => RunState::InventoryMenu {
                        highlighted: menu.get_previous_page_index(*highlighted),
                    },
                    MenuAction::Select => match inventory_entities.get(*highlighted) {
                        Some(ent) => {
                            let is_ranged = {
                                let ranged = self.world.read_storage::<Ranged>();
                                match ranged.get(*ent) {
                                    Some(ranged_props) => Some(ranged_props.range),
                                    _ => None,
                                }
                            };
                            match is_ranged {
                                Some(range) => RunState::ItemUseTargeting { range, item: *ent },
                                None => {
                                    player::use_item(&mut self.world, *ent, None);
                                    RunState::PlayerTurn
                                }
                            }
                        }
                        None => RunState::InventoryMenu {
                            highlighted: *highlighted,
                        },
                    },
                    _ => RunState::InventoryMenu {
                        highlighted: *highlighted,
                    },
                }
            }
            RunState::DropItemMenu { highlighted } => {
                let inventory = inventory::get_player_inventory_list(&mut self.world);
                let (inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                let menu_options: Box<[MenuOption]> = inventory_names
                    .iter()
                    .enumerate()
                    .map(|(index, text)| {
                        let state = match *highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();
                let menu = Menu::new(menu_options, 10);
                ScreenMapMenu::new(
                    menu.get_page_at_index(*highlighted),
                    &format!(
                        "Drop Item  < {}/{} >",
                        menu.page_number_at_index(*highlighted) + 1,
                        menu.page_count() + 1
                    ),
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    MenuAction::MoveHighlightNext => RunState::DropItemMenu {
                        highlighted: menu.get_next_index(*highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::DropItemMenu {
                        highlighted: menu.get_previous_index(*highlighted),
                    },
                    MenuAction::NextPage => RunState::DropItemMenu {
                        highlighted: menu.get_next_page_index(*highlighted),
                    },
                    MenuAction::PreviousPage => RunState::DropItemMenu {
                        highlighted: menu.get_previous_page_index(*highlighted),
                    },
                    MenuAction::Select => match inventory_entities.get(*highlighted) {
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
                        None => RunState::DropItemMenu {
                            highlighted: *highlighted,
                        },
                    },
                    _ => RunState::DropItemMenu {
                        highlighted: *highlighted,
                    },
                }
            }
            RunState::ExitGameMenu { highlighted } => {
                let menu_options: Box<[MenuOption]> =
                    ["Yes, exit the dungeon", "No, remain in the dungeon"]
                        .iter()
                        .enumerate()
                        .map(|(index, text)| {
                            let state = match *highlighted == index {
                                true => MenuOptionState::Highlighted,
                                false => MenuOptionState::Normal,
                            };
                            MenuOption::new(text, state)
                        })
                        .collect();
                let menu = Menu::new(menu_options, 10);
                ScreenMapMenu::new(menu.get_page(0), "Exit Dungeon?", "Escape to Cancel")
                    .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    MenuAction::MoveHighlightNext => RunState::ExitGameMenu {
                        highlighted: menu.get_next_page_index(*highlighted),
                    },

                    MenuAction::MoveHighlightPrev => RunState::ExitGameMenu {
                        highlighted: menu.get_previous_page_index(*highlighted),
                    },
                    MenuAction::Select => match highlighted {
                        0 => {
                            persistence::delete_save();
                            match has_objective_in_backpack(&self.world) {
                                true => RunState::SuccessScreen,
                                false => RunState::FailureScreen,
                            }
                        }
                        _ => RunState::AwaitingInput {
                            offset_x: 0,
                            offset_y: 0,
                        },
                    },
                    _ => RunState::ExitGameMenu {
                        highlighted: *highlighted,
                    },
                }
            }
            RunState::ItemUseTargeting { range, item } => {
                let visible_tiles = ranged::get_visible_tiles_in_range(&self.world, *range);
                let target = ranged::get_target(&self.world, ctx, &visible_tiles);
                ScreenMapTargeting::new(*range, target, Some("Select Target".to_string()))
                    .draw(ctx, &mut self.world);
                let action = map_input_to_targeting_action(ctx, target);
                match action {
                    TargetingAction::NoAction => RunState::ItemUseTargeting {
                        range: *range,
                        item: *item,
                    },
                    TargetingAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    TargetingAction::Selected(target) => {
                        player::use_item(&mut self.world, *item, Some(target as usize));
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::InteractMenu {
                highlighted,
                target,
            } => {
                let options: Vec<InteractionType> =
                    get_interaction_options_for_target(&self.world, *target);
                let menu: Menu = get_menu_from_interaction_options(*highlighted, &options);
                let title = self
                    .world
                    .read_storage::<Name>()
                    .get(*target)
                    .unwrap()
                    .name
                    .clone();
                ScreenMapInteractMenu::new(
                    menu.get_page_at_index(*highlighted),
                    Some(&title),
                    Some(""),
                )
                .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::Exit => RunState::InteractiveEntityTargeting { target_idx: 0 },
                    MenuAction::Select => {
                        if let Some(interaction_type) = options.get(*highlighted) {
                            self.queued_action = Some((*target, interaction_type.clone()));
                        }
                        RunState::AwaitingInput {
                            offset_x: 0,
                            offset_y: 0,
                        }
                    }
                    MenuAction::MoveHighlightNext => RunState::InteractMenu {
                        highlighted: menu.get_next_index(*highlighted),
                        target: *target,
                    },
                    MenuAction::MoveHighlightPrev => RunState::InteractMenu {
                        highlighted: menu.get_previous_index(*highlighted),
                        target: *target,
                    },
                    MenuAction::NextPage => RunState::InteractMenu {
                        highlighted: menu.get_next_page_index(*highlighted),
                        target: *target,
                    },
                    MenuAction::PreviousPage => RunState::InteractMenu {
                        highlighted: menu.get_previous_page_index(*highlighted),
                        target: *target,
                    },
                    MenuAction::NoAction | _ => RunState::InteractMenu {
                        highlighted: *highlighted,
                        target: *target,
                    },
                }
            }
            RunState::InteractiveEntityTargeting { target_idx } => {
                let targets = get_interaction_targets(&self.world);
                let target_ent = targets.get(*target_idx);
                ScreenMapInteractTarget::new(target_ent, Some(copy::CTA_INTERACT))
                    .draw(ctx, &mut self.world);
                match map_input_to_interaction_targeting_action(ctx) {
                    InteractionTargetingAction::Selected => match target_ent {
                        Some(target_ent) => RunState::InteractMenu {
                            highlighted: 0,
                            target: *target_ent,
                        },
                        None => RunState::AwaitingInput {
                            offset_x: 0,
                            offset_y: 0,
                        },
                    },
                    InteractionTargetingAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    InteractionTargetingAction::Next => RunState::InteractiveEntityTargeting {
                        target_idx: utils::select_next_idx(*target_idx, targets.len()),
                    },
                    InteractionTargetingAction::Previous => RunState::InteractiveEntityTargeting {
                        target_idx: utils::select_previous_idx(*target_idx, targets.len()),
                    },
                    InteractionTargetingAction::NoAction => RunState::InteractiveEntityTargeting {
                        target_idx: *target_idx,
                    },
                }
            }
            RunState::InteractionTypeEntityTargeting {
                target_idx,
                targets,
                interaction_type,
                cta,
            } => {
                let target_ent = targets.get(*target_idx);
                ScreenMapInteractTarget::new(target_ent, *cta).draw(ctx, &mut self.world);
                match map_input_to_interaction_targeting_action(ctx) {
                    InteractionTargetingAction::Selected => {
                        if let Some(target_ent) = target_ent {
                            self.queued_action = Some((*target_ent, *interaction_type));
                        }
                        RunState::AwaitingInput {
                            offset_x: 0,
                            offset_y: 0,
                        }
                    }
                    InteractionTargetingAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    InteractionTargetingAction::Next => RunState::InteractionTypeEntityTargeting {
                        target_idx: utils::select_next_idx(*target_idx, targets.len()),
                        targets: targets.clone(),
                        interaction_type: *interaction_type,
                        cta: *cta,
                    },
                    InteractionTargetingAction::Previous => {
                        RunState::InteractionTypeEntityTargeting {
                            target_idx: utils::select_previous_idx(*target_idx, targets.len()),
                            targets: targets.clone(),
                            interaction_type: *interaction_type,
                            cta: *cta,
                        }
                    }
                    InteractionTargetingAction::NoAction => {
                        RunState::InteractionTypeEntityTargeting {
                            target_idx: *target_idx,
                            targets: targets.clone(),
                            interaction_type: *interaction_type,
                            cta: *cta,
                        }
                    }
                }
            }
            RunState::OpenContainerMenu {
                highlighted,
                container,
            } => {
                let inventory =
                    inventory::get_container_inventory_list(&mut self.world, &container);
                let (inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                let menu_options: Box<[MenuOption]> = inventory_names
                    .iter()
                    .enumerate()
                    .map(|(index, text)| {
                        let state = match *highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();

                let menu = Menu::new(menu_options, 10);
                ScreenMapMenu::new(
                    menu.get_page_at_index(*highlighted),
                    &format!(
                        "Take Item  < {}/{} >",
                        menu.page_number_at_index(*highlighted),
                        menu.page_count() + 1
                    ),
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    MenuAction::MoveHighlightNext => RunState::OpenContainerMenu {
                        highlighted: menu.get_next_index(*highlighted),
                        container: *container,
                    },
                    MenuAction::MoveHighlightPrev => RunState::OpenContainerMenu {
                        highlighted: menu.get_previous_index(*highlighted),
                        container: *container,
                    },
                    MenuAction::NextPage => RunState::OpenContainerMenu {
                        highlighted: menu.get_next_page_index(*highlighted),
                        container: *container,
                    },
                    MenuAction::PreviousPage => RunState::OpenContainerMenu {
                        highlighted: menu.get_previous_page_index(*highlighted),
                        container: *container,
                    },
                    MenuAction::Select => match inventory_entities.get(*highlighted) {
                        Some(ent) => {
                            let mut intent = self.world.write_storage::<WantsToPickUpItem>();
                            intent
                                .insert(
                                    *self.world.fetch::<Entity>(),
                                    WantsToPickUpItem {
                                        item: *ent,
                                        container: entity_option::EntityOption::new(Some(
                                            *container,
                                        )),
                                    },
                                )
                                .expect("Unable To Insert Pick Up Item Intent");
                            RunState::PlayerTurn
                        }
                        None => RunState::OpenContainerMenu {
                            highlighted: *highlighted,
                            container: *container,
                        },
                    },
                    _ => RunState::OpenContainerMenu {
                        highlighted: *highlighted,
                        container: *container,
                    },
                }
            }
            RunState::ActionMenu { highlighted } => {
                let menu_options: Box<[MenuOption]> = [
                    "Use Item",
                    "Drop Item",
                    "Open Container",
                    "Search Area",
                    "Disarm Trap",
                    "Arm Trap",
                    "Grab Furniture",
                    "Release Furniture",
                    "Attack",
                    "Hide in Container",
                    "Douse",
                    "Light",
                    "Equipment Menu",
                ]
                .iter()
                .enumerate()
                .map(|(index, text)| {
                    let state = match *highlighted == index {
                        true => MenuOptionState::Highlighted,
                        false => MenuOptionState::Normal,
                    };
                    MenuOption::new(text, state)
                })
                .collect();
                let menu = Menu::new(menu_options, 10);
                ScreenMapMenu::new(
                    menu.get_page_at_index(*highlighted),
                    "Choose an action",
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::MoveHighlightNext => RunState::ActionMenu {
                        highlighted: menu.get_next_index(*highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::ActionMenu {
                        highlighted: menu.get_previous_index(*highlighted),
                    },
                    MenuAction::NoAction => RunState::ActionMenu {
                        highlighted: *highlighted,
                    },
                    MenuAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    MenuAction::Select => match highlighted {
                        0 => RunState::InventoryMenu { highlighted: 0 },
                        1 => RunState::DropItemMenu { highlighted: 0 },
                        2 => RunState::InteractionTypeEntityTargeting {
                            target_idx: 0,
                            targets: get_interaction_type_targets::<Container>(&self.world),
                            interaction_type: InteractionType::OpenContainer,
                            cta: Some(copy::CTA_INTERACT_OPEN_CONTAINER),
                        },
                        3 => {
                            player::search_hidden(&mut self.world);
                            RunState::PlayerTurn
                        }
                        4 => RunState::InteractionTypeEntityTargeting {
                            target_idx: 0,
                            targets: get_interaction_type_targets::<Disarmable>(&self.world),
                            interaction_type: InteractionType::Disarm,
                            cta: Some(copy::CTA_INTERACT_DISARM),
                        },
                        5 => RunState::InteractionTypeEntityTargeting {
                            target_idx: 0,
                            targets: get_interaction_type_targets::<Armable>(&self.world),
                            interaction_type: InteractionType::Disarm,
                            cta: Some(copy::CTA_INTERACT_DISARM),
                        },
                        6 => RunState::InteractionTypeEntityTargeting {
                            target_idx: 0,
                            targets: get_interaction_type_targets::<Grabbable>(&self.world),
                            interaction_type: InteractionType::Grab,
                            cta: Some(copy::CTA_INTERACT_GRAB),
                        },
                        7 => {
                            player::release_entity(&mut self.world);
                            RunState::AwaitingInput {
                                offset_x: 0,
                                offset_y: 0,
                            }
                        }
                        8 => RunState::InteractionTypeEntityTargeting {
                            target_idx: 0,
                            targets: get_interaction_type_targets::<CombatStats>(&self.world),
                            interaction_type: InteractionType::Attack,
                            cta: Some(copy::CTA_INTERACT_ATTACK),
                        },
                        9 => RunState::InteractionTypeEntityTargeting {
                            target_idx: 0,
                            targets: get_interaction_type_targets::<HidingSpot>(&self.world),
                            interaction_type: InteractionType::HideIn,
                            cta: Some(copy::CTA_INTERACT_HIDE),
                        },
                        10 => RunState::InteractionTypeEntityTargeting {
                            target_idx: 0,
                            targets: get_interaction_type_targets::<Dousable>(&self.world),
                            interaction_type: InteractionType::Douse,
                            cta: Some(copy::CTA_INTERACT_DOUSE),
                        },
                        11 => RunState::InteractionTypeEntityTargeting {
                            target_idx: 0,
                            targets: get_interaction_type_targets::<Lightable>(&self.world),
                            interaction_type: InteractionType::Light,
                            cta: Some(copy::CTA_INTERACT_LIGHT),
                        },
                        12 => RunState::EquipmentMenu {
                            highlighted: 0,
                            action_highlighted: 0,
                            action_menu: false,
                        },
                        _ => RunState::ActionMenu {
                            highlighted: *highlighted,
                        },
                    },
                    _ => RunState::ActionMenu {
                        highlighted: *highlighted,
                    },
                }
            }
            RunState::EquipMenu {
                highlighted,
                position,
            } => {
                let equipment: Vec<(String, Option<Entity>)> = {
                    let player_ent = self.world.fetch::<Entity>();
                    let equipment = self.world.read_storage::<Equipable>();
                    let names = self.world.read_storage::<Name>();
                    let inventories = self.world.read_storage::<Inventory>();
                    let player_inventory = inventories.get(*player_ent).unwrap();
                    iter::once((String::from("Nothing"), None))
                        .chain(
                            player_inventory
                                .items
                                .iter()
                                .filter(|ent| {
                                    if let Some(equip) = equipment.get(**ent) {
                                        return equip
                                            .positions
                                            .iter()
                                            .find(|p| *p == position)
                                            .is_some();
                                    }
                                    false
                                })
                                .map(|ent| (names.get(*ent).unwrap().name.clone(), Some(*ent))),
                        )
                        .collect()
                };

                let menu_options = equipment
                    .iter()
                    .enumerate()
                    .map(|(index, (name, _))| {
                        let state = match *highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(name, state)
                    })
                    .collect();
                let menu = Menu::new(menu_options, 10);
                ScreenMapMenu::new(
                    menu.get_page_at_index(*highlighted),
                    "Choose an Item to Equip",
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::MoveHighlightNext => RunState::EquipMenu {
                        highlighted: menu.get_next_index(*highlighted),
                        position: *position,
                    },
                    MenuAction::MoveHighlightPrev => RunState::EquipMenu {
                        highlighted: menu.get_next_index(*highlighted),
                        position: *position,
                    },
                    MenuAction::NoAction => RunState::EquipMenu {
                        highlighted: *highlighted,
                        position: *position,
                    },
                    MenuAction::Exit => RunState::EquipmentMenu {
                        highlighted: 0,
                        action_highlighted: 0,
                        action_menu: false,
                    },
                    MenuAction::Select => {
                        let (_, entity) = equipment[*highlighted];
                        player::equip_item(&mut self.world, entity, *position);
                        RunState::PlayerTurn
                    }
                    _ => RunState::EquipMenu {
                        highlighted: *highlighted,
                        position: *position,
                    },
                }
            }
            RunState::EquipmentMenu {
                highlighted,
                action_highlighted,
                action_menu,
            } => {
                let slots: Vec<(String, EquipmentPositions, Option<Entity>)> = {
                    let player_ent = self.world.fetch::<Entity>();
                    let equipment = self.world.read_storage::<Equipment>();
                    let player_equipment = equipment.get(*player_ent).unwrap();
                    let names = self.world.read_storage::<Name>();
                    let off_hand_item = match player_equipment.off_hand {
                        Some(e) => (names.get(e).unwrap().name.clone(), Some(e)),
                        None => (String::from("Empty"), None),
                    };
                    let dominant_hand_item = match player_equipment.dominant_hand {
                        Some(e) => (names.get(e).unwrap().name.clone(), Some(e)),
                        None => (String::from("Empty"), None),
                    };
                    vec![
                        (
                            format!("Dominant Hand: {}", dominant_hand_item.0),
                            EquipmentPositions::DominantHand,
                            dominant_hand_item.1,
                        ),
                        (
                            format!("Off Hand:  {}", off_hand_item.0),
                            EquipmentPositions::OffHand,
                            off_hand_item.1,
                        ),
                    ]
                };

                let mut submenu_actions = vec![];
                if let Some((_name, position, ent)) = slots.get(*highlighted) {
                    submenu_actions.push(("Exchange", EquipMenuType::Exchange(*position)));
                    if let Some(equipment_ent) = ent {
                        if self
                            .world
                            .read_storage::<Lightable>()
                            .get(*equipment_ent)
                            .is_some()
                        {
                            submenu_actions.push(("Light", EquipMenuType::Light(*equipment_ent)))
                        }
                        if self
                            .world
                            .read_storage::<Dousable>()
                            .get(*equipment_ent)
                            .is_some()
                        {
                            submenu_actions.push(("Douse", EquipMenuType::Douse(*equipment_ent)))
                        }
                    }
                }
                let description = {
                    match slots.get(*highlighted) {
                        Some((_position, _, e)) => match e {
                            Some(ent) => {
                                let info = self.world.read_storage::<Info>();
                                let ent_info = info.get(*ent);
                                match ent_info {
                                    Some(i) => String::from(&i.description),
                                    None => String::from("No Description"),
                                }
                            }
                            None => String::from("No Description"),
                        },
                        None => String::from("No Description"),
                    }
                };
                let submenu_options = submenu_actions
                    .iter()
                    .enumerate()
                    .map(|(index, (text, _))| {
                        let state = match *action_highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();

                let menu_options = slots
                    .iter()
                    .enumerate()
                    .map(|(index, (text, _position, _))| {
                        let state = match *highlighted == index {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        };
                        MenuOption::new(text, state)
                    })
                    .collect();
                let menu = Menu::new(menu_options, 10);
                let submenu = Menu::new(submenu_options, 10);
                ScreenMapItemMenu::new(
                    menu.get_page_at_index(*highlighted),
                    submenu.get_page_at_index(*action_highlighted),
                    *action_menu,
                    &description,
                    "Equipment",
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::MoveHighlightNext => RunState::EquipmentMenu {
                        highlighted: match action_menu {
                            true => *highlighted,
                            false => menu.get_next_index(*highlighted),
                        },
                        action_highlighted: match action_menu {
                            true => submenu.get_next_index(*action_highlighted),
                            false => *action_highlighted,
                        },
                        action_menu: *action_menu,
                    },
                    MenuAction::MoveHighlightPrev => RunState::EquipmentMenu {
                        highlighted: match action_menu {
                            true => *highlighted,
                            false => menu.get_previous_index(*highlighted),
                        },
                        action_highlighted: match action_menu {
                            true => submenu.get_next_index(*action_highlighted),
                            false => *action_highlighted,
                        },
                        action_menu: *action_menu,
                    },
                    MenuAction::NoAction => RunState::EquipmentMenu {
                        highlighted: *highlighted,
                        action_highlighted: *action_highlighted,
                        action_menu: *action_menu,
                    },
                    MenuAction::Exit => RunState::AwaitingInput {
                        offset_x: 0,
                        offset_y: 0,
                    },
                    MenuAction::Select => match submenu_actions[*action_highlighted].1 {
                        EquipMenuType::Exchange(position) => RunState::EquipMenu {
                            highlighted: 0,
                            position,
                        },
                        EquipMenuType::Douse(ent) => {
                            player::douse_item(&mut self.world, ent);
                            RunState::PlayerTurn
                        }
                        EquipMenuType::Light(ent) => {
                            player::light_item(&mut self.world, ent);
                            RunState::PlayerTurn
                        }
                    },
                    MenuAction::NextMenu | MenuAction::PreviousMenu => RunState::EquipmentMenu {
                        highlighted: *highlighted,
                        action_highlighted: *action_highlighted,
                        action_menu: !*action_menu,
                    },
                    _ => RunState::EquipmentMenu {
                        highlighted: *highlighted,
                        action_highlighted: *action_highlighted,
                        action_menu: *action_menu,
                    },
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
                let new_game_state = match *highlighted == 0 {
                    true => MenuOptionState::Highlighted,
                    false => MenuOptionState::Normal,
                };
                let continue_state = match has_save_game {
                    true => match *highlighted == 1 {
                        true => MenuOptionState::Highlighted,
                        false => MenuOptionState::Normal,
                    },
                    false => MenuOptionState::Disabled,
                };
                let credits_state = match *highlighted == 2 {
                    true => MenuOptionState::Highlighted,
                    false => MenuOptionState::Normal,
                };
                let menu = if cfg!(target_arch = "wasm32") {
                    Menu::new(
                        Box::new([
                            MenuOption::new("New Game", new_game_state),
                            MenuOption::new("Continue", continue_state),
                            MenuOption::new("Credits", credits_state),
                        ]),
                        10,
                    )
                } else {
                    let quit_state = match *highlighted == 3 {
                        true => MenuOptionState::Highlighted,
                        false => MenuOptionState::Normal,
                    };
                    Menu::new(
                        Box::new([
                            MenuOption::new("New Game", new_game_state),
                            MenuOption::new("Continue", continue_state),
                            MenuOption::new("Credits", credits_state),
                            MenuOption::new("Quit", quit_state),
                        ]),
                        10,
                    )
                };

                ScreenMainMenu::new(menu.get_page_at_index(*highlighted)).draw(ctx);
                match map_input_to_horizontal_menu_action(ctx) {
                    MenuAction::Exit => RunState::MainMenu {
                        highlighted: *highlighted,
                    },
                    MenuAction::MoveHighlightNext => RunState::MainMenu {
                        highlighted: menu.get_next_index(*highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::MainMenu {
                        highlighted: menu.get_previous_index(*highlighted),
                    },
                    MenuAction::Select => match *highlighted {
                        0 => {
                            initialize_new_game(&mut self.world);
                            RunState::IntroScreen
                        }
                        1 => {
                            persistence::load_game(&mut self.world);
                            persistence::delete_save();
                            RunState::AwaitingInput {
                                offset_x: 0,
                                offset_y: 0,
                            }
                        }
                        2 => RunState::CreditsScreen,
                        3 => std::process::exit(0),
                        _ => RunState::MainMenu {
                            highlighted: *highlighted,
                        },
                    },
                    _ => RunState::MainMenu {
                        highlighted: *highlighted,
                    },
                }
            }
            #[cfg(debug_assertions)]
            RunState::DebugMenu { highlighted } => {
                let menu_options = Box::new([
                    MenuOption::new(
                        "Wrath of God",
                        match *highlighted == 0 {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        },
                    ),
                    MenuOption::new(
                        "Gitaxian Probe",
                        match *highlighted == 1 {
                            true => MenuOptionState::Highlighted,
                            false => MenuOptionState::Normal,
                        },
                    ),
                ]);
                let menu = Menu::new(menu_options, 10);
                ScreenMapMenu::new(
                    menu.get_page_at_index(*highlighted),
                    "Debug Menu",
                    "Escape to Cancel",
                )
                .draw(ctx, &mut self.world);
                match map_input_to_menu_action(ctx) {
                    MenuAction::Exit => RunState::DebugMenu {
                        highlighted: *highlighted,
                    },
                    MenuAction::MoveHighlightNext => RunState::DebugMenu {
                        highlighted: menu.get_next_index(*highlighted),
                    },
                    MenuAction::MoveHighlightPrev => RunState::DebugMenu {
                        highlighted: menu.get_previous_index(*highlighted),
                    },
                    MenuAction::Select => match highlighted {
                        0 => {
                            debug::kill_all_monsters(&mut self.world);
                            self.world
                                .fetch_mut::<GameLog>()
                                .entries
                                .insert(0, "all monsters removed".to_owned());
                            RunState::AwaitingInput {
                                offset_x: 0,
                                offset_y: 0,
                            }
                        }
                        1 => {
                            debug::reveal_map(&mut self.world);
                            self.world
                                .fetch_mut::<GameLog>()
                                .entries
                                .insert(0, "map revealed".to_owned());
                            RunState::AwaitingInput {
                                offset_x: 0,
                                offset_y: 0,
                            }
                        }
                        _ => RunState::DebugMenu {
                            highlighted: *highlighted,
                        },
                    },
                    _ => RunState::DebugMenu {
                        highlighted: *highlighted,
                    },
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
        queued_action: None,
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
    gs.world.register::<WantsToPickUpItem>();
    gs.world.register::<WantsToUse>();
    gs.world.register::<WantsToDropItem>();
    gs.world.register::<ProvidesHealing>();
    gs.world.register::<Consumable>();
    gs.world.register::<Ranged>();
    gs.world.register::<AreaOfEffect>();
    gs.world.register::<Confusion>();
    gs.world.register::<Confused>();
    gs.world.register::<SimpleMarker<Saveable>>();
    gs.world.register::<SerializationHelper>();
    gs.world.register::<Blood>();
    gs.world.register::<ParticleLifetime>();
    gs.world.register::<Hidden>();
    gs.world.register::<EntryTrigger>();
    gs.world.register::<EntityMoved>();
    gs.world.register::<SingleActivation>();
    gs.world.register::<Triggered>();
    gs.world.register::<Objective>();
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
    gs.world.register::<HidingSpot>();
    gs.world.register::<Hiding>();
    gs.world.register::<WantsToHide>();
    gs.world.register::<Equipment>();
    gs.world.register::<Equipable>();
    gs.world.register::<WantsToEquip>();
    gs.world.register::<CausesDamage>();
    gs.world.register::<CausesLight>();
    gs.world.register::<Info>();
    gs.world.register::<Lightable>();
    gs.world.register::<Dousable>();
    gs.world.register::<WantsToDouse>();
    gs.world.register::<WantsToLight>();
    gs.world.register::<Disarmable>();
    gs.world.register::<Armable>();
    gs.world.register::<DamageHistory>();
    gs.world.register::<Inventory>();
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
    gs.world.insert(CorpseSpawner::new());
    let context = RltkBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Apprentice")
        .build()
        .expect("failed to create context");
    rltk::main_loop(context, gs).expect("failed to start apprentice");
}
