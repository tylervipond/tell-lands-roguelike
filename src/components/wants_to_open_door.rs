use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Clone, Debug)]
pub struct WantsToOpenDoor {
    pub door: Entity,
}
