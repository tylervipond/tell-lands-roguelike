use crate::components::{memory::MemoryLocation, Hiding, Memory, Position, Viewshed, WantsToHide};
use specs::{Entity, Join, ReadExpect, ReadStorage, System, WriteStorage};
pub struct UpdateMemoriesSystem {}

impl<'a> System<'a> for UpdateMemoriesSystem {
    type SystemData = (
        WriteStorage<'a, Memory>,
        ReadStorage<'a, Viewshed>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, WantsToHide>,
        ReadStorage<'a, Hiding>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut memories, viewsheds, player_entity, positions, hide_intents, hiding) = data;
        let player_position = positions.get(*player_entity).unwrap();
        for (memory, viewshed, position) in (&mut memories, &viewsheds, &positions).join() {
            if hiding.get(*player_entity).is_none()
                && position.level == player_position.level
                && viewshed.visible_tiles.contains(&player_position.idx)
            {
                memory.last_known_enemy_positions.insert(
                    *player_entity,
                    MemoryLocation(player_position.level as i32, player_position.idx),
                );
                if let Some(hide_intent) = hide_intents.get(*player_entity) {
                    if let Some(container_ent) = hide_intent.hiding_spot {
                        memory
                            .known_enemy_hiding_spots
                            .insert(*player_entity, container_ent);
                    }
                } else {
                    memory.known_enemy_hiding_spots.remove(&player_entity);
                }
            }

            let reached_wander_destination = match memory.wander_destination {
                None => false,
                Some(MemoryLocation(level, idx)) => {
                    idx == position.idx && level == position.level as i32
                }
            };
            if reached_wander_destination {
                memory.wander_destination = None;
            }
            let found_mem_pos = {
                match memory.last_known_enemy_positions.iter().find(
                    |(_e, MemoryLocation(level, idx))| {
                        *idx == position.idx && *level == position.level as i32
                    },
                ) {
                    Some((e, _location)) => Some(*e),
                    None => None,
                }
            };

            if let Some(e) = found_mem_pos {
                memory.last_known_enemy_positions.remove(&e);
            }
        }
    }
}
