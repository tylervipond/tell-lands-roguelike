use crate::entity_option::EntityOption;
use specs::{Component, DenseVecStorage};

#[derive(Component, Debug, Clone)]
pub struct WantsToGrab {
    pub thing: EntityOption,
}
