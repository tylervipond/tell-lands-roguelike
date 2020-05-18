use rltk::Point;
use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct WantsToTrap {
    pub item: Entity,
    pub target: Option<Point>,
}
