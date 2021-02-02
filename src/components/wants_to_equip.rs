use super::equipable::EquipmentPositions;
use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug, Clone)]
pub struct WantsToEquip {
    pub equipment: Option<Entity>,
    pub position: EquipmentPositions,
}
