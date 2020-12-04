use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct CausesDamage {
    pub min: i32,
    pub max: i32,
    pub bonus: i32
}
