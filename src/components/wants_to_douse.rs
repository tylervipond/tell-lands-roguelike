use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug, Clone)]
pub struct WantsToDouse {
    pub item: Entity,
}
