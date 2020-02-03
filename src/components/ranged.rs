use specs::{Component, DenseVecStorage};

#[derive(Component, Debug)]
pub struct Ranged {
  pub range: i32,
}
