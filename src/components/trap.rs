use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TrapType {
    BearTrap,
    Caltrops,
    PitTrap
}

impl Default for TrapType {
    fn default() -> Self {
        TrapType::PitTrap
    }
}

#[derive(Component, ConvertSaveload, Clone, Debug, Default)]
pub struct Trap {
    pub trap_type: TrapType,
}
