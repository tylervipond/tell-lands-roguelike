use crate::components::{Hiding, Name, Position, WantsToHide};
use crate::services::GameLog;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct HideSystem {}

impl<'a> System<'a> for HideSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToHide>,
        WriteStorage<'a, Hiding>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut wants_to_hides,
            mut hidings,
            player_entity,
            mut log,
            names,
            mut positions,
        ) = data;
        for (wants_to_hide, entity) in (&mut wants_to_hides, &entities).join() {
            match wants_to_hide.hiding_spot {
                Some(hiding_spot) => {
                    let hiding_spot_idx = positions.get(hiding_spot).unwrap().idx;
                    let ent_position = positions.get_mut(entity).unwrap();
                    ent_position.idx = hiding_spot_idx;
                    hidings
                        .insert(
                            entity,
                            Hiding {
                                hiding_spot: Some(hiding_spot),
                            },
                        )
                        .expect("couldn't add hiding to entity");
                    if entity == *player_entity {
                        let name = match names.get(hiding_spot) {
                            Some(name) => name.name.to_owned(),
                            None => "unknown".to_string(),
                        };
                        log.add(format!("You hide in the {}", name))
                    }
                }
                None => {
                    hidings
                        .remove(entity)
                        .expect("couldn't remove hiding from entity");
                }
            };
        }
        wants_to_hides.clear();
    }
}
