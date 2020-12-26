use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

#[derive(Component, Deserialize, Serialize, Debug, Clone)]
pub struct Dousable {}
