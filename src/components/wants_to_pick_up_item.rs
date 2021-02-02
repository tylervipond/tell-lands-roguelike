use std::collections::HashSet;
use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Clone, Debug)]
pub struct WantsToPickUpItem {
    pub container: Option<Entity>,
    pub items: HashSet<Entity>,
}
