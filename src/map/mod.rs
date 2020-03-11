use rltk::{Console, Rltk, RGB};
use specs::{Entity, World, WorldExt};

pub mod basic_map;
pub mod rect;
pub mod tile_type;

pub use crate::components::{dungeon_level::DungeonLevel, player::Player, viewshed::Viewshed};

pub use crate::dungeon::dungeon::Dungeon;
pub use basic_map::Map;
pub use tile_type::TileType;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 43;
pub const MAP_COUNT: usize = MAP_HEIGHT * MAP_WIDTH;

pub fn xy_idx(x: i32, y: i32) -> i32 {
  (y * MAP_WIDTH as i32 + x)
}

pub fn idx_xy(idx: i32) -> (i32, i32) {
  (idx % MAP_WIDTH as i32, idx / MAP_WIDTH as i32)
}

pub fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
  let map_idx = map.xy_idx(x, y) as usize;
  return map.tiles[map_idx] == TileType::Wall && map.revealed_tiles[map_idx];
}

pub fn get_wall_tile(map: &Map, x: i32, y: i32) -> u8 {
  if x < 1 || x > map.width - 2 || y < 1 || y > map.height - 2 {
    return 35;
  }

  let mut mask: u8 = 0;
  if is_revealed_and_wall(map, x, y - 1) {
    mask += 1;
  }
  if is_revealed_and_wall(map, x, y + 1) {
    mask += 2;
  }
  if is_revealed_and_wall(map, x - 1, y) {
    mask += 4;
  }
  if is_revealed_and_wall(map, x + 1, y) {
    mask += 8;
  }
  match mask {
    0 => 9,
    1 | 2 | 3 => 186,
    4 | 8 | 12 => 205,
    5 => 188,
    6 => 187,
    7 => 185,
    9 => 200,
    10 => 201,
    11 => 204,
    13 => 202,
    14 => 203,
    15 => 206,
    _ => 35,
  }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
  let mut dungeon = ecs.fetch_mut::<Dungeon>();
  let player_ent = ecs.fetch::<Entity>();
  let levels = ecs.read_storage::<DungeonLevel>();
  let player_level = levels.get(*player_ent).unwrap();
  let map = dungeon.get_map(player_level.level).unwrap();
  for (i, tile) in map.tiles.iter().enumerate() {
    if map.revealed_tiles[i] {
      let x = i % MAP_WIDTH as usize;
      let y = (i - (i % MAP_WIDTH as usize)) / MAP_WIDTH as usize;
      let character = match tile {
        TileType::Floor => rltk::to_cp437('.'),
        TileType::Wall => get_wall_tile(map, x as i32, y as i32),
        TileType::DownStairs => rltk::to_cp437('>'),
        TileType::UpStairs => rltk::to_cp437('<'),
        TileType::Exit => 219,
      };
      let foreground_color = match map.visible_tiles[i] {
        true => rltk::GREEN,
        false => rltk::WHITE,
      };
      ctx.set(
        x as i32,
        y as i32,
        RGB::named(foreground_color),
        RGB::named(rltk::BLACK),
        character,
      )
    }
  }
}
