use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Triggered {}
