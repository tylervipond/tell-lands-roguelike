pub mod basic_map;
pub mod rect;
pub mod tile_type;

pub use basic_map::Map;
pub use tile_type::TileType;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 43;
pub const MAP_COUNT: usize = MAP_HEIGHT * MAP_WIDTH;

pub fn xy_idx(x: i32, y: i32) -> i32 {
  y * MAP_WIDTH as i32 + x
}

pub fn idx_xy(idx: i32) -> (i32, i32) {
  (idx % MAP_WIDTH as i32, idx / MAP_WIDTH as i32)
}
