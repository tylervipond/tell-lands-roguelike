use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug, Clone)]
pub struct WantsToDisarmTrap {
    pub trap: Entity,
}
