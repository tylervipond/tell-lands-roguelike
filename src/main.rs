rltk::add_wasm_support!();
use rltk::{Console, GameState, Point, RandomNumberGenerator, Rltk, RltkBuilder};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
#[macro_use]
extern crate specs_derive;
extern crate serde;
mod components;
mod credits_screen_action;
mod death_screen_action;
mod dungeon;
mod exit_game_menu_action;
mod failure_screen_action;
mod game_log;
mod gui;
mod input;
mod intro_screen_action;
mod inventory;
mod inventory_action;
mod main_menu_action;
mod main_menu_option;
mod map;
mod map_action;
mod persistence;
mod player;
mod ranged;
mod run_state;
mod services;
mod sizes;
mod spawner;
mod success_screen_action;
mod systems;
mod targeting_action;
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
use dungeon::dungeon::Dungeon;
use exit_game_menu_action::ExitGameMenuAction;
use failure_screen_action::FailureScreenAction;
use input::{
    map_input_to_credits_screen_action, map_input_to_death_screen_action,
    map_input_to_exit_game_action, map_input_to_failure_screen_action,
    map_input_to_intro_screen_action, map_input_to_inventory_action, map_input_to_main_menu_action,
    map_input_to_map_action, map_input_to_success_screen_action, map_input_to_targeting_action,
};
use intro_screen_action::IntroScreenAction;
use inventory_action::InventoryAction;
use main_menu_action::MainMenuAction;
use main_menu_option::MainMenuOption;
use map::draw_map;
use map_action::MapAction;
use player::{get_player_inventory_list, player_action};
use run_state::RunState;
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

fn draw_renderables_to_map(ecs: &World, ctx: &mut Rltk) {
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();
    let levels = ecs.read_storage::<DungeonLevel>();
    let hidden = ecs.read_storage::<Hidden>();
    let dungeon = ecs.fetch::<Dungeon>();
    let player_ent = ecs.fetch::<Entity>();
    let player_level = levels.get(*player_ent).unwrap();
    let map = dungeon.maps.get(&player_level.level).unwrap();
    let mut sorted_renderables = (&positions, &renderables, &levels, !&hidden)
        .join()
        .filter(|(p, _r, l, _h)| {
            return l.level == player_level.level
                && map.visible_tiles[map.xy_idx(p.x, p.y) as usize];
        })
        .collect::<Vec<_>>();
    sorted_renderables.sort_unstable_by(|a, b| b.1.layer.cmp(&a.1.layer));
    for (pos, render, _l, _h) in sorted_renderables.iter() {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
}

fn draw_screen(ecs: &mut World, ctx: &mut Rltk) {
    ctx.cls();
    draw_map(ecs, ctx);
    draw_renderables_to_map(ecs, ctx);
    gui::draw_ui(ecs, ctx);
    gui::draw_tooltip(ecs, ctx);
}

fn player_can_leave_dungeon(ecs: &mut World) -> bool {
    let player_ent = ecs.fetch::<Entity>();
    let dungeon_level = ecs.read_storage::<DungeonLevel>();
    let player_level = dungeon_level.get(*player_ent).unwrap();
    let mut dungeon = ecs.fetch_mut::<Dungeon>();
    let map = dungeon.get_map(player_level.level).unwrap();
    if let Some(exit_point) = map.exit {
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
    let map = dungeon.get_map(9).unwrap();
    let (player_x, player_y) = map.rooms[0].center();
    ecs.remove::<Point>();
    ecs.insert(Point::new(player_x, player_y));
    ecs.remove::<Entity>();
    let player_entity = spawner::spawn_player(ecs, player_x as i32, player_y as i32, 9);
    ecs.insert(player_entity);
    dungeon.maps.iter().for_each(|(i, m)| {
        for room in m.rooms.iter().skip(1) {
            spawner::spawn_entities_for_room(ecs, &room, *i);
        }
    });
    let mut rng = ecs.get_mut::<RandomNumberGenerator>().unwrap();
    let objective_floor = utils::get_random_between_numbers(rng, 1, 9);
    let map = dungeon.get_map(objective_floor).unwrap();
    let room = utils::get_random_between_numbers(rng, 0, (map.rooms.len() - 1) as i32);
    let (x, y) = map
        .rooms
        .get(room as usize)
        .unwrap()
        .get_random_coord(&mut rng);
    spawner::spawn_objective(ecs, map.xy_idx(x, y), objective_floor);

    ecs.remove::<Dungeon>();
    ecs.insert(dungeon);
    ecs.remove::<game_log::GameLog>();
    ecs.insert(game_log::GameLog {
        entries: vec!["Welcome to Tell-Lands".to_owned()],
    });
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
                draw_screen(&mut self.ecs, ctx);
                self.run_systems(ctx);
                DamageSystem::delete_the_dead(&mut self.ecs);
                RunState::AwaitingInput
            }
            RunState::AwaitingInput => {
                draw_screen(&mut self.ecs, ctx);
                self.run_systems(ctx);
                let action = map_input_to_map_action(ctx);
                match action {
                    MapAction::Exit => {
                        persistence::save_game(&mut self.ecs);
                        RunState::MainMenu { highlighted: 0 }
                    }
                    MapAction::NoAction => RunState::AwaitingInput,
                    MapAction::ShowInventoryMenu => RunState::InventoryMenu,
                    MapAction::ShowDropMenu => RunState::DropItemMenu,
                    MapAction::LeaveDungeon => match player_can_leave_dungeon(&mut self.ecs) {
                        true => RunState::ExitGameMenu { highlighted: 0 },
                        false => {
                            let mut log = self.ecs.fetch_mut::<game_log::GameLog>();
                            log.entries.push(
                                "You must first locate the exit to leave the dungeon".to_string(),
                            );
                            RunState::AwaitingInput
                        }
                    },
                    _ => {
                        player_action(&mut self.ecs, action);
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::PlayerTurn => {
                draw_screen(&mut self.ecs, ctx);
                self.run_systems(ctx);
                DamageSystem::delete_the_dead(&mut self.ecs);
                RunState::MonsterTurn
            }
            RunState::MonsterTurn => {
                draw_screen(&mut self.ecs, ctx);
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
            RunState::InventoryMenu => {
                draw_screen(&mut self.ecs, ctx);
                let inventory = get_player_inventory_list(&mut self.ecs);
                let (mut inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                gui::show_inventory(ctx, inventory_names, "Use Item");
                let action = map_input_to_inventory_action(ctx, &mut inventory_entities);
                match action {
                    InventoryAction::NoAction => RunState::InventoryMenu,
                    InventoryAction::Exit => RunState::AwaitingInput,
                    InventoryAction::Selected(ent) => {
                        let ranged = self.ecs.read_storage::<Ranged>();
                        if let Some(ranged_props) = ranged.get(ent) {
                            RunState::ShowTargeting {
                                range: ranged_props.range,
                                item: ent,
                            }
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUse>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUse {
                                        item: ent,
                                        target: None,
                                    },
                                )
                                .expect("Unable To Insert Use Item Intent");
                            RunState::PlayerTurn
                        }
                    }
                }
            }
            RunState::DropItemMenu => {
                draw_screen(&mut self.ecs, ctx);
                let inventory = get_player_inventory_list(&mut self.ecs);
                let (mut inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                gui::show_inventory(ctx, inventory_names, "Drop Item");
                let action = map_input_to_inventory_action(ctx, &mut inventory_entities);
                match action {
                    InventoryAction::NoAction => RunState::DropItemMenu,
                    InventoryAction::Exit => RunState::AwaitingInput,
                    InventoryAction::Selected(ent) => {
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent
                            .insert(*self.ecs.fetch::<Entity>(), WantsToDropItem { item: ent })
                            .expect("Unable To Insert Drop Item Intent");
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::ExitGameMenu { highlighted } => {
                draw_screen(&mut self.ecs, ctx);
                let menu = vec![
                    "Yes, exit the dungeon".to_string(),
                    "No, remain in the dungeon".to_string(),
                ];
                gui::show_exit_game_menu(ctx, &menu, highlighted);
                let action = map_input_to_exit_game_action(ctx, highlighted);
                match action {
                    ExitGameMenuAction::Exit => RunState::AwaitingInput,
                    ExitGameMenuAction::NoAction => RunState::ExitGameMenu { highlighted },
                    ExitGameMenuAction::MoveHighlightDown => RunState::ExitGameMenu {
                        highlighted: match highlighted + 1 > menu.len() {
                            true => 0,
                            false => highlighted + 1,
                        },
                    },
                    ExitGameMenuAction::MoveHighlightUp => RunState::ExitGameMenu {
                        highlighted: match highlighted == 0 {
                            true => menu.len(),
                            false => highlighted - 1,
                        },
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
                draw_screen(&mut self.ecs, ctx);
                let visible_tiles = ranged::get_visible_tiles_in_range(&self.ecs, range);
                gui::show_valid_targeting_area(ctx, &visible_tiles);
                let target = ranged::get_target(ctx, &visible_tiles);
                if let Some(point) = target {
                    gui::show_current_target(ctx, point);
                }
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
                ctx.cls();
                gui::show_intro_screen(ctx);
                let action = map_input_to_intro_screen_action(ctx);
                match action {
                    IntroScreenAction::Exit => RunState::MainMenu { highlighted: 0 },
                    IntroScreenAction::Continue => RunState::PreRun,
                    IntroScreenAction::NoAction => RunState::IntroScreen,
                }
            }
            RunState::DeathScreen => {
                ctx.cls();
                gui::show_death_screen(ctx);
                let action = map_input_to_death_screen_action(ctx);
                match action {
                    DeathScreenAction::Exit => RunState::MainMenu { highlighted: 0 },
                    DeathScreenAction::NoAction => RunState::DeathScreen,
                }
            }
            RunState::SuccessScreen => {
                ctx.cls();
                gui::show_success_screen(ctx);
                let action = map_input_to_success_screen_action(ctx);
                match action {
                    SuccessScreenAction::Exit => RunState::CreditsScreen,
                    SuccessScreenAction::NoAction => RunState::SuccessScreen,
                }
            }
            RunState::FailureScreen => {
                ctx.cls();
                gui::show_failure_screen(ctx);
                let action = map_input_to_failure_screen_action(ctx);
                match action {
                    FailureScreenAction::Exit => RunState::MainMenu { highlighted: 0 },
                    FailureScreenAction::NoAction => RunState::FailureScreen,
                }
            }
            RunState::CreditsScreen => {
                ctx.cls();
                gui::show_credits_screen(ctx);
                let action = map_input_to_credits_screen_action(ctx);
                match action {
                    CreditsScreenAction::Exit => RunState::MainMenu { highlighted: 0 },
                    CreditsScreenAction::NoAction => RunState::CreditsScreen,
                }
            }
            RunState::MainMenu { highlighted } => {
                let has_save_game = persistence::has_save_game();
                let menu = vec![
                    MainMenuOption::new("New Game", false),
                    MainMenuOption::new("Continue", !has_save_game),
                    MainMenuOption::new("Quit", false),
                ];

                ctx.cls();
                gui::show_main_menu(ctx, &menu, highlighted);
                let action = map_input_to_main_menu_action(ctx, highlighted);
                match action {
                    MainMenuAction::Exit => RunState::MainMenu { highlighted },
                    MainMenuAction::NoAction => RunState::MainMenu { highlighted },
                    MainMenuAction::MoveHighlightDown => RunState::MainMenu {
                        highlighted: match menu.get(highlighted + 1) {
                            Some(o) => match o.disabled {
                                true => match highlighted < menu.len() - 1 {
                                    true => highlighted + 2,
                                    false => 0,
                                },
                                false => highlighted + 1,
                            },
                            None => 0,
                        },
                    },
                    MainMenuAction::MoveHighlightUp => RunState::MainMenu {
                        highlighted: match highlighted {
                            0 => menu.len() - 1,
                            _ => match menu.get(highlighted - 1) {
                                Some(o) => match o.disabled {
                                    true => match highlighted > 1 {
                                        true => highlighted - 2,
                                        false => menu.len() - 1,
                                    },
                                    false => highlighted - 1,
                                },
                                None => menu.len() - 1,
                            },
                        },
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
        };
        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_runstate
        }
    }
}

fn main() {
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
    let rng = RandomNumberGenerator::new();
    gs.ecs.insert(rng);
    gs.ecs.insert(RunState::MainMenu { highlighted: 0 });
    gs.ecs
        .insert(services::particle_effect_spawner::ParticleEffectSpawner::new());
    gs.ecs.insert(services::blood_spawner::BloodSpawner::new());
    let context = RltkBuilder::simple80x50().with_title("Apprentice").build();
    rltk::main_loop(context, gs);
}
