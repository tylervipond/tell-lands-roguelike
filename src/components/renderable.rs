use rltk::RGB;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Renderable {
  pub glyph: u8,
  pub fg: RGB,
  pub bg: RGB,
}
