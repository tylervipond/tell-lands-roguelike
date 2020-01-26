rltk::add_wasm_support!();
use rltk::{Console, GameState, Point, RandomNumberGenerator, Rltk};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;
mod components;
mod game_log;
mod gui;
mod input;
mod inventory;
mod inventory_action;
mod map;
mod map_action;
mod player;
mod run_state;
mod spawner;
mod systems;
mod utils;

use components::{
    blocks_tile::BlocksTile, combat_stats::CombatStats, consumable::Consumable,
    in_backpack::InBackpack, item::Item, monster::Monster, name::Name, player::Player,
    position::Position, potion::Potion, provides_healing::ProvidesHealing, renderable::Renderable,
    suffer_damage::SufferDamage, viewshed::Viewshed, wants_to_drop_item::WantsToDropItem,
    wants_to_melee::WantsToMelee, wants_to_pick_up_item::WantsToPickUpItem,
    wants_to_use::WantsToUse,
};
use game_log::GameLog;
use input::{map_input_to_inventory_action, map_input_to_map_action};
use inventory_action::InventoryAction;
use map::{draw_map, Map};
use map_action::MapAction;
use player::{get_player_inventory_list, player_action};
use run_state::RunState;
use systems::{
    damage_system::DamageSystem, item_collection_system::ItemCollectionSystem,
    item_drop_system::ItemDropSystem, map_indexing_system::MapIndexingSystem,
    melee_combat_system::MeleeCombatSystem, monster_ai_system::MonsterAI,
    use_item_system::UseItemSystem, visibility_system::VisibilitySystem,
};

fn draw_renderables_to_map(ecs: &World, ctx: &mut Rltk) {
    let positions = ecs.read_storage::<Position>();
    let renderables = ecs.read_storage::<Renderable>();
    let map = ecs.fetch::<Map>();
    let mut sorted_renderables = (&positions, &renderables).join().collect::<Vec<_>>();
    sorted_renderables.sort_unstable_by(|a, b| b.1.layer.cmp(&a.1.layer));
    for (pos, render) in sorted_renderables.iter() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn draw_screen(ecs: &World, ctx: &mut Rltk) {
    ctx.cls();
    draw_map(ecs, ctx);
    draw_renderables_to_map(ecs, ctx);
    gui::draw_ui(ecs, ctx)
}

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee_combat = MeleeCombatSystem {};
        melee_combat.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);
        let mut to_use = UseItemSystem {};
        to_use.run_now(&self.ecs);
        let mut drop = ItemDropSystem {};
        drop.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        draw_screen(&self.ecs, ctx);
        let old_runstate = { *(self.ecs.fetch::<RunState>()) };
        let new_runstate = match old_runstate {
            RunState::PreRun => {
                self.run_systems();
                RunState::AwaitingInput
            }
            RunState::AwaitingInput => {
                let action = map_input_to_map_action(ctx);
                match action {
                    MapAction::NoAction => RunState::AwaitingInput,
                    MapAction::ShowInventoryMenu => RunState::InventoryMenu,
                    MapAction::ShowDropMenu => RunState::DropItemMenu,
                    _ => {
                        player_action(&mut self.ecs, action);
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::PlayerTurn => {
                self.run_systems();
                RunState::MonsterTurn
            }
            RunState::MonsterTurn => {
                self.run_systems();
                RunState::AwaitingInput
            }
            RunState::InventoryMenu => {
                let inventory = get_player_inventory_list(&mut self.ecs);
                let (mut inventory_entities, inventory_names): (Vec<_>, Vec<_>) =
                    inventory.into_iter().unzip();
                gui::show_inventory(ctx, inventory_names, "Use Item");
                let action = map_input_to_inventory_action(ctx, &mut inventory_entities);
                match action {
                    InventoryAction::NoAction => RunState::InventoryMenu,
                    InventoryAction::Exit => RunState::AwaitingInput,
                    InventoryAction::Selected(ent) => {
                        let mut intent = self.ecs.write_storage::<WantsToUse>();
                        intent
                            .insert(*self.ecs.fetch::<Entity>(), WantsToUse { item: ent })
                            .expect("Unable To Insert Use Item Intent");
                        RunState::PlayerTurn
                    }
                }
            }
            RunState::DropItemMenu => {
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
        };
        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_runstate
        }
        DamageSystem::delete_the_dead(&mut self.ecs);
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
    gs.ecs.insert(game_log::GameLog {
        entries: vec!["Welcome to Tell-Lands".to_owned()],
    });
    let map = Map::create_basic_map();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(Point::new(player_x, player_y));
    let player_entity = spawner::spawn_player(&mut gs.ecs, player_x, player_y);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    let rng = RandomNumberGenerator::new();
    gs.ecs.insert(rng);
    for (_i, room) in map.rooms.iter().skip(1).enumerate() {
        spawner::spawn_entities_for_room(&mut gs.ecs, &room);
    }
    gs.ecs.insert(map);
    let context = Rltk::init_simple8x8(80, 50, "Tell-Lands", "resources");
    rltk::main_loop(context, gs);
}
