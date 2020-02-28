use crate::map::{Map};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Dungeon {
  pub maps: HashMap<i32, Map>,
}

impl Dungeon {
  pub fn new() -> Self {
    Self {
      maps: HashMap::new(),
    }
  }
  pub fn generate(bottom_floor: i32, top_floor: i32) -> Self {
    let maps = (bottom_floor..top_floor).fold(HashMap::new(), |mut acc, floor_number| {
      let mut map = Map::create_basic_map(floor_number);
      if floor_number != top_floor - 1 {
        map.add_up_stairs();
      }
      if floor_number != bottom_floor {
        map.add_down_stairs();
      }
      acc.insert(floor_number, map);
      return acc;
    });
    Self { maps }
  }

  pub fn get_map(&mut self, floor: i32) -> Option<&mut Map> {
    self.maps.get_mut(&floor)
  }
}
