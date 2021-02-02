use specs::{Component, DenseVecStorage};

#[derive(Component, Debug, Clone)]
pub struct WantsToMove {
    pub idx: usize,
}
