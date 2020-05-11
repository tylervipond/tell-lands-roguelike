use crate::components::flammable::Flammable;
use specs::{Entities, Join, ReadStorage, System};

pub struct FireDieSystem {}

impl<'a> System<'a> for FireDieSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Flammable>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, flammables) = data;
        for (entity, flammable) in (&entities, &flammables).join() {
            if flammable.turns_remaining < 1 {
                entities.delete(entity).expect("couldn't delete entity");
            }
        }
    }
}
