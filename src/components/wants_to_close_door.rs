use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Clone, Debug)]
pub struct WantsToCloseDoor {
    pub door: Entity,
}
