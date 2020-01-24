use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug)]
pub struct WantsToUse {
  pub item: Entity,
}
