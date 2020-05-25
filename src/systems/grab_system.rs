use crate::components::{Grabbing, Name, WantsToGrab};
use crate::services::GameLog;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct GrabSystem {}

impl<'a> System<'a> for GrabSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToGrab>,
        WriteStorage<'a, Grabbing>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_to_grabs, mut grabbings, player_entity, mut log, names) = data;
        for (wants_to_grab, entity) in (&mut wants_to_grabs, &entities).join() {
            match wants_to_grab.thing.get_entity() {
                Some(thing) => {
                    grabbings
                        .insert(entity, Grabbing { thing })
                        .expect("couldn't add grabbing to entity");
                    if entity == *player_entity {
                        let name = match names.get(thing) {
                            Some(name) => name.name.to_owned(),
                            None => "unknown".to_string(),
                        };
                        log.add(format!("You grab {}", name))
                    }
                }
                None => {
                    grabbings
                        .remove(entity)
                        .expect("couldn't remove grabbing from entity");
                }
            };
        }
        wants_to_grabs.clear();
    }
}
