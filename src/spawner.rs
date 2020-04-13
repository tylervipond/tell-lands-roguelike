use crate::components::{
  area_of_effect::AreaOfEffect, blocks_tile::BlocksTile, combat_stats::CombatStats,
  confusion::Confusion, consumable::Consumable, dungeon_level::DungeonLevel,
  entry_trigger::EntryTrigger, hidden::Hidden, inflicts_damage::InflictsDamage, item::Item,
  monster::Monster, name::Name, objective::Objective, player::Player, position::Position,
  provides_healing::ProvidesHealing, ranged::Ranged, renderable::Renderable, saveable::Saveable,
  single_activation::SingleActivation, viewshed::Viewshed,
};
use crate::dungeon::{
  level::Level,
  operations::{idx_xy, xy_idx},
  rect::Rect,
};
use rltk::{to_cp437, RandomNumberGenerator, RGB};
use specs::{
  saveload::{MarkedBuilder, SimpleMarker},
  Builder, Entity, EntityBuilder, World, WorldExt,
};

pub const MAX_MONSTERS_PER_ROOM: i32 = 2;
pub const MAX_ITEMS_PER_ROOM: i32 = 2;

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

pub fn spawn_objective(ecs: &mut World, idx: i32, level: &Level) -> Entity {
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

pub fn spawn_monster_entities_for_room(ecs: &mut World, rect: &Rect, level: &Level) {
  let mut monster_spawn_points: Vec<usize> = vec![];
  {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_monsters = rng.range(0, MAX_MONSTERS_PER_ROOM);
    for _i in 0..num_monsters {
      let mut added = false;
      while !added {
        let (x, y) = rect.get_random_coord(&mut rng);
        let idx = xy_idx(&level, x, y) as usize;
        if !monster_spawn_points.contains(&idx) {
          monster_spawn_points.push(idx);
          added = true;
        }
      }
    }
  }
  for idx in monster_spawn_points.iter() {
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

pub fn spawn_item_entities_for_room(ecs: &mut World, rect: &Rect, level: &Level) {
  let mut item_spawn_points: Vec<usize> = vec![];
  {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_items = rng.roll_dice(1, MAX_ITEMS_PER_ROOM + 2) - 3;
    for _i in 0..num_items {
      let mut added = false;
      while !added {
        let (x, y) = rect.get_random_coord(&mut rng);
        let idx = xy_idx(&level, x, y) as usize;
        if !item_spawn_points.contains(&idx) {
          item_spawn_points.push(idx);
          added = true;
        }
      }
    }
  }
  for idx in item_spawn_points.iter() {
    spawn_random_item(ecs, (*idx) as i32, level);
  }
}

pub fn spawn_entities_for_room(ecs: &mut World, rect: &Rect, level: &Level) {
  spawn_monster_entities_for_room(ecs, rect, level);
  spawn_item_entities_for_room(ecs, rect, level);
}
