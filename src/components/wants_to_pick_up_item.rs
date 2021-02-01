use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};

use crate::entity_option::EntityOption;

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToPickUpItem {
  pub container: EntityOption,
  pub item: Entity,
}
