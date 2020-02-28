use crate::components::{blocks_tile::BlocksTile, dungeon_level::DungeonLevel, position::Position};
use crate::dungeon::dungeon::Dungeon;
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
    let map = dungeon.get_map(player_level.level).unwrap();
    map.populate_blocked();
    map.clear_content_index();
    for (position, entity, level) in (&positions, &entities, &dungeon_levels).join() {
      if level.level == player_level.level {
        let idx = map.xy_idx(position.x, position.y);
        let _p = blockers.get(entity);
        if let Some(_p) = _p {
          map.blocked[idx as usize] = true
        }
        map.tile_content[idx as usize].push(entity);
      }
    }
  }
}
