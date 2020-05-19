use super::trap_type::TrapType;
use rltk::{RGB, YELLOW};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ItemType {
    BearTrap,
    Caltrops,
}

pub fn get_glyph_for_item(item_type: &ItemType) -> u16 {
    match item_type {
        ItemType::Caltrops => rltk::to_cp437('%'),
        ItemType::BearTrap => rltk::to_cp437('^'),
    }
}

pub fn get_name_for_item(item_type: &ItemType) -> String {
    match item_type {
        ItemType::Caltrops => "Caltrops".to_string(),
        ItemType::BearTrap => "Beartrap".to_string(),
    }
}

pub fn get_foreground_color_for_item(item_type: &ItemType) -> RGB {
    match item_type {
        ItemType::Caltrops | ItemType::BearTrap => RGB::named(YELLOW),
    }
}

pub fn item_is_consumable(item_type: &ItemType) -> bool {
    match item_type {
        ItemType::Caltrops | ItemType::BearTrap => true,
    }
}

pub fn get_range_for_item(item_type: &ItemType) -> Option<i32> {
    match item_type {
        ItemType::Caltrops => Some(2),
        ItemType::BearTrap => Some(1),
    }
}

pub fn get_trap_type_for_item(item_type: &ItemType) -> Option<TrapType> {
    match item_type {
        ItemType::Caltrops => Some(TrapType::Caltrops),
        ItemType::BearTrap => Some(TrapType::BearTrap),
    }
}
