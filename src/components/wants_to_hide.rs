use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

#[derive(Component, Clone, Debug)]
pub struct WantsToHide {
    pub hiding_spot: Option<Entity>,
}

#[derive(Serialize, Deserialize)]
pub struct WantsToHideData<M: Eq> {
    pub hiding_spot: Option<M>,
}

impl<M: Marker + Serialize> ConvertSaveload<M> for WantsToHide
where
    for<'de> M: Deserialize<'de>,
{
    type Data = WantsToHideData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>,
    {
        let hiding_spot =  match self.hiding_spot {
            Some(entity) => ids(entity),
            _ => None
        };
        Ok(WantsToHideData {
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
        Ok(WantsToHide {
            hiding_spot
        })
    }
}
