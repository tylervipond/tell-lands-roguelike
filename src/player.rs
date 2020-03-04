use crate::components::{
  combat_stats::CombatStats, dungeon_level::DungeonLevel, in_backpack::InBackpack, item::Item,
  name::Name, player::Player, position::Position, viewshed::Viewshed, wants_to_melee::WantsToMelee,
  wants_to_pick_up_item::WantsToPickUpItem,
};
use crate::dungeon::dungeon::Dungeon;
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
  let mut dungeon = ecs.fetch_mut::<Dungeon>();
  let player_ent = ecs.fetch::<Entity>();
  let dungeon_levels = ecs.read_storage::<DungeonLevel>();

  let player_level = dungeon_levels.get(*player_ent).unwrap();
  let map = dungeon.get_map(player_level.level).unwrap();

  for (entity, _player, pos, viewshed) in
    (&entities, &mut players, &mut positions, &mut viewsheds).join()
  {
    let x = pos.x + delta_x;
    let y = pos.y + delta_y;
    let destination_index = map.xy_idx(x, y);

    for potential_target in map.tile_content[destination_index as usize].iter() {
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
    if !map.blocked[destination_index as usize] {
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
  let levels = ecs.read_storage::<DungeonLevel>();
  let mut gamelog = ecs.fetch_mut::<GameLog>();
  let player_level = levels.get(*player_entity).unwrap();

  let target_item: Option<Entity> =
    (&entities, &items, &positions, &levels)
      .join()
      .find_map(|(ent, _item, position, level)| {
        if position.x == player_pos.x
          && position.y == player_pos.y
          && level.level == player_level.level
        {
          return Some(ent);
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

fn points_match(point_a: &Point, point_b: &Point) -> bool {
  point_a.x == point_b.x && point_a.y == point_b.y
}

fn can_go_up(current_map: &Map, player_point: &Point) -> bool {
  if let Some(stairs_up_coords) = current_map.stairs_up {
    return points_match(&stairs_up_coords, player_point);
  }
  return false;
}

fn can_go_down(current_map: &Map, player_point: &Point) -> bool {
  if let Some(stairs_down_coords) = current_map.stairs_down {
    return points_match(&stairs_down_coords, player_point);
  }
  return false;
}


fn try_go_down_stairs(ecs: &mut World) {
  let mut player_point = ecs.write_resource::<Point>();
  let mut levels = ecs.write_storage::<DungeonLevel>();
  let player_entity = ecs.fetch::<Entity>();
  let mut player_level = levels.get_mut(*player_entity).unwrap();
  let mut dungeon = ecs.fetch_mut::<Dungeon>();
  let current_map = dungeon.get_map(player_level.level).unwrap();
  if !can_go_down(current_map, &player_point) {
    return;
  }
  let next_level_number = player_level.level - 1;
  let next_map = dungeon.get_map(next_level_number);
  if let Some(next_map) = next_map {
    let stairs_up_coords = next_map.stairs_up.unwrap();
    let mut positions = ecs.write_storage::<Position>();
    let mut player_position = positions.get_mut(*player_entity).unwrap();
    player_position.x = stairs_up_coords.x;
    player_position.y = stairs_up_coords.y;
    player_level.level = next_level_number;
    player_point.x = stairs_up_coords.x;
    player_point.y = stairs_up_coords.y;
  }
}

fn try_go_up_stairs(ecs: &mut World) {
  let mut player_point = ecs.write_resource::<Point>();
  let mut levels = ecs.write_storage::<DungeonLevel>();
  let player_entity = ecs.fetch::<Entity>();
  let mut player_level = levels.get_mut(*player_entity).unwrap();
  let mut dungeon = ecs.fetch_mut::<Dungeon>();
  let current_map = dungeon.get_map(player_level.level).unwrap();
  if !can_go_up(current_map, &player_point) {
    return;
  }
  let next_level_number = player_level.level + 1;
  let next_map = dungeon.get_map(next_level_number);
  if let Some(next_map) = next_map {
    let stairs_down_coords = next_map.stairs_down.unwrap();
    let mut positions = ecs.write_storage::<Position>();
    let mut player_position = positions.get_mut(*player_entity).unwrap();
    player_position.x = stairs_down_coords.x;
    player_position.y = stairs_down_coords.y;
    player_level.level = next_level_number;
    player_point.x = stairs_down_coords.x;
    player_point.y = stairs_down_coords.y;
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
    MapAction::GoDownStairs => try_go_down_stairs(ecs),
    MapAction::GoUpStairs => try_go_up_stairs(ecs),
    _ => {}
  }
}
