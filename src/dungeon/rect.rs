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

  pub fn contains(&self, x: i32, y: i32) -> bool {
    return self.x1 <= x && self.x2 >= x && self.y1 <= y && self.y2 >= y;
  }
}
