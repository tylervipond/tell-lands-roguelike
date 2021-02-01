use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DamageType {
    Blunt,
    Slash,
    Stab,
    Hack,
    Burn,
    Crush,
    Pierce,
}
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct CausesDamage {
    pub min: i32,
    pub max: i32,
    pub bonus: i32,
    pub damage_type: Box<[DamageType]>,
}
