use serde::{Deserialize, Serialize};
use specs::{
  error::NoError,
  saveload::{ConvertSaveload, Marker},
  Component, DenseVecStorage, Entity,
};
use super::equipable::EquipmentPositions;

#[derive(Component, Debug, Clone)]
pub struct WantsToEquip {
  pub equipment: Option<Entity>,
  pub position: EquipmentPositions
}

#[derive(Serialize, Deserialize)]
pub struct WantsToEquipData<M: Eq> {
    pub equipment: Option<M>,
    pub position: EquipmentPositions
}

impl<M: Marker + Serialize> ConvertSaveload<M> for WantsToEquip
where
    for<'de> M: Deserialize<'de>,
{
    type Data = WantsToEquipData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let equipment =  match self.equipment {
            Some(entity) => ids(entity),
            _ => None
        };
        Ok(WantsToEquipData {
            equipment,
            position: self.position,
            }
        )
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let equipment =  match data.equipment {
            Some(entity) => ids(entity),
            _ => None
        };
        Ok(WantsToEquip {
            equipment,
            position: data.position,
        })
    }
}