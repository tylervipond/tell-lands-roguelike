use rltk::Point;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Viewshed {
  pub visible_tiles: Vec<Point>,
  pub range: i32,
  pub dirty: bool,
}
