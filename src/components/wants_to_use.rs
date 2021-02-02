use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Clone, Debug)]
pub struct WantsToUse {
    pub item: Entity,
    pub target: Option<usize>,
}
