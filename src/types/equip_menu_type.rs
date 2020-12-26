use specs::Entity;
use crate::components::equipable::EquipmentPositions;

pub enum EquipMenuType {
    Exchange(EquipmentPositions),
    Light(Entity),
    Douse(Entity)
}