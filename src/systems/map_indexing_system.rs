use crate::components::{blocks_tile::BlocksTile, position::Position};
use crate::map::basic_map::Map;
use specs::{Entities, Join, ReadStorage, System, WriteExpect};

pub struct MapIndexingSystem {}

// note that currently this system deals with two things, updating 
// the blockers, and updating the content of a given tile. This should 
// possibly be broken down into two distinct systems. Are there 
// performance considerations?
impl<'a> System<'a> for MapIndexingSystem {
  type SystemData = (
    WriteExpect<'a, Map>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, BlocksTile>,
    Entities<'a>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, positions, blockers, entities) = data;

    map.populate_blocked();
    map.clear_content_index();
    for (position, entity) in (&positions, &entities).join() {
      let idx = map.xy_idx(position.x, position.y);
      let _p = blockers.get(entity);
      if let Some(_p) = _p {
        map.blocked[idx as usize] = true
      }
      map.tile_content[idx as usize].push(entity);
    }
  }
}
