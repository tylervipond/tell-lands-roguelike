use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug)]
pub struct InBackpack {
  pub owner: Entity,
}
