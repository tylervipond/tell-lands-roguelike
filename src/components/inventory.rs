use crate::entity_set::EntitySet;
use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Inventory {
    pub items: EntitySet
}
