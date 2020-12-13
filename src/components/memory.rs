use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MemoryDestination {
    pub idx: i32,
    pub level: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct MemoryPosition {
    pub idx: i32,
    pub level: i32,
    pub entity: Entity,
}

impl PartialEq for MemoryPosition {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

impl Eq for MemoryPosition {}

impl Hash for MemoryPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MemoryHidingSpot {
    pub enemy: Entity,
    pub hiding_spot: Entity,
}

impl PartialEq for MemoryHidingSpot {
    fn eq(&self, other: &Self) -> bool {
        self.enemy == other.enemy
    }
}

impl Eq for MemoryHidingSpot {}

impl Hash for MemoryHidingSpot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.enemy.hash(state);
    }
}


#[derive(Component, Clone, Debug)]
pub struct Memory {
    pub last_known_enemy_positions: HashSet<MemoryPosition>,
    pub known_enemy_hiding_spots: HashSet<MemoryHidingSpot>,
    pub wander_destination: Option<MemoryDestination>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MemoryDataPosition<M: Eq + Copy> {
    pub idx: i32,
    pub level: i32,
    pub entity: M,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MemoryDataHidingSpot<M: Eq + Copy> {
    pub enemy: M,
    pub hiding_spot: M,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MemoryData<M: Eq + Copy> {
    pub last_known_enemy_positions: Vec<MemoryDataPosition<M>>,
    pub known_enemy_hiding_spots: Vec<MemoryDataHidingSpot<M>>,
    pub wander_destination: Option<MemoryDestination>,
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
        let last_known_enemy_positions = self
            .last_known_enemy_positions
            .iter()
            .map(|memory_position| MemoryDataPosition {
                idx: memory_position.idx,
                level: memory_position.level,
                entity: ids(memory_position.entity).unwrap(),
            })
            .collect();
        let known_enemy_hiding_spots = self
            .known_enemy_hiding_spots
            .iter()
            .map(|memory_hiding_spot| MemoryDataHidingSpot {
                enemy: ids(memory_hiding_spot.enemy).unwrap(),
                hiding_spot: ids(memory_hiding_spot.hiding_spot).unwrap(),
            })
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
        let last_known_enemy_positions = data
            .last_known_enemy_positions
            .iter()
            .map(|memory_data_position| MemoryPosition {
                idx: memory_data_position.idx,
                level: memory_data_position.level,
                entity: ids(memory_data_position.entity).unwrap(),
            })
            .collect();

        let known_enemy_hiding_spots = data
            .known_enemy_hiding_spots
            .iter()
            .map(|memory_data_hiding_spot| MemoryHidingSpot {
                enemy: ids(memory_data_hiding_spot.enemy).unwrap(),
                hiding_spot: ids(memory_data_hiding_spot.hiding_spot).unwrap(),
            })
            .collect();
        Ok(Memory {
            last_known_enemy_positions,
            known_enemy_hiding_spots,
            wander_destination: data.wander_destination,
        })
    }
}
