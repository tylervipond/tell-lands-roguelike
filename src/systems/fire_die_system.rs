use crate::{
    components::{Flammable, Position},
    services::DebrisSpawner,
};
use specs::{Entities, Join, ReadStorage, System, WriteExpect};

pub struct FireDieSystem {}

impl<'a> System<'a> for FireDieSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Flammable>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, DebrisSpawner>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, flammables, positions, mut debris_spawner) = data;
        for (entity, flammable) in (&entities, &flammables).join() {
            if flammable.turns_remaining < 1 {
                let position = positions.get(entity).unwrap();
                debris_spawner.request_burnt_debris(position.idx, position.level);
                entities.delete(entity).expect("couldn't delete entity");
            }
        }
    }
}
