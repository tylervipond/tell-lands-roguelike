use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug)]
pub struct WantsToPickUpItem {
  pub collected_by: Entity,
  pub item: Entity,
}
