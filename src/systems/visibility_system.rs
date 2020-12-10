use crate::components::{Player, Position, Viewshed};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

/**
 * Currently enemy AI won't take any actions if the player is not visible, so
 * we're only going to work with entities on the current map.
 */
pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Dungeon>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut dungeon, entities, mut viewshed, position, player) = data;
        for (ent, viewshed, position) in
            (&entities, &mut viewshed, &position).join()
        {
            let level = dungeon.get_level_mut(position.level).unwrap();
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.los_tiles.clear();
                viewshed.los_tiles = level_utils::get_field_of_view_from_idx(&*level, position.idx as i32, viewshed.range);
            }
            viewshed.visible_tiles = viewshed.los_tiles.iter()
                .cloned()
                .filter(|idx| level.lit_tiles[*idx as usize])
                .chain(
                    level_utils::get_field_of_view_from_idx(&*level, position.idx as i32, 2)
                    .iter().cloned()
                )
                .collect();
                if let Some(_p) = player.get(ent) {
                    for t in level.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for idx in viewshed.visible_tiles.iter() {
                        level.revealed_tiles[*idx as usize] = true;
                        level.visible_tiles[*idx as usize] = true;
                    }
                }
            }
        }
    }

