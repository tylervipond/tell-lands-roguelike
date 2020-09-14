use crate::components::{
    memory::{MemoryDestination, MemoryHidingSpot, MemoryPosition},
    DungeonLevel, Hiding, Memory, Position, Viewshed, WantsToHide,
};
use rltk::Point;
use specs::{Entity, Join, ReadExpect, ReadStorage, System, WriteStorage};
pub struct UpdateMemoriesSystem {}

impl<'a> System<'a> for UpdateMemoriesSystem {
    type SystemData = (
        WriteStorage<'a, Memory>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, DungeonLevel>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, WantsToHide>,
        ReadStorage<'a, Hiding>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut memories,
            viewsheds,
            dungeon_levels,
            player_entity,
            player_position,
            positions,
            hide_intents,
            hiding,
        ) = data;
        for (memory, viewshed, dungeon_level, position) in
            (&mut memories, &viewsheds, &dungeon_levels, &positions).join()
        {
            if hiding.get(*player_entity).is_none() {
                if let Some(point) = viewshed
                    .visible_tiles
                    .iter()
                    .find(|p| p.x == player_position.x && p.y == player_position.y)
                {
                    if hiding.get(*player_entity).is_none() {}
                    memory.last_known_enemy_positions.insert(MemoryPosition {
                        x: point.x,
                        y: point.y,
                        level: dungeon_level.level as i32,
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
                Some(MemoryDestination { x, y, level }) => {
                    x == position.x && y == position.y && level == dungeon_level.level as i32
                }
            };
            if reached_wander_destination {
                memory.wander_destination = None;
            }
            let found_mem_pos = memory.last_known_enemy_positions.iter().cloned().find(|mem_pos| {
                mem_pos.x == position.x
                    && mem_pos.y == position.y
                    && mem_pos.level == dungeon_level.level as i32
            });

            if let Some(found_mem_pos) = found_mem_pos {
                memory.last_known_enemy_positions.remove(&found_mem_pos);
            }
        }
    }
}
