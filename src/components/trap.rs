use crate::types::TrapType;
use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

impl Default for TrapType {
    fn default() -> Self {
        TrapType::PitTrap
    }
}

#[derive(Component, ConvertSaveload, Clone, Debug, Default)]
pub struct Trap {
    pub trap_type: TrapType,
}
