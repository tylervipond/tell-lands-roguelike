use super::level::Level;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Dungeon {
  pub levels: HashMap<u8, Level>,
}

impl Dungeon {
  pub fn get_level(&self, floor: u8) -> Option<&Level> {
    self.levels.get(&floor)
  }

  pub fn get_level_mut(&mut self, floor: u8) -> Option<&mut Level> {
    self.levels.get_mut(&floor)
  }
}
