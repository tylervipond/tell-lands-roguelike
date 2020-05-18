use crate::components::{
  AreaOfEffect, BlocksTile, CausesFire, CombatStats, Confusion, Consumable, Contained, Container,
  DungeonLevel, EntryTrigger, Flammable, Hidden, InflictsDamage, Item, Monster, Name, Objective,
  Player, Position, ProvidesHealing, Ranged, Renderable, Saveable, SingleActivation, Trap,
  Viewshed,
};
use crate::dungeon::{
  level::Level, level_utils, rect::Rect, room::Room, room_type::RoomType, tile_type::TileType,
};
use crate::entity_set::EntitySet;
use crate::types::{trap_type, TrapType};
use rltk::{to_cp437, RandomNumberGenerator, RGB};
use specs::{
  saveload::{MarkedBuilder, SimpleMarker},
  Builder, Entity, EntityBuilder, Join, World, WorldExt,
};
use std::cmp;

pub const MAX_MONSTERS_PER_ROOM: i32 = 2;
pub const MAX_ITEMS_PER_ROOM: i32 = 2;
pub const MAX_MISC_PER_ROOM: i32 = 10;
pub const MAX_TRAPS_SET_PER_LEVEL: i32 = 10;

fn path_is_blocked(path_idx: (i32, i32, i32), level: &Level) -> bool {
  let (tile_1, tile_2, tile_3) = path_idx;
  if level_utils::tile_is_blocked(tile_1, level)
    || level_utils::tile_is_blocked(tile_2, level)
    || level_utils::tile_is_blocked(tile_3, level)
  {
    return true;
  }
  false
}

pub fn tile_can_be_blocked(idx: i32, level: &Level) -> bool {
  let (n, e, s, w) = level_utils::get_cardinal_idx(idx, level);
  let (ne, se, sw, nw) = level_utils::get_ordinal_idx(idx, level);

  if !level_utils::tile_is_blocked(w, level) && !level_utils::tile_is_blocked(e, level) {
    if path_is_blocked((nw, n, ne), level) || path_is_blocked((sw, s, se), level) {
      return false;
    }
  }

  if !level_utils::tile_is_blocked(n, level) && !level_utils::tile_is_blocked(s, level) {
    if path_is_blocked((nw, w, sw), level) || path_is_blocked((ne, e, se), level) {
      return false;
    }
  }

  true
}

fn create_marked_entity_with_position<'a>(
  ecs: &'a mut World,
  map_idx: i32,
  level: &'a Level,
) -> EntityBuilder<'a> {
  let (x, y) = level_utils::idx_xy(level, map_idx);
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(DungeonLevel { level: level.depth })
    .marked::<SimpleMarker<Saveable>>()
}

fn create_marked_entity_in_container<'a>(
  ecs: &'a mut World,
  container_entity: Entity,
) -> EntityBuilder<'a> {
  ecs
    .create_entity()
    .with(Contained {
      container: container_entity,
    })
    .marked::<SimpleMarker<Saveable>>()
}

pub fn spawn_player(ecs: &mut World, x: i32, y: i32, level: u8) -> Entity {
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
  create_marked_entity_with_position(ecs, idx, level)
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
  create_marked_entity_with_position(ecs, idx, level)
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

fn make_entity_health_potion<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
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
}

pub fn spawn_health_potion(world: &mut World, idx: i32, level: &Level) -> Entity {
  make_entity_health_potion(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_health_potion_in_container(world: &mut World, container_entity: Entity) -> Entity {
  make_entity_health_potion(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_magic_missile_scroll<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
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
}

fn spawn_magic_missile_scroll(world: &mut World, idx: i32, level: &Level) -> Entity {
  make_entity_magic_missile_scroll(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_magic_missile_scroll_in_container(world: &mut World, container_entity: Entity) -> Entity {
  make_entity_magic_missile_scroll(create_marked_entity_in_container(world, container_entity))
    .build()
}

fn make_entity_fireball_scroll<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
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
    .with(CausesFire {})
    .with(AreaOfEffect { radius: 3 })
}

fn spawn_fireball_scroll(world: &mut World, idx: i32, level: &Level) -> Entity {
  make_entity_fireball_scroll(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_fireball_scroll_in_container(world: &mut World, container_entity: Entity) -> Entity {
  make_entity_fireball_scroll(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_confusion_scroll<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
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
}

fn spawn_confusion_scroll(world: &mut World, idx: i32, level: &Level) -> Entity {
  make_entity_confusion_scroll(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_confusion_scroll_in_container(world: &mut World, container_entity: Entity) -> Entity {
  make_entity_confusion_scroll(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_bear_trap<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
    .with(Name {
      name: "Bear Trap".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('^'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Item {})
    .with(Consumable {})
    .with(Ranged { range: 1 })
    .with(Trap {
      trap_type: TrapType::BearTrap,
    })
}

fn spawn_bear_trap(world: &mut World, idx: i32, level: &Level) -> Entity {
  make_entity_bear_trap(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_bear_trap_in_container(world: &mut World, container_entity: Entity) -> Entity {
  make_entity_bear_trap(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_caltrops<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
  builder
    .with(Name {
      name: "Caltrops".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('%'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Item {})
    .with(Consumable {})
    .with(Ranged { range: 3 })
    .with(Trap {
      trap_type: TrapType::Caltrops,
    })
}

fn spawn_caltrops(world: &mut World, idx: i32, level: &Level) -> Entity {
  make_entity_caltrops(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_caltrops_in_container(world: &mut World, container_entity: Entity) -> Entity {
  make_entity_caltrops(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_set_trap<'a>(
  builder: EntityBuilder<'a>,
  type_of_trap: &TrapType,
) -> EntityBuilder<'a> {
  builder
    .with(Name {
      name: trap_type::get_name_for_trap(type_of_trap),
    })
    .with(Renderable {
      glyph: trap_type::get_glyph_for_trap(type_of_trap),
      fg: RGB::named(rltk::RED),
      bg: RGB::named(rltk::BLACK),
      layer: 2,
    })
    .with(Hidden {
      found_by: EntitySet::new(),
    })
    .with(EntryTrigger {})
    .with(InflictsDamage {
      amount: trap_type::get_damage_for_trap(type_of_trap),
    })
}

fn spawn_set_bear_trap(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  make_entity_set_trap(
    create_marked_entity_with_position(ecs, idx, level),
    &TrapType::BearTrap,
  )
  .with(SingleActivation {})
  .build()
}

fn spawn_set_caltrops(ecs: &mut World, idx: i32, level: &Level) -> Entity {
  make_entity_set_trap(
    create_marked_entity_with_position(ecs, idx, level),
    &TrapType::Caltrops,
  )
  .build()
}

fn spawn_monster_entities_for_room(ecs: &mut World, room: &Room, level: &mut Level) {
  let spawn_points = {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_monsters = rng.range(0, MAX_MONSTERS_PER_ROOM);
    level_utils::get_spawn_points(&room.rect, level, &mut rng, num_monsters)
  };
  for idx in spawn_points.iter() {
    spawn_random_monster(ecs, (*idx) as i32, level);
    level.blocked[*idx as usize] = true;
  }
}

fn spawn_set_traps(ecs: &mut World, idx: i32, level: &Level) {
  let roll = get_random_in_range(ecs, 1, 2);
  match roll {
    1 => spawn_set_bear_trap(ecs, idx, level),
    _ => spawn_set_caltrops(ecs, idx, level),
  };
}

fn spawn_random_item(world: &mut World, idx: i32, level: &Level) {
  let roll = get_random_in_range(world, 1, 7);
  match roll {
    1 | 2 => spawn_health_potion(world, idx, level),
    3 => spawn_fireball_scroll(world, idx, level),
    4 => spawn_confusion_scroll(world, idx, level),
    5 => spawn_bear_trap(world, idx, level),
    6 => spawn_caltrops(world, idx, level),
    _ => spawn_magic_missile_scroll(world, idx, level),
  };
}

fn spawn_random_item_in_container(world: &mut World, container_entity: Entity) {
  let roll = get_random_in_range(world, 1, 7);
  match roll {
    1 | 2 => spawn_health_potion_in_container(world, container_entity),
    3 => spawn_fireball_scroll_in_container(world, container_entity),
    4 => spawn_confusion_scroll_in_container(world, container_entity),
    5 => spawn_bear_trap_in_container(world, container_entity),
    6 => spawn_caltrops_in_container(world, container_entity),
    _ => spawn_magic_missile_scroll_in_container(world, container_entity),
  };
}

fn get_containers_in_room(ecs: &World, room: &Room) -> Vec<Entity> {
  let containers = ecs.read_storage::<Container>();
  let positions = ecs.read_storage::<Position>();
  let entities = ecs.entities();
  (&containers, &positions, &entities)
    .join()
    .filter(|(_c, p, _e)| room.rect.contains(p.x, p.y))
    .map(|(_c, _p, e)| e)
    .collect()
}

fn get_random_in_range(ecs: &mut World, min: i32, max: i32) -> i32 {
  let mut rng = ecs.write_resource::<RandomNumberGenerator>();
  rng.range(min, max + 1)
}

pub fn spawn_item_entities_for_room(ecs: &mut World, room: &Room, level: &Level) {
  let containers_in_room = get_containers_in_room(ecs, room);
  let min_items = match room.room_type {
    Some(RoomType::TreasureRoom) => 2,
    _ => 0,
  };
  let num_items = get_random_in_range(ecs, min_items, MAX_ITEMS_PER_ROOM + 2) - 3;
  if num_items > 0 {
    // more than half of the items should be in containers if there are any.
    let min_items_in_containers = num_items as f32 * 0.6;
    let num_items_in_containers = cmp::min(
      get_random_in_range(ecs, min_items_in_containers.ceil() as i32, num_items),
      containers_in_room.len() as i32,
    );
    let num_items_not_in_containers = num_items - num_items_in_containers;
    let spawn_points = {
      let mut rng = ecs.write_resource::<RandomNumberGenerator>();
      level_utils::get_spawn_points(&room.rect, level, &mut rng, num_items_not_in_containers)
    };
    for idx in spawn_points.iter() {
      spawn_random_item(ecs, (*idx) as i32, level);
    }
    for _ in 0..num_items_in_containers {
      let container_idx = get_random_in_range(ecs, 0, (containers_in_room.len() - 1) as i32);
      spawn_random_item_in_container(ecs, containers_in_room[container_idx as usize]);
    }
  }
}

pub fn spawn_barrel(ecs: &mut World, idx: i32, level: &Level) {
  create_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Barrel".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('B'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Flammable { turns_remaining: 8 })
    .with(BlocksTile {})
    .build();
}

pub fn spawn_treasure_chest(ecs: &mut World, idx: i32, level: &Level) {
  create_marked_entity_with_position(ecs, idx, level)
    .with(Name {
      name: "Trasure Chest".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('T'),
      fg: RGB::named(rltk::BROWN3),
      bg: RGB::named(rltk::BLACK),
      layer: 1,
    })
    .with(Container {})
    .with(Flammable { turns_remaining: 8 })
    .with(BlocksTile {})
    .build();
}

pub fn spawn_debris(ecs: &mut World, idx: i32, level: &Level) {
  create_marked_entity_with_position(ecs, idx, level)
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
    .build();
}

fn spawn_miscellaneous_entities_for_room(ecs: &mut World, room: &Room, level: &mut Level) {
  if let Some(room_type) = room.room_type {
    let spawn_points = {
      let mut rng = ecs.write_resource::<RandomNumberGenerator>();
      let num_miscellaneous = rng.roll_dice(1, MAX_MISC_PER_ROOM + 2) - 3;
      match room_type {
        RoomType::Collapsed => {
          level_utils::get_spawn_points(&room.rect, level, &mut rng, num_miscellaneous)
        }
        _ => level_utils::get_wall_adjacent_spawn_points(
          &room.rect,
          level,
          &mut rng,
          num_miscellaneous,
        ),
      }
    };
    for idx in spawn_points.iter() {
      if !level_utils::tile_is_blocked((*idx) as i32, level)
        && tile_can_be_blocked((*idx) as i32, level)
      {
        match room_type {
          RoomType::Collapsed => spawn_debris(ecs, (*idx) as i32, level),
          RoomType::StoreRoom => spawn_barrel(ecs, (*idx) as i32, level),
          RoomType::TreasureRoom => spawn_treasure_chest(ecs, (*idx) as i32, level),
        }
      }
      level.blocked[*idx as usize] = true;
    }
  }
}

pub fn spawn_entities_for_room(ecs: &mut World, room: &Room, level: &mut Level) {
  // Miscellaneous must spawn before monsters/items are placed
  spawn_miscellaneous_entities_for_room(ecs, room, level);
  spawn_monster_entities_for_room(ecs, room, level);
  spawn_item_entities_for_room(ecs, room, level);
}

fn spawn_set_traps_for_level(world: &mut World, level: &mut Level) {
  let num_set_traps = {
    let mut rng = world.write_resource::<RandomNumberGenerator>();
    rng.range(4, MAX_TRAPS_SET_PER_LEVEL)
  };
  let mut possible_spawn_points: Vec<usize> = level
    .tiles
    .iter()
    .enumerate()
    .filter(|(idx, tile)| {
      **tile == TileType::Floor && !level_utils::tile_is_blocked(*idx as i32, level)
    })
    .map(|(idx, _)| idx)
    .collect();
  for _ in 0..num_set_traps {
    let idx = {
      let mut rng = world.write_resource::<RandomNumberGenerator>();
      rng.range(0, possible_spawn_points.len())
    };
    let spawn_point = possible_spawn_points.remove(idx);
    spawn_set_traps(world, spawn_point as i32, level);
  }
}

pub fn spawn_entities_for_level(world: &mut World, level: &mut Level) {
  let count = level.rooms.len();
  for i in (0..count).skip(1) {
    let room = level.rooms[i].clone();
    spawn_entities_for_room(world, &room, level);
  }
  spawn_set_traps_for_level(world, level);
}

pub fn spawn_objective_for_room(ecs: &mut World, rect: &Rect, level: &Level) {
  let idx = {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    level_utils::get_random_spawn_point(rect, level, &mut rng)
  };
  spawn_objective(ecs, idx as i32, level);
}
