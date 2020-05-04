use super::rect::Rect;
use crate::dungeon::room_type::RoomType;
use rltk::RandomNumberGenerator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
  pub rect: Rect,
  pub room_type: Option<RoomType>,
}

impl Room {
  pub fn new(rect: Rect) -> Self {
    let roll = {
      let mut rng = RandomNumberGenerator::new();
      rng.roll_dice(1, 3)
    };

    let room = Room {
      rect,
      room_type: match roll {
        1 => Some(RoomType::TreasureRoom),
        2 => Some(RoomType::Collapsed),
        3 => Some(RoomType::StoreRoom),
        _ => None,
      },
    };
    room
  }
}
