use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};
#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize)]
pub enum EquipmentPositions {
    DominantHand,
    OffHand,
    Head,
    Torso,
    Legs,
    Feet,
    Arms
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Equipable {
    pub positions: Box<[EquipmentPositions]>
}