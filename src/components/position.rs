use specs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}
