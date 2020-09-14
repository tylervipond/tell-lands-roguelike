use crate::components::{
    BlocksTile, DungeonLevel, EntityMoved, Grabbing, Hiding, Position, Viewshed, WantsToMove,
};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use rltk::Point;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Grabbing>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, DungeonLevel>,
        WriteExpect<'a, Dungeon>,
        WriteStorage<'a, EntityMoved>,
        WriteExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, BlocksTile>,
        WriteStorage<'a, Hiding>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut grabbings,
            mut wants_to_moves,
            mut positions,
            mut viewsheds,
            dungeon_levels,
            mut dungeon,
            mut moved,
            mut player_point,
            player_entity,
            blocks_tiles,
            mut hidings,
        ) = data;

        for (entity, wants_to_move, dungeon_level, grabbing, viewshed) in (
            &entities,
            &mut wants_to_moves,
            &dungeon_levels,
            (&mut grabbings).maybe(),
            (&mut viewsheds).maybe(),
        )
            .join()
        {
            let level = dungeon.get_level_mut(dungeon_level.level).unwrap();
            let ent_position = {
                let pos = positions.get_mut(entity).unwrap();
                (pos.x, pos.y)
            };
            let delta = (
                wants_to_move.x - ent_position.0,
                wants_to_move.y - ent_position.1,
            );
            let current_idx = level_utils::xy_idx(&level, ent_position.0, ent_position.1);
            let ent_destination_idx = level_utils::xy_idx(&level, wants_to_move.x, wants_to_move.y);
            if let Some(grabbing) = grabbing {
                let mut thing_pos = positions.get_mut(grabbing.thing).unwrap();
                let thing_destination_x = thing_pos.x + delta.0;
                let thing_destination_y = thing_pos.y + delta.1;
                let thing_destination_idx =
                    level_utils::xy_idx(&level, thing_destination_x, thing_destination_y);
                let thing_destination_is_ent_position = thing_destination_idx == current_idx;
                let thing_destination_is_blocked =
                    level_utils::tile_is_blocked(thing_destination_idx, &level)
                        && !thing_destination_is_ent_position;
                let thing_current_idx = level_utils::xy_idx(&level, thing_pos.x, thing_pos.y);
                let ent_destination_is_thing_position = ent_destination_idx == thing_current_idx;
                let ent_destination_is_blocked =
                    level_utils::tile_is_blocked(ent_destination_idx, &level)
                        && !ent_destination_is_thing_position;
                if !thing_destination_is_blocked && !ent_destination_is_blocked {
                    thing_pos.x = thing_destination_x;
                    thing_pos.y = thing_destination_y;
                    if let Some(_) = blocks_tiles.get(grabbing.thing) {
                        level.blocked[thing_current_idx as usize] = false;
                        level.blocked[thing_destination_idx as usize] = true;
                    }
                }
            }
            if !level_utils::tile_is_blocked(ent_destination_idx, &level) {
                if let Some(mut viewshed) = viewshed {
                    viewshed.dirty = true;
                }
                let mut ent_pos = positions.get_mut(entity).unwrap();
                ent_pos.x = wants_to_move.x;
                ent_pos.y = wants_to_move.y;
                if let Some(_) = blocks_tiles.get(entity) {
                    level.blocked[current_idx as usize] = false;
                    level.blocked[ent_destination_idx as usize] = true;
                }
                moved
                    .insert(entity, EntityMoved {})
                    .expect("unable to insert EntityMoved");
                if entity == *player_entity {
                    player_point.x = wants_to_move.x;
                    player_point.y = wants_to_move.y;
                }
                hidings.remove(entity);
            }
        }
        wants_to_moves.clear();
    }
}
