// Taken from https://github.com/amethyst/specs/issues/681 so that we can
// serialize and deserialize Vec<Entity>

use serde::{Deserialize, Serialize};
use specs::{
    saveload::{ConvertSaveload, Marker},
    Entity,
};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct EntitySet<T: Eq + Hash>(HashSet<T>);

impl<T: Eq + Hash> EntitySet<T> {
    pub fn new() -> EntitySet<T> {
        EntitySet { 0: HashSet::new() }
    }

    pub fn with_capacity(capacity: usize) -> EntitySet<T> {
        EntitySet {
            0: HashSet::with_capacity(capacity),
        }
    }
    pub fn insert(&mut self, item: T) -> bool {
        self.0.insert(item)
    }

    pub fn contains(&self, item: &T) -> bool {
        self.0.contains(item)
    }
}

impl<T: Eq + Hash> std::ops::Deref for EntitySet<T> {
    type Target = HashSet<T>;
    fn deref(&self) -> &HashSet<T> {
        &self.0
    }
}

impl<T: Eq + Hash> std::ops::DerefMut for EntitySet<T> {
    fn deref_mut(&mut self) -> &mut HashSet<T> {
        &mut self.0
    }
}

impl<C: Eq + Hash, M: Serialize + Marker> ConvertSaveload<M> for EntitySet<C>
where
    for<'de> M: Deserialize<'de>,
    C: ConvertSaveload<M>,
{
    type Data = Vec<<C as ConvertSaveload<M>>::Data>;
    type Error = <C as ConvertSaveload<M>>::Error;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let mut output = Vec::with_capacity(self.len());
        for item in self.iter() {
            let converted_item = item.convert_into(|entity| ids(entity))?;

            output.push(converted_item);
        }
        Ok(output)
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let mut output: EntitySet<C> = EntitySet::with_capacity(data.len());
        for item in data.into_iter() {
            let converted_item = ConvertSaveload::convert_from(item, |marker| ids(marker))?;
            output.insert(converted_item);
        }
        Ok(output)
    }
}
