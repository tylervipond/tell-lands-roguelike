use crate::components::{
  confused::Confused, dungeon_level::DungeonLevel, monster::Monster, position::Position,
  viewshed::Viewshed, wants_to_melee::WantsToMelee,
};
use crate::dungeon::dungeon::Dungeon;
use crate::RunState;
use rltk::{a_star_search, DistanceAlg::Pythagoras, Point};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    WriteExpect<'a, Dungeon>,
    ReadExpect<'a, Point>,
    ReadExpect<'a, Entity>,
    ReadExpect<'a, RunState>,
    Entities<'a>,
    WriteStorage<'a, Viewshed>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Monster>,
    WriteStorage<'a, WantsToMelee>,
    WriteStorage<'a, Confused>,
    ReadStorage<'a, DungeonLevel>,
  );
  // This is currently very limited. Monsters will only act if they can see a player, which means that they must
  // also be on the same map to act.
  fn run(&mut self, data: Self::SystemData) {
    let (
      mut dungeon,
      player_position,
      player_entity,
      runstate,
      entities,
      mut viewsheds,
      mut positions,
      monsters,
      mut wants_to_melee,
      mut confused,
      levels,
    ) = data;
    if *runstate != RunState::MonsterTurn {
      return;
    }

    let player_level = levels.get(*player_entity).unwrap();
    let map = dungeon.get_map(player_level.level).unwrap();

    for (_monsters, entity, mut viewshed, mut position, level) in (
      &monsters,
      &entities,
      &mut viewsheds,
      &mut positions,
      &levels,
    )
      .join()
    {
      if let Some(is_confused) = confused.get_mut(entity) {
        is_confused.turns -= 1;
        if is_confused.turns < 1 {
          confused.remove(entity);
        }
        continue;
      }
      if level.level != player_level.level {
        continue;
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
        let idx1 = map.xy_idx(position.x, position.y) as usize;
        let idx2 = map.xy_idx(player_position.x, player_position.y) as usize;
        let path = a_star_search(idx1, idx2, &mut *map);
        if path.success && path.steps.len() > 1 {
          let (x, y) = map.idx_xy(path.steps[1] as i32);
          position.x = x as i32;
          position.y = y as i32;
          viewshed.dirty = true;
          map.blocked[idx1] = false;
          map.blocked[path.steps[1]] = true;
        }
      }
    }
  }
}
