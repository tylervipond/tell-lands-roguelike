use crate::components::{DungeonLevel, Player, Position, Viewshed};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use rltk::Point;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

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
    ReadExpect<'a, Entity>,
    ReadStorage<'a, DungeonLevel>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (mut dungeon, entities, mut viewshed, position, player, player_ent, dungeon_levels) = data;
    let player_level = dungeon_levels.get(*player_ent).unwrap();
    let level = dungeon.get_level(player_level.level).unwrap();
    for (ent, viewshed, position, dungeon_level) in
      (&entities, &mut viewshed, &position, &dungeon_levels).join()
    {
      if viewshed.dirty && dungeon_level.level == player_level.level {
        viewshed.dirty = false;
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles =
          rltk::field_of_view(Point::new(position.x, position.y), viewshed.range, &*level);
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
