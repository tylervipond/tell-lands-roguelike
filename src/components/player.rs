use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Player {}
