use rltk::Point;
use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Viewshed {
  pub visible_tiles: Vec<Point>,
  pub range: i32,
  pub dirty: bool,
}
