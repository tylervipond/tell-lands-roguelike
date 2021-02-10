use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

use std::{collections::HashMap, hash::Hash};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MemoryLocation(pub i32, pub usize);

#[derive(Component, Clone, Debug)]
pub struct Memory {
    pub last_known_enemy_positions: HashMap<Entity, MemoryLocation>,
    pub known_enemy_hiding_spots: HashMap<Entity, Entity>,
    pub wander_destination: Option<MemoryLocation>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MemoryData<M: Eq + Copy + Hash> {
    pub last_known_enemy_positions: Vec<(M, i32, usize)>,
    pub known_enemy_hiding_spots: Vec<(M, M)>,
    pub wander_destination: Option<MemoryLocation>,
}

impl<M: Marker + Serialize + Copy + Hash> ConvertSaveload<M> for Memory
where
    for<'de> M: Deserialize<'de>,
{
    type Data = MemoryData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let last_known_enemy_positions = self
            .last_known_enemy_positions
            .iter()
            .map(|(e, MemoryLocation(level, idx))| (ids(*e).unwrap(), *level, *idx))
            .collect();
        let known_enemy_hiding_spots = self
            .known_enemy_hiding_spots
            .iter()
            .map(|(enemy, hiding_spot)| (ids(*enemy).unwrap(), ids(*hiding_spot).unwrap()))
            .collect();
        Ok(MemoryData {
            last_known_enemy_positions,
            known_enemy_hiding_spots,
            wander_destination: self.wander_destination,
        })
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let last_known_enemy_positions = data.last_known_enemy_positions.iter().fold(
            HashMap::new(),
            |mut acc, (e, location, idx)| {
                acc.insert(ids(*e).unwrap(), MemoryLocation(*location, *idx));
                acc
            },
        );

        let known_enemy_hiding_spots = data.known_enemy_hiding_spots.iter().fold(
            HashMap::new(),
            |mut acc, (enemy, hiding_spot)| {
                acc.insert(ids(*enemy).unwrap(), ids(*hiding_spot).unwrap());
                acc
            },
        );
        Ok(Memory {
            last_known_enemy_positions,
            known_enemy_hiding_spots,
            wander_destination: data.wander_destination,
        })
    }
}
