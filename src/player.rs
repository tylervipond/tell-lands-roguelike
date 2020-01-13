use crate::components::{
  combat_stats::CombatStats, in_backpack::InBackpack, item::Item, name::Name, player::Player,
  position::Position, viewshed::Viewshed, wants_to_melee::WantsToMelee,
  wants_to_pick_up_item::WantsToPickUpItem,
};
use crate::map::basic_map::Map;
use crate::{game_log::GameLog, map_action::MapAction};
use rltk::Point;
use specs::{Entity, Join, World, WorldExt};
use std::cmp::{max, min};


pub type InventoryList = Vec<(Entity, String)>;

// It's not really "try move player" though, it's more like "player act"
// because the function does more than just move the player. Should probably
// break this up a bit.
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut players = ecs.write_storage::<Player>();
  let mut viewsheds = ecs.write_storage::<Viewshed>();
  let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
  let combat_stats = ecs.read_storage::<CombatStats>();
  let entities = ecs.entities();
  let mut ppos = ecs.write_resource::<Point>();
  let map = ecs.fetch::<Map>();

  for (entity, _player, pos, viewshed) in
    (&entities, &mut players, &mut positions, &mut viewsheds).join()
  {
    let x = pos.x + delta_x;
    let y = pos.y + delta_y;
    let destination_index = map.xy_idx(x, y);

    for potential_target in map.tile_content[destination_index].iter() {
      let target = combat_stats.get(*potential_target);
      if let Some(_target) = target {
        wants_to_melee
          .insert(
            entity,
            WantsToMelee {
              target: *potential_target,
            },
          )
          .expect("Add target failed");
        return;
      }
    } // this block of code could be written in a better way.
    if !map.blocked[destination_index] {
      pos.x = min(79, max(0, x));
      pos.y = min(49, max(0, y));
      viewshed.dirty = true;
      ppos.x = pos.x;
      ppos.y = pos.y;
    }
  }
}

fn try_pickup_item(ecs: &mut World) {
  let player_pos = ecs.fetch::<Point>();
  let player_entity = ecs.fetch::<Entity>();
  let entities = ecs.entities();
  let items = ecs.read_storage::<Item>();
  let positions = ecs.read_storage::<Position>();
  let mut gamelog = ecs.fetch_mut::<GameLog>();

  let target_item: Option<Entity> = (&entities, &items, &positions).join().find_map(|p| {
    if p.2.x == player_pos.x && p.2.y == player_pos.y {
      return Some(p.0);
    }
    return None;
  });

  match target_item {
    None => gamelog
      .entries
      .insert(0, "there is nothing here to pick up".to_string()),
    Some(item) => {
      let mut pickup = ecs.write_storage::<WantsToPickUpItem>();
      pickup
        .insert(
          *player_entity,
          WantsToPickUpItem {
            collected_by: *player_entity,
            item,
          },
        )
        .expect("Unable to insert want to pick up");
    }
  }
}

pub fn get_player_inventory_list(ecs: &mut World) -> InventoryList {
  let player_entity = ecs.fetch::<Entity>();
  let names = ecs.read_storage::<Name>();
  let backpack = ecs.read_storage::<InBackpack>();
  let entities = ecs.entities();
  (&backpack, &entities, &names)
    .join()
    .filter(|i| i.0.owner == *player_entity)
    .map(|i| (i.1, i.2.name.to_string()))
    .collect()
}

pub fn player_action(ecs: &mut World, action: MapAction) {
  match action {
    MapAction::MoveLeft => try_move_player(-1, 0, ecs),
    MapAction::MoveRight => try_move_player(1, 0, ecs),
    MapAction::MoveUp => try_move_player(0, -1, ecs),
    MapAction::MoveDown => try_move_player(0, 1, ecs),
    MapAction::MoveUpLeft => try_move_player(-1, -1, ecs),
    MapAction::MoveUpRight => try_move_player(1, -1, ecs),
    MapAction::MoveDownLeft => try_move_player(-1, 1, ecs),
    MapAction::MoveDownRight => try_move_player(1, 1, ecs),
    MapAction::PickupItem => try_pickup_item(ecs),
    _ => {}
  }
}
