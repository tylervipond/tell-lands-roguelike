// adapted from https://github.com/amethyst/specs/issues/681 so that we can
// serialize and deserialize Vec<Entity>

use serde::{Deserialize, Serialize};
use specs::{
    saveload::{ConvertSaveload, Marker},
    Entity,
};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct EntitySet(HashSet<Entity>);

impl EntitySet {
    pub fn new() -> EntitySet {
        EntitySet(HashSet::new())
    }

    pub fn with_capacity(capacity: usize) -> EntitySet {
        EntitySet(HashSet::with_capacity(capacity))
    }

    pub fn insert(&mut self, item: Entity) -> bool {
        self.0.insert(item)
    }

    pub fn contains(&self, item: &Entity) -> bool {
        self.0.contains(item)
    }
}

impl std::ops::Deref for EntitySet {
    type Target = HashSet<Entity>;
    fn deref(&self) -> &HashSet<Entity> {
        &self.0
    }
}

impl std::ops::DerefMut for EntitySet {
    fn deref_mut(&mut self) -> &mut HashSet<Entity> {
        &mut self.0
    }
}

impl<M: Serialize + Marker> ConvertSaveload<M> for EntitySet
where
    for<'de> M: Deserialize<'de>
{
    type Data = Vec<<Entity as ConvertSaveload<M>>::Data>;
    type Error = <Entity as ConvertSaveload<M>>::Error;

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
        let mut output: EntitySet = EntitySet::with_capacity(data.len());
        for item in data.into_iter() {
            let converted_item = ConvertSaveload::convert_from(item, |marker| ids(marker))?;
            output.insert(converted_item);
        }
        Ok(output)
    }
}
