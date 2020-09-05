use crate::components::{memory::MemoryPosition, DungeonLevel, Memory, Position, Viewshed};
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
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut memories, viewsheds, dungeon_levels, player_entity, player_position, positions) =
            data;
        for (memory, viewshed, dungeon_level, position) in
            (&mut memories, &viewsheds, &dungeon_levels, &positions).join()
        {
            match viewshed
                .visible_tiles
                .iter()
                .find(|p| p.x == player_position.x && p.y == player_position.y)
            {
                Some(point) => {
                    memory.last_known_enemy_positions.insert(
                        *player_entity,
                        MemoryPosition {
                            x: point.x,
                            y: point.y,
                            level: dungeon_level.level as i32,
                        },
                    );
                }
                None => {}
            };
            let reached_wander_destination = match memory.wander_destination {
                None => false,
                Some(MemoryPosition { x, y, level }) => {
                    x == position.x && y == position.y && level == dungeon_level.level as i32
                }
            };
            if reached_wander_destination {
                memory.wander_destination = None;
            }
            let enemy_position_reached = match memory.last_known_enemy_positions.iter().find(
                |(_, MemoryPosition { x, y, level })| {
                    *x == position.x && *y == position.y && *level == dungeon_level.level as i32
                },
            ) {
                Some((ent, _)) => Some(*ent),
                _ => None
            };

            if let Some(ent) = enemy_position_reached {
                memory.last_known_enemy_positions.remove(&ent);
            }
        }
    }
}
