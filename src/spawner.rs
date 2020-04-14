use crate::components::{
  area_of_effect::AreaOfEffect, blocks_tile::BlocksTile, combat_stats::CombatStats,
  confusion::Confusion, consumable::Consumable, dungeon_level::DungeonLevel,
  entry_trigger::EntryTrigger, hidden::Hidden, inflicts_damage::InflictsDamage, item::Item,
  miscellaneous::Miscellaneous, monster::Monster, name::Name, objective::Objective, player::Player,
  position::Position, provides_healing::ProvidesHealing, ranged::Ranged, renderable::Renderable,
  saveable::Saveable, single_activation::SingleActivation, viewshed::Viewshed,
};
use crate::dungeon::{
  level::Level,
  operations::{idx_xy, xy_idx},
  rect::Rect,
  room::Room,
  room_type::RoomType,
  tile_type::TileType,
};
use rltk::{to_cp437, RandomNumberGenerator, RGB};
use specs::{
  saveload::{MarkedBuilder, SimpleMarker},
  Builder, Entity, EntityBuilder, Join, World, WorldExt,
};

pub const MAX_MONSTERS_PER_ROOM: i32 = 2;
pub const MAX_ITEMS_PER_ROOM: i32 = 2;
pub const MAX_MISC_PER_ROOM: i32 = 10;

pub fn get_cardinal_idx(idx: i32, level: &Level) -> (i32, i32, i32, i32) {
  let (x, y) = idx_xy(level, idx);
  let n = xy_idx(level, x, y - 1);
  let e = xy_idx(level, x + 1, y);
  let s = xy_idx(level, x, y + 1);
  let w = xy_idx(level, x - 1, y);

  return (n, e, s, w);
}

pub fn get_ordinal_idx(idx: i32, level: &Level) -> (i32, i32, i32, i32) {
  let (x, y) = idx_xy(level, idx);
  let ne = xy_idx(level, x + 1, y - 1);
  let se = xy_idx(level, x + 1, y + 1);
  let sw = xy_idx(level, x - 1, y + 1);
  let nw = xy_idx(level, x - 1, y - 1);

  return (ne, se, sw, nw);
}

pub fn path_is_blocked(ecs: &World, path_idx: (i32, i32, i32), level: &Level) -> bool {
  let (tile_1, tile_2, tile_3) = path_idx;
  if tile_is_blocked(ecs, tile_1, level)
    || tile_is_blocked(ecs, tile_2, level)
    || tile_is_blocked(ecs, tile_3, level)
  {
    return true;
  }
  return false;
}

pub fn tile_can_be_blocked(ecs: &World, idx: i32, level: &Level) -> bool {
  let (n, e, s, w) = get_cardinal_idx(idx, level);
  let (ne, se, sw, nw) = get_ordinal_idx(idx, level);

  if !tile_is_blocked(ecs, w, level) && !tile_is_blocked(ecs, e, level) {
    if path_is_blocked(ecs, (nw, n, ne), level) || path_is_blocked(ecs, (sw, s, se), level) {
      return false;
    }
  }

  if !tile_is_blocked(ecs, n, level) && !tile_is_blocked(ecs, s, level) {
    if path_is_blocked(ecs, (nw, w, sw), level) || path_is_blocked(ecs, (ne, e, se), level) {
      return false;
    }
  }

  return true;
}

pub fn tile_is_blocked(ecs: &World, idx: i32, level: &Level) -> bool {
  if level.tiles[idx as usize] == TileType::Wall {
    return true;
  }

  let blocks_tile = ecs.read_storage::<BlocksTile>();
  let positions = ecs.read_storage::<Position>();
  let entities = ecs.entities();

  for (_entity, _blocks_tile, pos) in (&entities, &blocks_tile, &positions).join() {
    if xy_idx(level, pos.x, pos.y) == idx {
      return true;
    }
  }

  return false;
}

fn created_marked_entity_with_position<'a>(
  ecs: &'a mut World,
  map_idx: i32,
  level: &'a Level,
) -> EntityBuilder<'a> {
  let (x, y) = idx_xy(level, map_idx);
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(DungeonLevel { level: level.depth })
    .marked::<SimpleMarker<Saveable>>()
}

pub fn spawn_player(ecs: &mut World, x: i32, y: i32, level: i32) -> Entity {
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(Renderable {
      glyph: to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
      layer: 0,
    })
    .with(DungeonLevel { level })
    .with(Player {})
    .with(Viewshed {
      range: 8,
      visible_tiles: vec![],
      dirty: true,
    })
    .with(Name {
      name: "Player".to_owned(),
    })
    .with(CombatStats {
      max_hp: 30,
      hp: 30,
      power: 5,
      defense: 2,
    })
    .marked::<SimpleMarker<Saveable>>()
    .build()
}

pub fn spawn_monster<S: ToString>(
  ecs: &mut World,
  idx: i32,
  glyph: u16,
  name: S,
  level: &Level,
) -> Entity {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Renderable {
      glyph,
      fg: RGB::named(rltk::RED),
      bg: RGB::named(rltk::BLACK),
      layer: 0,
    })
    .with(Viewshed {
      visible_tiles: vec![],
      range: 8,
      dirty: true,
    })
    .with(Monster {})
    .with(Name {
      name: name.to_string(),
    })
    .with(BlocksTile {})
    .with(CombatStats {
      max_hp: 16,
      hp: 16,
      defense: 1,
      power: 4,
    })
    .build()
}

fn spawn_objective(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "The Talisman".to_string(),
    })
    .with(Renderable {
      glyph: 241,
      fg: RGB::named(rltk::LIGHT_SALMON),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Item {})
    .with(Objective {})
    .build()
}

pub fn spawn_orc(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  spawn_monster(ecs, idx, to_cp437('o'), "Orc", level)
}

pub fn spawn_goblin(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  spawn_monster(ecs, idx, to_cp437('g'), "Goblin", level)
}

pub fn spawn_random_monster(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  let roll = {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    rng.roll_dice(1, 2)
  };
  match roll {
    1 => spawn_orc(ecs, idx, level),
    _ => spawn_goblin(ecs, idx, level),
  }
}

pub fn spawn_health_potion(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Health Potion".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('i'),
      fg: RGB::named(rltk::LIGHT_BLUE),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Item {})
    .with(Consumable {})
    .with(ProvidesHealing { amount: 8 })
    .build()
}

pub fn spawn_magic_missile_scroll(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Scroll of Magic Missile".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437(')'),
      fg: RGB::named(rltk::CYAN),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Item {})
    .with(Consumable {})
    .with(Ranged { range: 6 })
    .with(InflictsDamage { amount: 8 })
    .build()
}

pub fn spawn_fireball_scroll(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Scroll of Fireball".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437(')'),
      fg: RGB::named(rltk::ORANGE),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Item {})
    .with(Consumable {})
    .with(Ranged { range: 6 })
    .with(InflictsDamage { amount: 20 })
    .with(AreaOfEffect { radius: 3 })
    .build()
}

pub fn spawn_confusion_scroll(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Scroll of Confusion".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437(')'),
      fg: RGB::named(rltk::PINK),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Item {})
    .with(Consumable {})
    .with(Ranged { range: 6 })
    .with(Confusion { turns: 4 })
    .build()
}

pub fn spawn_bear_trap(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Bear Trap".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('^'),
      fg: RGB::named(rltk::RED),
      bg: RGB::named(rltk::BLACK),
      layer: 2,
    })
    .with(Hidden {})
    .with(EntryTrigger {})
    .with(InflictsDamage { amount: 6 })
    .with(SingleActivation {})
    .build()
}

fn get_spawn_point(rect: &Rect, level: &Level, rng: &mut RandomNumberGenerator) -> u16 {
  let idx1 = xy_idx(&level, rect.x1, rect.y1);
  let idx2 = xy_idx(&level, rect.x2, rect.y2);
  let floor_tiles_in_rect: Vec<i32> = (idx1..idx2)
    .filter(|idx| level.tiles[*idx as usize] == TileType::Floor)
    .collect();
  // this could throw if we somehow end up with a zero length vec for floor tiles,
  // that would mean that our level generation has a problem.
  let selected_index = rng.range(0, floor_tiles_in_rect.len());
  floor_tiles_in_rect[selected_index] as u16
}

fn get_spawn_points(
  rect: &Rect,
  level: &Level,
  rng: &mut RandomNumberGenerator,
  count: i32,
) -> Vec<u16> {
  (0..count)
    .map(|_| get_spawn_point(rect, level, rng))
    .collect()
}

pub fn spawn_monster_entities_for_room(ecs: &mut World, room: &Room, level: &Level) {
  let spawn_points = {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_monsters = rng.range(0, MAX_MONSTERS_PER_ROOM);
    get_spawn_points(&room.rect, level, &mut rng, num_monsters)
  };
  for idx in spawn_points.iter() {
    spawn_random_monster(ecs, (*idx) as i32, level);
  }
}

fn spawn_random_item(ecs: &mut World, idx: i32, level: &Level) {
  let roll = {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    rng.roll_dice(1, 6)
  };
  match roll {
    1 | 2 => {
      spawn_health_potion(ecs, idx, level);
    }
    3 => {
      spawn_fireball_scroll(ecs, idx, level);
    }
    4 => {
      spawn_confusion_scroll(ecs, idx, level);
    }
    5 => {
      spawn_bear_trap(ecs, idx, level);
    }
    _ => {
      spawn_magic_missile_scroll(ecs, idx, level);
    }
  }
}

pub fn spawn_item_entities_for_room(ecs: &mut World, room: &Room, level: &Level) {
  let spawn_points = {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_items = rng.roll_dice(1, MAX_ITEMS_PER_ROOM + 2) - 3;
    get_spawn_points(&room.rect, level, &mut rng, num_items)
  };
  for idx in spawn_points.iter() {
    spawn_random_item(ecs, (*idx) as i32, level);
  }
}

pub fn spawn_barrel(ecs: &mut World, idx: i32, level: &Level) {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Barrel".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('B'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(BlocksTile {})
    .with(Miscellaneous {})
    .build();
}

pub fn spawn_treasure_chest(ecs: &mut World, idx: i32, level: &Level) {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Trasure Chest".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('T'),
      fg: RGB::named(rltk::BROWN1),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(BlocksTile {})
    .with(Miscellaneous {})
    .build();
}

pub fn spawn_debris(ecs: &mut World, idx: i32, level: &Level) {
  created_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Debris".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('x'),
      fg: RGB::named(rltk::GREY),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(BlocksTile {})
    .with(Miscellaneous {})
    .build();
}

pub fn spawn_miscellaneous_entities_for_room(ecs: &mut World, room: &Room, level: &Level) {
  let mut miscellaneous_spawn_points: Vec<usize> = vec![];
  {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_miscellaneous = rng.roll_dice(1, MAX_MISC_PER_ROOM + 2) - 3;
    for _i in 0..num_miscellaneous {
      let mut added = false;
      while !added {
        let (x, y) = match room.room_type {
          RoomType::TreasureRoom => room.rect.get_random_wall_adjacent_coord(&mut rng),
          RoomType::StoreRoom => room.rect.get_random_wall_adjacent_coord(&mut rng),
          RoomType::Collapsed => room.rect.get_random_coord(&mut rng),
          RoomType::Empty => room.rect.get_random_coord(&mut rng),
        };
        let idx = xy_idx(&level, x, y) as usize;
        if !miscellaneous_spawn_points.contains(&idx) {
          miscellaneous_spawn_points.push(idx);
          added = true;
        }
      }
    }
  }
  for idx in miscellaneous_spawn_points.iter() {
    if !tile_is_blocked(ecs, (*idx) as i32, level) && tile_can_be_blocked(ecs, (*idx) as i32, level)
    {
      match room.room_type {
        RoomType::TreasureRoom => spawn_treasure_chest(ecs, (*idx) as i32, level),
        RoomType::StoreRoom => spawn_barrel(ecs, (*idx) as i32, level),
        RoomType::Collapsed => spawn_debris(ecs, (*idx) as i32, level),
        RoomType::Empty => {}
      };
    }
  }
}

pub fn spawn_entities_for_room(ecs: &mut World, room: &Room, level: &Level) {
  // Miscellaneous must spawn before monsters/items are placed
  spawn_miscellaneous_entities_for_room(ecs, room, level);
  spawn_monster_entities_for_room(ecs, room, level);
  spawn_item_entities_for_room(ecs, room, level);
}

pub fn spawn_objective_for_room(ecs: &mut World, rect: &Rect, level: &Level) {
  let idx = {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    get_spawn_point(rect, level, &mut rng)
  };
  spawn_objective(ecs, idx as i32, level);
}
