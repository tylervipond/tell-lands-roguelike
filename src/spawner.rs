use crate::components::{
  blocks_tile::BlocksTile, combat_stats::CombatStats, item::Item, monster::Monster, name::Name,
  player::Player, position::Position, potion::Potion, renderable::Renderable, viewshed::Viewshed, 
};
use crate::map::{idx_xy, rect::Rect, xy_idx};
use rltk::{to_cp437, RandomNumberGenerator, RGB};
use specs::{Builder, Entity, World, WorldExt};

pub const MAX_MONSTERS_PER_ROOM: i32 = 4;
pub const MAX_ITEMS_PER_ROOM: i32 = 2;

pub fn spawn_player(ecs: &mut World, x: i32, y: i32) -> Entity {
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(Renderable {
      glyph: to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::BLACK),
    })
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
    .build()
}

pub fn spawn_monster<S: ToString>(ecs: &mut World, idx: i32, glyph: u8, name: S) -> Entity {
  let (x, y) = idx_xy(idx);
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(Renderable {
      glyph,
      fg: RGB::named(rltk::RED),
      bg: RGB::named(rltk::BLACK),
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

pub fn spawn_orc(ecs: &mut World, idx: i32) -> Entity {
  spawn_monster(ecs, idx, to_cp437('o'), "Orc")
}

pub fn spawn_goblin(ecs: &mut World, idx: i32) -> Entity {
  spawn_monster(ecs, idx, to_cp437('g'), "Goblin")
}

pub fn spawn_random_monster(ecs: &mut World, idx: i32) -> Entity {
  let roll = {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    rng.roll_dice(1, 2)
  };
  match roll {
    1 => spawn_orc(ecs, idx),
    _ => spawn_goblin(ecs, idx),
  }
}

pub fn spawn_health_potion(ecs: &mut World, idx: i32) -> Entity {
  let (x, y) = idx_xy(idx);
  ecs
    .create_entity()
    .with(Position { x, y })
    .with(Name {
      name: "Health Potion".to_string(),
    })
    .with(Renderable {
      glyph: to_cp437('i'),
      fg: RGB::named(rltk::LIGHT_BLUE),
      bg: RGB::named(rltk::BLACK),
    })
    .with(Item {})
    .with(Potion { heal_amount: 8 })
    .build()
}

pub fn spawn_monster_entities_for_room(ecs: &mut World, rect: &Rect) {
  let mut monster_spawn_points: Vec<usize> = vec![];
  {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_monsters = rng.roll_dice(1, MAX_MONSTERS_PER_ROOM + 2) - 3;
    for _i in 0..num_monsters {
      let mut added = false;
      while !added {
        let (x, y) = rect.get_random_coord(&mut rng);
        let idx = xy_idx(x, y);
        if !monster_spawn_points.contains(&idx) {
          monster_spawn_points.push(idx);
          added = true;
        }
      }
    }
  }
  for idx in monster_spawn_points.iter() {
    spawn_random_monster(ecs, (*idx) as i32);
  }
}

pub fn spawn_item_entities_for_room(ecs: &mut World, rect: &Rect) {
  let mut item_spawn_points: Vec<usize> = vec![];
  {
    let mut rng = ecs.write_resource::<RandomNumberGenerator>();
    let num_items = rng.roll_dice(1, MAX_ITEMS_PER_ROOM + 2) -3;
    for _i in 0..num_items {
      let mut added = false;
      while !added {
      let (x, y) = rect.get_random_coord(&mut rng);
        let idx = xy_idx(x, y);
        if !item_spawn_points.contains(&idx) {
          item_spawn_points.push(idx);
          added = true;
        }
      }
    }
  }
  for idx in item_spawn_points.iter() {
    spawn_health_potion(ecs, (*idx) as i32);
  }
}

pub fn spawn_entities_for_room(ecs: &mut World, rect: &Rect) {
  spawn_monster_entities_for_room(ecs, rect);
  spawn_item_entities_for_room(ecs, rect);
}
