use serde::{Deserialize, Serialize};

use crate::components::causes_damage::DamageType;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum TrapType {
    BearTrap,
    Caltrops,
    PitTrap,
}

pub fn get_glyph_for_trap(trap_type: &TrapType) -> u16 {
    match trap_type {
        TrapType::Caltrops => rltk::to_cp437('%'),
        TrapType::BearTrap => rltk::to_cp437('^'),
        TrapType::PitTrap => 9,
    }
}

pub fn get_damage_for_trap(trap_type: &TrapType) -> i32 {
    match trap_type {
        TrapType::Caltrops => 4,
        TrapType::BearTrap => 8,
        TrapType::PitTrap => 10,
    }
}

pub fn get_name_for_trap(trap_type: &TrapType) -> String {
    match trap_type {
        TrapType::Caltrops => "Armed Caltrops".to_string(),
        TrapType::BearTrap => "Armed BearTrap".to_string(),
        TrapType::PitTrap => "PitTrap".to_string(),
    }
}

pub fn is_trap_single_activation(trap_type: &TrapType) -> bool {
    match trap_type {
        TrapType::BearTrap => true,
        _ => false,
    }
}

pub fn get_damage_type_for_trap(trap_type: &TrapType) -> Box<[DamageType]> {
    match trap_type {
        TrapType::Caltrops => Box::new([DamageType::Pierce]),
        TrapType::BearTrap => Box::new([DamageType::Crush]),
        TrapType::PitTrap => Box::new([DamageType::Crush]),
    }
}
