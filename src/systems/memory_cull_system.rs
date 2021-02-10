use crate::components::Memory;
use specs::{Entities, Entity, Join, System, WriteStorage};

// This should be temporary, being able to wipe memories based on whether the related
// entity is dead or not implies some sort of omniscience, which is likely not realistic
// for any creature wandering around in a dungeon. Needed to avoid certain issues with
// remembered ents no longer existing in the system.
pub struct MemoryCullSystem {}

impl<'a> System<'a> for MemoryCullSystem {
    type SystemData = (WriteStorage<'a, Memory>, Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut memories, entities) = data;
        for memory in (&mut memories).join() {
            let last_known_positions_to_remove: Vec<Entity> = memory
                .last_known_enemy_positions
                .keys()
                .filter(|e| !entities.is_alive(**e))
                .map(|e| *e)
                .collect();
            for key in last_known_positions_to_remove.iter() {
                memory.last_known_enemy_positions.remove(key);
            }
            let hiding_spots_to_remove: Vec<Entity> = memory
                .known_enemy_hiding_spots
                .iter()
                .filter(|(e, hiding_spot)| {
                    !entities.is_alive(**e) || !entities.is_alive(**hiding_spot)
                })
                .map(|(e, _h)| *e)
                .collect();
            for key in hiding_spots_to_remove.iter() {
                memory.known_enemy_hiding_spots.remove(key);
            }
        }
    }
}
