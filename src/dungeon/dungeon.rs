use crate::dungeon::level::Level;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::dungeon::operations::{add_down_stairs, add_exit, add_up_stairs, create_bsp_level};
#[derive(Default, Serialize, Deserialize, Clone, Debug)]

pub struct Dungeon {
  pub levels: HashMap<i32, Level>,
}

impl Dungeon {
  pub fn generate(bottom_floor: i32, top_floor: i32) -> Self {
    let levels = (bottom_floor..top_floor).fold(HashMap::new(), |mut acc, floor_number| {
      let mut level = create_bsp_level(floor_number);
      if floor_number != top_floor - 1 {
        add_up_stairs(&mut level);
      } else {
        add_exit(&mut level);
      }
      if floor_number != bottom_floor {
        add_down_stairs(&mut level);
      }
      acc.insert(floor_number, level);
      return acc;
    });
    Self { levels }
  }

  pub fn get_level(&mut self, floor: i32) -> Option<&mut Level> {
    self.levels.get_mut(&floor)
  }
}
