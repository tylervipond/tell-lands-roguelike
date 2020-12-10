use crate::components::{BlocksTile, Position};
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
  );
  fn run(&mut self, data: Self::SystemData) {
    let (mut dungeon, player_ent, positions, blockers, entities) = data;
    let player_level = positions.get(*player_ent).unwrap().level;
    let mut level = dungeon.get_level_mut(player_level).unwrap();
    level_utils::populate_blocked(&mut level);
    level_utils::clear_content_index(&mut level);
    for (position, entity) in (&positions, &entities).join() {
      if position.level == player_level {
        let blocked = blockers.get(entity);
        if let Some(_) = blocked {
          level.blocked[position.idx] = true
        }
        level.tile_content[position.idx].push(entity);
      }
    }
  }
}
