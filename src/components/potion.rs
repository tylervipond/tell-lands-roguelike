use specs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
pub struct Potion {
  pub heal_amount: i32,
}