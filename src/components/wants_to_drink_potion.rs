use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug)]
pub struct WantsToDrinkPotion {
  pub potion: Entity,
}
