use rltk::{GameState, Point, RandomNumberGenerator, Rltk, RltkBuilder};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate specs_derive;
extern crate serde;
mod components;
mod credits_screen_action;
mod death_screen_action;
#[cfg(debug_assertions)]
mod debug_menu_action;
mod dungeon;
mod exit_game_menu_action;
mod failure_screen_action;
mod game_log;
mod input;
mod intro_screen_action;
mod inventory_action;
mod main_menu_action;
mod map_action;
mod menu_option;
mod persistence;
mod player;
mod ranged;
mod run_state;
mod screens;
mod services;
mod spawner;
mod success_screen_action;
mod systems;
mod targeting_action;
mod ui_components;
mod utils;
use components::{
    area_of_effect::AreaOfEffect, blocks_tile::BlocksTile, blood::Blood, combat_stats::CombatStats,
    confused::Confused, confusion::Confusion, consumable::Consumable, dungeon_level::DungeonLevel,
    entity_moved::EntityMoved, entry_trigger::EntryTrigger, hidden::Hidden,
    in_backpack::InBackpack, inflicts_damage::InflictsDamage, item::Item, monster::Monster,
    name::Name, objective::Objective, particle_lifetime::ParticleLifetime, player::Player,
    position::Position, potion::Potion, provides_healing::ProvidesHealing, ranged::Ranged,
    renderable::Renderable, saveable::Saveable, serialization_helper::SerializationHelper,
    single_activation::SingleActivation, suffer_damage::SufferDamage, triggered::Triggered,
    viewshed::Viewshed, wants_to_drop_item::WantsToDropItem, wants_to_melee::WantsToMelee,
    wants_to_pick_up_item::WantsToPickUpItem, wants_to_use::WantsToUse,
};
use credits_screen_action::CreditsScreenAction;
use death_screen_action::DeathScreenAction;
#[cfg(debug_assertions)]
use debug_menu_action::DebugMenuAction;
use dungeon::dungeon::Dungeon;
use exit_game_menu_action::ExitGameMenuAction;
use failure_screen_action::FailureScreenAction;
#[cfg(debug_assertions)]
use input::map_input_to_debug_menu_action;
use input::{
    map_input_to_credits_screen_action, map_input_to_death_screen_action,
    map_input_to_exit_game_action, map_input_to_failure_screen_action,
    map_input_to_intro_screen_action, map_input_to_inventory_action, map_input_to_main_menu_action,
    map_input_to_map_action, map_input_to_success_screen_action, map_input_to_targeting_action,
};
use intro_screen_action::IntroScreenAction;
use inventory_action::InventoryAction;
use main_menu_action::MainMenuAction;
use map_action::MapAction;
use menu_option::{MenuOption, MenuOptionState};
use player::{get_player_inventory_list, player_action};
use run_state::RunState;
use screens::{
    screen_credits::ScreenCredits, screen_death::ScreenDeath, screen_failure::ScreenFailure,
    screen_intro::ScreenIntro, screen_main_menu::ScreenMainMenu,
    screen_map_generic::ScreenMapGeneric, screen_map_menu::ScreenMapMenu,
    screen_map_targeting::ScreenMapTargeting, screen_success::ScreenSuccess,
};
use success_screen_action::SuccessScreenAction;
use systems::{
    blood_spawn_system::BloodSpawnSystem, damage_system::DamageSystem,
    item_collection_system::ItemCollectionSystem, item_drop_system::ItemDropSystem,
    map_indexing_system::MapIndexingSystem, melee_combat_system::MeleeCombatSystem,
    monster_ai_system::MonsterAI, particle_spawn_system::ParticleSpawnSystem,
    remove_particle_effects_system::RemoveParticleEffectsSystem,
    remove_triggered_traps_system::RemoveTriggeredTrapsSystem,
    reveal_traps_system::RevealTrapsSystem, trigger_system::TriggerSystem,
    update_particle_effects_system::UpdateParticleEffectsSystem, use_item_system::UseItemSystem,
    visibility_system::VisibilitySystem,
};
use targeting_action::TargetingAction;

fn player_can_leave_dungeon(ecs: &mut World) -> bool {
    let player_ent = ecs.fetch::<Entity>();
    let dungeon_level = ecs.read_storage::<DungeonLevel>();
    let player_level = dungeon_level.get(*player_ent).unwrap();
    let mut dungeon = ecs.fetch_mut::<Dungeon>();
    let level = dungeon.get_level(player_level.level).unwrap();
    if let Some(exit_point) = level.exit {
        let player_point = ecs.fetch::<Point>();
        return player_point.x == exit_point.x && player_point.y == exit_point.y;
    }
    false
}

fn has_objective_in_backpack(ecs: &World) -> bool {
    let player_ent = ecs.fetch::<Entity>();
    let backpacks = ecs.read_storage::<InBackpack>();
    let objectives = ecs.read_storage::<Objective>();
    for (_objective, backpack) in (&objectives, &backpacks).join() {
        if backpack.owner == *player_ent {
            return true;
        }
    }
    false
}

#[cfg(debug_assertions)]
fn kill_all_monsters(ecs: &mut World) {
    let monster_ents: Vec<Entity> = {
        let entities = ecs.entities();
        let monsters = ecs.read_storage::<Monster>();
        (&entities, &monsters).join().map(|(e, _)| e).collect()
    };
    ecs.delete_entities(&monster_ents)
        .expect("couldn't delete ents");
}

#[cfg(debug_assertions)]
fn reveal_map(ecs: &mut World) {
    use dungeon::constants::MAP_COUNT;
    let player_ent = ecs.fetch::<Entity>();
    let dungeon_level = ecs.read_storage::<DungeonLevel>();
    let player_level = dungeon_level.get(*player_ent).unwrap();
    let mut dungeon = ecs.fetch_mut::<Dungeon>();
    let mut level = dungeon.get_level(player_level.level).unwrap();
    level.revealed_tiles = vec![true; MAP_COUNT]
}

fn initialize_new_game(ecs: &mut World) {
    ecs.write_storage::<Position>().clear();
    ecs.write_storage::<Renderable>().clear();
    ecs.write_storage::<Player>().clear();
    ecs.write_storage::<Viewshed>().clear();
    ecs.write_storage::<Monster>().clear();
    ecs.write_storage::<Name>().clear();
    ecs.write_storage::<BlocksTile>().clear();
    ecs.write_storage::<WantsToMelee>().clear();
    ecs.write_storage::<SufferDamage>().clear();
    ecs.write_storage::<CombatStats>().clear();
    ecs.write_storage::<Item>().clear();
    ecs.write_storage::<Potion>().clear();
    ecs.write_storage::<InBackpack>().clear();
    ecs.write_storage::<WantsToPickUpItem>().clear();
    ecs.write_storage::<WantsToUse>().clear();
    ecs.write_storage::<WantsToDropItem>().clear();
    ecs.write_storage::<ProvidesHealing>().clear();
    ecs.write_storage::<Consumable>().clear();
    ecs.write_storage::<Ranged>().clear();
    ecs.write_storage::<InflictsDamage>().clear();
    ecs.write_storage::<AreaOfEffect>().clear();
    ecs.write_storage::<Confusion>().clear();
    ecs.write_storage::<Confused>().clear();
    ecs.write_storage::<SimpleMarker<Saveable>>().clear();
    ecs.write_storage::<SerializationHelper>().clear();
    ecs.write_storage::<DungeonLevel>().clear();
    ecs.write_storage::<Blood>().clear();
    ecs.write_storage::<ParticleLifetime>().clear();
    ecs.write_storage::<Hidden>().clear();
    ecs.write_storage::<EntryTrigger>().clear();
    ecs.write_storage::<EntityMoved>().clear();
    ecs.write_storage::<SingleActivation>().clear();
    ecs.write_storage::<Triggered>().clear();
    ecs.write_storage::<Objective>().clear();
    ecs.remove::<SimpleMarkerAllocator<Saveable>>();
    ecs.insert(SimpleMarkerAllocator::<Saveable>::new());
    let mut dungeon = Dungeon::generate(1, 10);
    let level = dungeon.get_level(9).unwrap();
    let (player_x, player_y) = level.rooms[0].center();
    ecs.remove::<Point>();
    ecs.insert(Point::new(player_x, player_y));
    ecs.remove::<Entity>();
    let player_entity = spawner::spawn_player(ecs, player_x as i32, player_y as i32, 9);
    ecs.insert(player_entity);
    dungeon.levels.iter().for_each(|(_, l)| {
        for room in l.rooms.iter().skip(1) {
            spawner::spawn_entities_for_room(ecs, &room, &l);
        }
    });
    let rng = ecs.get_mut::<RandomNumberGenerator>().unwrap();
    let objective_floor = utils::get_random_between_numbers(rng, 1, 9);
    let level = dungeon.get_level(objective_floor).unwrap();
    let room_idx = utils::get_random_between_numbers(rng, 0, (level.rooms.len() - 1) as i32);
    let room = level.rooms.get(room_idx as usize).unwrap();
    spawner::spawn_objective_for_room(ecs, &room, &level);

    ecs.remove::<Dungeon>();
    ecs.insert(dungeon);
    ecs.remove::<game_log::GameLog>();
    ecs.insert(game_log::GameLog {
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
    ecs: World,
}

impl State {
    fn run_systems(&mut self, ctx: &mut Rltk) {
        let mut update_particles = UpdateParticleEffectsSystem {
            elapsed_time: ctx.frame_time_ms,
        };
        update_particles.run_now(&self.ecs);
        let mut remove_particles = RemoveParticleEffectsSystem {};
        remove_particles.run_now(&self.ecs);
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee_combat = MeleeCombatSystem {};
        melee_combat.run_now(&self.ecs);
        let mut triggers = TriggerSystem {};
        triggers.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);
        let mut to_use = UseItemSystem {};
        to_use.run_now(&self.ecs);
        let mut drop = ItemDropSystem {};
        drop.run_now(&self.ecs);
        let mut remove_triggered_single_activation_traps_system = RemoveTriggeredTrapsSystem {};
        remove_triggered_single_activation_traps_system.run_now(&self.ecs);
        let mut reveal_traps = RevealTrapsSystem {};
        reveal_traps.run_now(&self.ecs);
        let mut blood_spawn_system = BloodSpawnSystem {};
        blood_spawn_system.run_now(&self.ecs);
        let mut particle_spawn_system = ParticleSpawnSystem {};
        particle_spawn_system.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let old_runstate = { *(self.ecs.fetch::<RunState>()) };
        let new_runstate = match old_runstate {
            RunState::PreRun => {
                ScreenMapGeneric::new().draw(ctx, &mut self.ecs);
                self.run_systems(ctx);
                DamageSystem::delete_the_dead(&mut self.ecs);
                RunState::AwaitingInput
            }
            RunState::AwaitingInput => {
                ScreenMapGeneric::new().draw(ctx, &mut self.ecs);
                self.run_systems(ctx);
                let action = map_input_to_map_action(ctx);
                match action {
                    MapAction::Exit => {
                        persistence::save_game(&mut self.ecs);
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
                    MapAction::LeaveDungeon => match player_can_leave_dungeon(&mut self.ecs) {
                        true => RunState::ExitGameMenu { highlighted: 0 },
                        false => {
                            let mut log = self.ecs.fetch_mut::<game_log::GameLog>();
                            log.entries.insert(
                                0,
                                "You must first locate the exit to leave the dungeon".to_string(),
                            );
                            RunState::AwaitingInput
                        }
                    },
                    #[cfg(debug_assertions)]
                    MapAction::ShowDebugMenu => RunState::DebugMenu { highlighted: 0 },
                    _ => {
                        player_action(&mut self.ecs, action);
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::PlayerTurn => {
                ScreenMapGeneric::new().draw(ctx, &mut self.ecs);
                self.run_systems(ctx);
                DamageSystem::delete_the_dead(&mut self.ecs);
                RunState::MonsterTurn
            }
            RunState::MonsterTurn => {
                ScreenMapGeneric::new().draw(ctx, &mut self.ecs);
                self.run_systems(ctx);
                DamageSystem::delete_the_dead(&mut self.ecs);
                let combat_stats = self.ecs.read_storage::<CombatStats>();
                let player_ent = self.ecs.fetch::<Entity>();
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
                let inventory = get_player_inventory_list(&mut self.ecs);
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
                .draw(ctx, &mut self.ecs);
                let action = map_input_to_inventory_action(ctx, highlighted);
                match action {
                    InventoryAction::NoAction => RunState::InventoryMenu { highlighted, page },
                    InventoryAction::Exit => RunState::AwaitingInput,
                    InventoryAction::MoveHighlightDown => RunState::InventoryMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                        page,
                    },
                    InventoryAction::MoveHighlightUp => RunState::InventoryMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                        page,
                    },
                    InventoryAction::NextPage => RunState::InventoryMenu {
                        highlighted,
                        page: select_next_menu_page(page, total_pages),
                    },
                    InventoryAction::PreviousPage => RunState::InventoryMenu {
                        highlighted,
                        page: select_prev_menu_page(page),
                    },
                    InventoryAction::Select { option } => {
                        match inventory_entities.get(page * items_per_page + option) {
                            Some(ent) => {
                                let ranged = self.ecs.read_storage::<Ranged>();
                                let is_ranged = ranged.get(*ent);
                                match is_ranged {
                                    Some(ranged_props) => RunState::ShowTargeting {
                                        range: ranged_props.range,
                                        item: *ent,
                                    },
                                    None => {
                                        let mut intent = self.ecs.write_storage::<WantsToUse>();
                                        intent
                                            .insert(
                                                *self.ecs.fetch::<Entity>(),
                                                WantsToUse {
                                                    item: *ent,
                                                    target: None,
                                                },
                                            )
                                            .expect("Unable To Insert Use Item Intent");
                                        RunState::PlayerTurn
                                    }
                                }
                            },
                            None => RunState::InventoryMenu { highlighted, page },
                        }
                    }
                }
            }
            RunState::DropItemMenu { highlighted, page } => {
                let inventory = get_player_inventory_list(&mut self.ecs);
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
                .draw(ctx, &mut self.ecs);
                let action = map_input_to_inventory_action(ctx, highlighted);
                match action {
                    InventoryAction::NoAction => RunState::DropItemMenu { highlighted, page },
                    InventoryAction::Exit => RunState::AwaitingInput,
                    InventoryAction::MoveHighlightDown => RunState::DropItemMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                        page,
                    },
                    InventoryAction::MoveHighlightUp => RunState::DropItemMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                        page,
                    },
                    InventoryAction::NextPage => RunState::DropItemMenu {
                        highlighted,
                        page: select_next_menu_page(page, total_pages),
                    },
                    InventoryAction::PreviousPage => RunState::DropItemMenu {
                        highlighted,
                        page: select_prev_menu_page(page),
                    },
                    InventoryAction::Select { option } => {
                        let ent = inventory_entities
                            .get(page * items_per_page + option)
                            .expect("got");
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent
                            .insert(*self.ecs.fetch::<Entity>(), WantsToDropItem { item: *ent })
                            .expect("Unable To Insert Drop Item Intent");
                        RunState::PlayerTurn
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
                    .draw(ctx, &mut self.ecs);
                let action = map_input_to_exit_game_action(ctx, highlighted);
                match action {
                    ExitGameMenuAction::Exit => RunState::AwaitingInput,
                    ExitGameMenuAction::NoAction => RunState::ExitGameMenu { highlighted },
                    ExitGameMenuAction::MoveHighlightDown => RunState::ExitGameMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                    },
                    ExitGameMenuAction::MoveHighlightUp => RunState::ExitGameMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                    },
                    ExitGameMenuAction::Select { option } => match option {
                        0 => {
                            persistence::delete_save();
                            match has_objective_in_backpack(&self.ecs) {
                                true => RunState::SuccessScreen,
                                false => RunState::FailureScreen,
                            }
                        }
                        _ => RunState::AwaitingInput,
                    },
                }
            }
            RunState::ShowTargeting { range, item } => {
                let visible_tiles = ranged::get_visible_tiles_in_range(&self.ecs, range);
                let target = ranged::get_target(ctx, &visible_tiles);
                ScreenMapTargeting::new(range, target).draw(ctx, &mut self.ecs);
                let action = map_input_to_targeting_action(ctx, target);
                match action {
                    TargetingAction::NoAction => RunState::ShowTargeting { range, item },
                    TargetingAction::Exit => RunState::AwaitingInput,
                    TargetingAction::Selected(target) => {
                        let mut intent = self.ecs.write_storage::<WantsToUse>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToUse {
                                    item,
                                    target: Some(target),
                                },
                            )
                            .expect("Unable To Insert Drop Item Intent");
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::IntroScreen => {
                ScreenIntro::new().draw(ctx);
                let action = map_input_to_intro_screen_action(ctx);
                match action {
                    IntroScreenAction::Exit => RunState::MainMenu { highlighted: 0 },
                    IntroScreenAction::Continue => RunState::PreRun,
                    IntroScreenAction::NoAction => RunState::IntroScreen,
                }
            }
            RunState::DeathScreen => {
                ScreenDeath::new().draw(ctx);
                let action = map_input_to_death_screen_action(ctx);
                match action {
                    DeathScreenAction::Exit => RunState::MainMenu { highlighted: 0 },
                    DeathScreenAction::NoAction => RunState::DeathScreen,
                }
            }
            RunState::SuccessScreen => {
                ScreenSuccess::new().draw(ctx);
                let action = map_input_to_success_screen_action(ctx);
                match action {
                    SuccessScreenAction::Exit => RunState::CreditsScreen,
                    SuccessScreenAction::NoAction => RunState::SuccessScreen,
                }
            }
            RunState::FailureScreen => {
                ScreenFailure::new().draw(ctx);
                let action = map_input_to_failure_screen_action(ctx);
                match action {
                    FailureScreenAction::Exit => RunState::MainMenu { highlighted: 0 },
                    FailureScreenAction::NoAction => RunState::FailureScreen,
                }
            }
            RunState::CreditsScreen => {
                ScreenCredits::new().draw(ctx);
                let action = map_input_to_credits_screen_action(ctx);
                match action {
                    CreditsScreenAction::Exit => RunState::MainMenu { highlighted: 0 },
                    CreditsScreenAction::NoAction => RunState::CreditsScreen,
                }
            }
            RunState::MainMenu { highlighted } => {
                let has_save_game = persistence::has_save_game();
                let continue_state = match has_save_game {
                    true => match highlighted == 1 {
                        true => MenuOptionState::Highlighted,
                        false => MenuOptionState::Normal,
                    },
                    false => MenuOptionState::Disabled,
                };
                let new_game_state = match highlighted == 0 {
                    true => MenuOptionState::Highlighted,
                    false => MenuOptionState::Normal,
                };
                let menu = if cfg!(target_arch = "wasm32") {
                    vec![
                        MenuOption::new("New Game", new_game_state),
                        MenuOption::new("Continue", continue_state),
                    ]
                } else {
                    let quit_state = match highlighted == 2 {
                        true => MenuOptionState::Highlighted,
                        false => MenuOptionState::Normal,
                    };
                    vec![
                        MenuOption::new("New Game", new_game_state),
                        MenuOption::new("Continue", continue_state),
                        MenuOption::new("Quit", quit_state),
                    ]
                };

                ScreenMainMenu::new(&menu).draw(ctx);
                let action = map_input_to_main_menu_action(ctx, highlighted);
                match action {
                    MainMenuAction::Exit => RunState::MainMenu { highlighted },
                    MainMenuAction::NoAction => RunState::MainMenu { highlighted },
                    MainMenuAction::MoveHighlightDown => RunState::MainMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                    },
                    MainMenuAction::MoveHighlightUp => RunState::MainMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                    },
                    MainMenuAction::Select { option } => match option {
                        0 => {
                            initialize_new_game(&mut self.ecs);
                            RunState::IntroScreen
                        }
                        1 => {
                            persistence::load_game(&mut self.ecs);
                            persistence::delete_save();
                            RunState::AwaitingInput
                        }
                        2 => std::process::exit(0),
                        _ => RunState::MainMenu { highlighted },
                    },
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
                    .draw(ctx, &mut self.ecs);
                let action = map_input_to_debug_menu_action(ctx, highlighted);
                match action {
                    DebugMenuAction::Exit => RunState::DebugMenu { highlighted },
                    DebugMenuAction::NoAction => RunState::DebugMenu { highlighted },
                    DebugMenuAction::MoveHighlightDown => RunState::DebugMenu {
                        highlighted: menu_option::select_next_menu_index(&menu, highlighted),
                    },
                    DebugMenuAction::MoveHighlightUp => RunState::DebugMenu {
                        highlighted: menu_option::select_previous_menu_index(&menu, highlighted),
                    },
                    DebugMenuAction::Select { option } => match option {
                        0 => {
                            kill_all_monsters(&mut self.ecs);
                            self.ecs
                                .fetch_mut::<game_log::GameLog>()
                                .entries
                                .insert(0, "all monsters removed".to_owned());
                            RunState::AwaitingInput
                        }
                        1 => {
                            reveal_map(&mut self.ecs);
                            self.ecs
                                .fetch_mut::<game_log::GameLog>()
                                .entries
                                .insert(0, "map revealed".to_owned());
                            RunState::AwaitingInput
                        }
                        _ => RunState::DebugMenu { highlighted },
                    },
                }
            }
        };
        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_runstate
        }
    }
}

#[wasm_bindgen]
pub fn start() {
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickUpItem>();
    gs.ecs.register::<WantsToUse>();
    gs.ecs.register::<WantsToDropItem>();
    gs.ecs.register::<ProvidesHealing>();
    gs.ecs.register::<Consumable>();
    gs.ecs.register::<Ranged>();
    gs.ecs.register::<InflictsDamage>();
    gs.ecs.register::<AreaOfEffect>();
    gs.ecs.register::<Confusion>();
    gs.ecs.register::<Confused>();
    gs.ecs.register::<SimpleMarker<Saveable>>();
    gs.ecs.register::<SerializationHelper>();
    gs.ecs.register::<DungeonLevel>();
    gs.ecs.register::<Blood>();
    gs.ecs.register::<ParticleLifetime>();
    gs.ecs.register::<Hidden>();
    gs.ecs.register::<EntryTrigger>();
    gs.ecs.register::<EntityMoved>();
    gs.ecs.register::<SingleActivation>();
    gs.ecs.register::<Triggered>();
    gs.ecs.register::<Objective>();
    gs.ecs.insert(SimpleMarkerAllocator::<Saveable>::new());
    gs.ecs.insert(game_log::GameLog {
        entries: vec!["Enter the dungeon apprentice! Bring back the Talisman!".to_owned()],
    }); // This needs to get moved to a continue game function I think...
    let rng = RandomNumberGenerator::new();
    gs.ecs.insert(rng);
    gs.ecs.insert(RunState::MainMenu { highlighted: 0 });
    gs.ecs
        .insert(services::particle_effect_spawner::ParticleEffectSpawner::new());
    gs.ecs.insert(services::blood_spawner::BloodSpawner::new());
    let context = RltkBuilder::simple80x50()
        .with_title("Apprentice")
        .build()
        .expect("failed to create context");
    rltk::main_loop(context, gs).expect("failed to start apprentice");
}
