use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Clone, Debug)]
pub struct WantsToHide {
    pub hiding_spot: Option<Entity>,
}
