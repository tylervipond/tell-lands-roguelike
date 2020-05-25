use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Debug, Clone)]
pub struct WantsToMove {
  pub x: i32,
  pub y: i32,
}
