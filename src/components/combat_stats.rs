use specs::{saveload::{ConvertSaveload, Marker}, Component, DenseVecStorage, Entity, error::NoError};
use serde::{Serialize, Deserialize};

#[derive(Component, Clone, ConvertSaveload, Debug)]
pub struct CombatStats {
  pub max_hp: i32,
  pub hp: i32,
  pub defense: i32,
  pub power: i32,
}
