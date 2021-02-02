use specs::{Component, DenseVecStorage};

#[derive(Component, Clone, Debug)]
pub struct WantsToOpenDoor {
    pub idx: usize,
    pub level: usize,
}
