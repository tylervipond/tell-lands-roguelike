use crate::components::{Grabbing, WantsToGrab};
use specs::{Entities, Join, System, WriteStorage};

pub struct GrabSystem {}

impl<'a> System<'a> for GrabSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToGrab>,
        WriteStorage<'a, Grabbing>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_to_grabs, mut grabbings) = data;
        for (wants_to_grab, entity) in (&mut wants_to_grabs, &entities).join() {
            match wants_to_grab.thing.get_entity() {
                Some(thing) => {
                    grabbings.insert(entity, Grabbing { thing }).expect("couldn't add grabbing to entity");
                }
                None => {
                    grabbings.remove(entity).expect("couldn't remove grabbing from entity");
                }
            };
        }
        wants_to_grabs.clear();
    }
}
