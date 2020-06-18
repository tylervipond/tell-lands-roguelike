use super::{
  rect::Rect,
  room_stamp_parts::{RoomPart, RoomPart::Floor},
};
use crate::dungeon::room_type::RoomType;
use crate::utils::get_random_element;
use rltk::RandomNumberGenerator;
use serde::{Deserialize, Serialize};
use stamp_rs::{Stamp, StampPart, StampPart::Use};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
  pub rect: Rect,
  pub room_type: Option<RoomType>,
  pub stamp: Stamp<StampPart<RoomPart>>,
}

impl Room {
  pub fn new(rect: Rect) -> Self {
    let room_type = match rect.area() {
      9..=100 => {
        let mut rng = RandomNumberGenerator::new();
        let choices = vec![
          Some(RoomType::TreasureRoom),
          Some(RoomType::Collapsed),
          Some(RoomType::StoreRoom),
          Some(RoomType::BedRoom),
          None,
        ];
        get_random_element(&mut rng, &choices).to_owned()
      }
      101..=200 => {
        let mut rng = RandomNumberGenerator::new();
        let choices = vec![
          Some(RoomType::MessHall),
          None,
        ];
        get_random_element(&mut rng, &choices).to_owned()
      }
      _ => None,
    };

    let stamp = Stamp::new(
      (0..rect.height())
        .map(|_| (0..rect.width()).map(|_| Use(Floor)).collect())
        .collect(),
    );
    Room {
      rect,
      room_type,
      stamp,
    }
  }
}
