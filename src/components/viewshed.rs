use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Viewshed {
  pub los_tiles: Vec<i32>,
  pub visible_tiles: Vec<i32>,
  pub range: i32,
  pub dirty: bool,
}
