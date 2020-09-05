use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MemoryPosition {
    pub x: i32,
    pub y: i32,
    pub level: i32,
}
#[derive(Component, Clone, Debug)]
pub struct Memory {
    pub last_known_enemy_positions: HashMap<Entity, MemoryPosition>,
    pub wander_destination: Option<MemoryPosition>,
}

#[derive(Serialize, Deserialize)]
pub struct MemoryData<M: Eq + Hash + Copy> {
    pub last_known_enemy_positions: HashMap<M, MemoryPosition>,
    pub wander_destination: Option<MemoryPosition>,
}

impl<M: Marker + Serialize + Copy> ConvertSaveload<M> for Memory
where
    for<'de> M: Deserialize<'de>,
{
    type Data = MemoryData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let last_known_enemy_positions = HashMap::from_iter(
            self.last_known_enemy_positions
                .iter()
                .map(|(k, v)| (ids(*k).unwrap(), *v)),
        );
        Ok(MemoryData {
            last_known_enemy_positions,
            wander_destination: self.wander_destination,
        })
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let last_known_enemy_positions = HashMap::from_iter(
            data.last_known_enemy_positions
                .iter()
                .map(|(k, v)| (ids(*k).unwrap(), *v)),
        );
        Ok(Memory {
            last_known_enemy_positions,
            wander_destination: data.wander_destination,
        })
    }
}
