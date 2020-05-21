use crate::components::{BlocksTile, DungeonLevel, Position};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect};

pub struct MapIndexingSystem {}

// note that currently this system deals with two things, updating
// the blockers, and updating the content of a given tile. This should
// possibly be broken down into two distinct systems. Are there
// performance considerations?
// I'm getting the current level and the current map in quite a few systems. should probably store those at some point
impl<'a> System<'a> for MapIndexingSystem {
  type SystemData = (
    WriteExpect<'a, Dungeon>,
    ReadExpect<'a, Entity>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, BlocksTile>,
    Entities<'a>,
    ReadStorage<'a, DungeonLevel>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (mut dungeon, player_ent, positions, blockers, entities, dungeon_levels) = data;
    let player_level = dungeon_levels.get(*player_ent).unwrap();
    let mut level = dungeon.get_level_mut(player_level.level).unwrap();
    level_utils::populate_blocked(&mut level);
    level_utils::clear_content_index(&mut level);
    for (position, entity, dungeon_level) in (&positions, &entities, &dungeon_levels).join() {
      if dungeon_level.level == player_level.level {
        let idx = level_utils::xy_idx(&level, position.x, position.y) as usize;
        let blocked = blockers.get(entity);
        if let Some(_) = blocked {
          level.blocked[idx] = true
        }
        level.tile_content[idx].push(entity);
      }
    }
  }
}
