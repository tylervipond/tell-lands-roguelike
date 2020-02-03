use specs::{Component, DenseVecStorage, Entity};
use rltk::Point;

#[derive(Component, Debug)]
pub struct WantsToUse {
  pub item: Entity,
  pub target: Option<Point>,
}
