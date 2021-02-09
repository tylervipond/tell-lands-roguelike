use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Viewshed {
  pub los_tiles: HashSet<usize>,
  pub visible_tiles: HashSet<usize>,
  pub range: u32,
  pub dirty: bool,
}
