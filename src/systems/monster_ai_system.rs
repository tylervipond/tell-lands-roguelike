use crate::components::{
  confused::Confused, monster::Monster, position::Position, viewshed::Viewshed,
  wants_to_melee::WantsToMelee,
};
use crate::map::Map;
use crate::RunState;
use rltk::{a_star_search, DistanceAlg::Pythagoras, Point};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    WriteExpect<'a, Map>,
    ReadExpect<'a, Point>,
    ReadExpect<'a, Entity>,
    ReadExpect<'a, RunState>,
    Entities<'a>,
    WriteStorage<'a, Viewshed>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Monster>,
    WriteStorage<'a, WantsToMelee>,
    WriteStorage<'a, Confused>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (
      mut map,
      player_position,
      player_entity,
      runstate,
      entities,
      mut viewsheds,
      mut positions,
      monsters,
      mut wants_to_melee,
      mut confused,
    ) = data;
    if *runstate != RunState::MonsterTurn {
      return;
    }
    for (_monsters, entity, mut viewshed, mut position) in
      (&monsters, &entities, &mut viewsheds, &mut positions).join()
    {
      if let Some(is_confused) = confused.get_mut(entity) {
        is_confused.turns -= 1;
        if is_confused.turns < 1 {
          confused.remove(entity);
        }
        return;
      }
      let distance = Pythagoras.distance2d(Point::new(position.x, position.y), *player_position);
      if distance < 1.5 {
        wants_to_melee
          .insert(
            entity,
            WantsToMelee {
              target: *player_entity,
            },
          )
          .expect("Unable to insert attack");
      } else if viewshed.visible_tiles.contains(&*player_position) {
        let idx1 = map.xy_idx(position.x, position.y) as i32;
        let idx2 = map.xy_idx(player_position.x, player_position.y) as i32;
        let path = a_star_search(idx1, idx2, &mut *map);
        if path.success && path.steps.len() > 1 {
          let (x, y) = map.idx_xy(path.steps[1]);
          position.x = x;
          position.y = y;
          viewshed.dirty = true;
          map.blocked[idx1 as usize] = false;
          map.blocked[path.steps[1] as usize] = true;
        }
      }
    }
  }
}
