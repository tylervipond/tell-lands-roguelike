use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Confused {
  pub turns: i32,
}
