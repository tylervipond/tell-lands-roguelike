use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum TileType {
  Wall,
  Floor,
}
