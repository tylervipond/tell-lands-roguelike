use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum RoomType {
  Empty,
  Collapsed,
  TreasureRoom,
  StoreRoom,
}
