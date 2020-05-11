use super::rect::Rect;
use crate::dungeon::room_type::RoomType;
use crate::utils::get_random_element;
use rltk::RandomNumberGenerator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
  pub rect: Rect,
  pub room_type: Option<RoomType>,
}

impl Room {
  pub fn new(rect: Rect) -> Self {
    let area = (rect.x2 - rect.x1) * (rect.y2 - rect.y1);
    let room_type = match area {
      9..=100 => {
        let mut rng = RandomNumberGenerator::new();
        let choices = vec![
          Some(RoomType::TreasureRoom),
          Some(RoomType::Collapsed),
          Some(RoomType::StoreRoom),
          None,
        ];
        get_random_element(&mut rng, &choices).to_owned()
      }
      _ => None,
    };
    Room { rect, room_type }
  }
}
