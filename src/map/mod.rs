use rltk::{Console, Rltk, RGB};
use specs::{World};

pub mod basic_map;
pub mod rect;
pub mod tile_type;

pub use crate::components::{player::Player, viewshed::Viewshed};

pub use basic_map::Map;
pub use tile_type::TileType;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 43;
pub const MAP_COUNT: usize = MAP_HEIGHT * MAP_WIDTH;


pub fn xy_idx(x: i32, y: i32) -> usize {
  (y * MAP_WIDTH as i32 + x) as usize
}

pub fn idx_xy(idx: i32) -> (i32, i32) {
  (idx % MAP_WIDTH as i32, idx / MAP_WIDTH as i32)
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
  let map = ecs.fetch::<Map>();

  for (i, tile) in map.tiles.iter().enumerate() {
    if map.revealed_tiles[i] {
      let x = i % MAP_WIDTH as usize;
      let y = (i - (i % MAP_WIDTH as usize)) / MAP_WIDTH as usize;
      let character = match tile {
        TileType::Floor => '.',
        TileType::Wall => '#',
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
        rltk::to_cp437(character),
      )
    }
  }
}
