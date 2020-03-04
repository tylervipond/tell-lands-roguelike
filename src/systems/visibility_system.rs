use rltk::{field_of_view, Point};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{
  dungeon_level::DungeonLevel, player::Player, position::Position, viewshed::Viewshed,
};
use crate::dungeon::dungeon::Dungeon;

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
    let (mut dungeon, entities, mut viewshed, position, player, player_ent, levels) = data;
    let player_level = levels.get(*player_ent).unwrap();
    let map = dungeon.get_map(player_level.level).unwrap();
    for (ent, viewshed, position, level) in (&entities, &mut viewshed, &position, &levels).join() {
      if viewshed.dirty && level.level == player_level.level {
        viewshed.dirty = false;
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles =
          field_of_view(Point::new(position.x, position.y), viewshed.range, &*map);
        let p: Option<&Player> = player.get(ent);
        if let Some(_p) = p {
          for t in map.visible_tiles.iter_mut() {
            *t = false
          }
          for vis in viewshed.visible_tiles.iter() {
            let idx = map.xy_idx(vis.x, vis.y) as usize;
            map.revealed_tiles[idx] = true;
            map.visible_tiles[idx] = true;
          }
        }
      }
    }
  }
}
