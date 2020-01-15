use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug, Clone)]
pub struct WantsToDropItem {
  pub item: Entity
}