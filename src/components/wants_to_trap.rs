use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Clone, Debug)]
pub struct WantsToTrap {
    pub item: Entity,
    pub target: Option<usize>,
}
