use specs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
pub struct InflictsDamage {
  pub amount: i32
}