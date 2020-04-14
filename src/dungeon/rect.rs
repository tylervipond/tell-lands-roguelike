use rltk::RandomNumberGenerator;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Rect {
  pub x1: i32,
  pub x2: i32,
  pub y1: i32,
  pub y2: i32,
}

impl Rect {
  pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
    Self {
      x1: x,
      x2: x + w,
      y1: y,
      y2: y + h,
    }
  }

  pub fn center(&self) -> (i32, i32) {
    ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
  }

  pub fn get_random_coord(&self, rng: &mut RandomNumberGenerator) -> (i32, i32) {
    let x = rng.range(self.x1 + 1, self.x2);
    let y = rng.range(self.y1 + 1, self.y2);
    (x, y)
  }

  pub fn get_random_wall_adjacent_coord(&self, rng: &mut RandomNumberGenerator) -> (i32, i32) {
    // Note: This function does not take into account circular rooms
    let random_x = rng.range(self.x1 + 1, self.x2);
    let random_y = rng.range(self.y1 + 1, self.y2);

    return match  rng.roll_dice(1, 4) {
      1 => (self.x1 + 1, random_y),
      2 => (self.x2 - 1, random_y),
      3 => (random_x, self.y1 + 1),
      4 => (random_x, self.y2 - 1),
      // Default to upper left corner, case should never occur on a D4
      _ => (self.x1, self.y1),
    }
  }
}
