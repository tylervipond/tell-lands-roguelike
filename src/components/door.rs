use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum DoorState {
    Opened,
    Closed
}
#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Door {
  pub state: DoorState,
}
