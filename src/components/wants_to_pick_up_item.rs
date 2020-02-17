use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToPickUpItem {
  pub collected_by: Entity,
  pub item: Entity,
}
