use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

#[derive(Component, Clone, Debug)]
pub struct Hiding {
    pub hiding_spot: Option<Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct HidingData<M: Eq> {
    pub hiding_spot: Option<M>,
}

impl<M: Marker + Serialize> ConvertSaveload<M> for Hiding
where
    for<'de> M: Deserialize<'de>,
{
    type Data = HidingData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let hiding_spot =  match self.hiding_spot {
            Some(entity) => ids(entity),
            _ => None
        };
        Ok(HidingData {
            hiding_spot
            }
        )
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>,
    {
        let hiding_spot =  match data.hiding_spot {
            Some(entity) => ids(entity),
            _ => None
        };
        Ok(Hiding {
            hiding_spot
        })
    }
}
