use specs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
pub struct ProvidesHealing {
  pub amount: i32,
}
