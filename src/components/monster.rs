use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MonsterSpecies {
    Goblin,
}
#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct Monster {
    pub species: MonsterSpecies,
}
