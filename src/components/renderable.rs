use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Debug, Clone)]
pub struct Renderable {
  pub glyph: u8,
  pub fg: RGB,
  pub bg: RGB,
  pub layer: i32,
}
