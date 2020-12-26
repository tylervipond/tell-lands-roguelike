use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct CausesLight {
    pub radius: usize,
    pub lit: bool,
    pub turns_remaining: Option<u32>,
}
