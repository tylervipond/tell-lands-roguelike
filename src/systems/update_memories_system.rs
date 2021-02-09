use crate::components::{
    memory::{MemoryDestination, MemoryHidingSpot, MemoryPosition},
    Hiding, Memory, Position, Viewshed, WantsToHide,
};
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
            if hiding.get(*player_entity).is_none() {
                if viewshed.visible_tiles.contains(&player_position.idx) {
                    if hiding.get(*player_entity).is_none() {}
                    memory.last_known_enemy_positions.insert(MemoryPosition {
                        idx: player_position.idx,
                        level: position.level as i32,
                        entity: *player_entity,
                    });
                    if let Some(hide_intent) = hide_intents.get(*player_entity) {
                        if let Some(container_ent) = hide_intent.hiding_spot {
                            memory.known_enemy_hiding_spots.insert(MemoryHidingSpot {
                                enemy: *player_entity,
                                hiding_spot: container_ent,
                            });
                        }
                    } else {
                        let hiding_spot = memory
                            .known_enemy_hiding_spots
                            .iter()
                            .cloned()
                            .find(|s| s.enemy == *player_entity);
                        if let Some(hiding_spot) = hiding_spot {
                            memory.known_enemy_hiding_spots.remove(&hiding_spot);
                        }
                    }
                };
            }

            let reached_wander_destination = match memory.wander_destination {
                None => false,
                Some(MemoryDestination { idx, level }) => {
                    idx == position.idx && level == position.level as i32
                }
            };
            if reached_wander_destination {
                memory.wander_destination = None;
            }
            let found_mem_pos = memory
                .last_known_enemy_positions
                .iter()
                .cloned()
                .find(|mem_pos| {
                    mem_pos.idx == position.idx && mem_pos.level == position.level as i32
                });

            if let Some(found_mem_pos) = found_mem_pos {
                memory.last_known_enemy_positions.remove(&found_mem_pos);
            }
        }
    }
}
