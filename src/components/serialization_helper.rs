use crate::dungeon::dungeon::Dungeon;
use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Debug, Clone)]
pub struct SerializationHelper {
  pub dungeon: Dungeon,
}
