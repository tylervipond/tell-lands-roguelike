use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

#[derive(Component, Clone, Debug)]
pub struct Equipment {
    pub off_hand: Option<Entity>,
    pub dominant_hand: Option<Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentData<M: Eq> {
    pub off_hand: Option<M>,
    pub dominant_hand: Option<M>,
}

impl<M: Marker + Serialize> ConvertSaveload<M> for Equipment
where
    for<'de> M: Deserialize<'de>,
{
    type Data = EquipmentData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let off_hand =  match self.off_hand {
            Some(entity) => ids(entity),
            _ => None
        };
        let dominant_hand =  match self.dominant_hand {
            Some(entity) => ids(entity),
            _ => None
        };
        Ok(EquipmentData {
            off_hand,
            dominant_hand,
            }
        )
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let off_hand =  match data.off_hand {
            Some(entity) => ids(entity),
            _ => None
        };
        let dominant_hand =  match data.dominant_hand {
            Some(entity) => ids(entity),
            _ => None
        };
        Ok(Equipment {
            off_hand,
            dominant_hand,
        })
    }
}