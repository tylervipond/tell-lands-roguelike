use crate::components::{DungeonLevel, Player, Position, Viewshed};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use rltk::Point;
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
        ReadStorage<'a, DungeonLevel>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut dungeon, entities, mut viewshed, position, player, dungeon_levels) = data;
        for (ent, viewshed, position, dungeon_level) in
            (&entities, &mut viewshed, &position, &dungeon_levels).join()
        {
            if viewshed.dirty {
                let level = dungeon.get_level_mut(dungeon_level.level).unwrap();
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = rltk::field_of_view(
                    Point::new(position.x, position.y),
                    viewshed.range,
                    &*level,
                )
                .into_iter()
                .filter(|p| {
                    let idx = level_utils::xy_idx(&level, p.x, p.y) as usize;
                    level.lit_tiles[idx]
                })
                .collect();
                if let Some(_p) = player.get(ent) {
                    for t in level.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = level_utils::xy_idx(&level, vis.x, vis.y) as usize;
                        level.revealed_tiles[idx] = true;
                        level.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
