use serde::{Deserialize, Serialize};
use specs::{
    saveload::{ConvertSaveload, Marker},
    Entity,
};

#[derive(Clone, Debug)]
pub struct EntityOption {
    ent: Option<Entity>,
}

impl EntityOption {
    pub fn new(ent: Option<Entity>) -> Self {
        Self { ent }
    }
    #[allow(dead_code)]
    pub fn set_entity(&mut self, ent: Option<Entity>) {
        self.ent = ent;
    }

    pub fn get_entity(&self) -> Option<Entity> {
        self.ent
    }
    #[allow(dead_code)]
    pub fn is_some(&self) -> bool {
        self.ent.is_some()
    }
}

impl<M: Marker + Serialize> ConvertSaveload<M> for EntityOption
where
    for<'de> M: Deserialize<'de>,
{
    type Data = Option<<Entity as ConvertSaveload<M>>::Data>;
    type Error = <Entity as ConvertSaveload<M>>::Error;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let marker = match self.get_entity() {
            Some(e) => ids(e),
            _ => None,
        };
        Ok(marker)
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let entity = match data {
            Some(e) => ids(e),
            _ => None,
        };
        Ok(EntityOption::new(entity))
    }
}
