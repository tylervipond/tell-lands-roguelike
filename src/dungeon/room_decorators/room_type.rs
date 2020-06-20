use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum RoomType {
  Collapsed,
  TreasureRoom,
  StoreRoom,
  BedRoom,
  MessHall,
  SittingRoom,
  Kitchen,
  Barracks,
  DiningRoom,
}
