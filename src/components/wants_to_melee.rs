use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug)]
pub struct WantsToMelee {
  pub target: Entity,
}
