use rltk::{field_of_view, Point};
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::components::{player::Player, position::Position, viewshed::Viewshed};
use crate::map::Map;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
  type SystemData = (
    WriteExpect<'a, Map>,
    Entities<'a>,
    WriteStorage<'a, Viewshed>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Player>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, entities, mut viewshed, position, player) = data;
    for (ent, viewshed, position) in (&entities, &mut viewshed, &position).join() {
      if viewshed.dirty {
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
            let idx = map.xy_idx(vis.x, vis.y);
            map.revealed_tiles[idx] = true;
            map.visible_tiles[idx] = true;
          }
        }
      }
    }
  }
}
