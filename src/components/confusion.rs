use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Confusion {
  pub turns: i32,
}
