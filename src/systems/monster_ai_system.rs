use crate::components::{
  Confused, DungeonLevel, Monster, Position, Viewshed, WantsToMelee, WantsToMove,
};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use rltk::{a_star_search, DistanceAlg::Pythagoras, Point};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = (
    WriteExpect<'a, Dungeon>,
    ReadExpect<'a, Point>,
    ReadExpect<'a, Entity>,
    Entities<'a>,
    ReadStorage<'a, Viewshed>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Monster>,
    WriteStorage<'a, WantsToMelee>,
    WriteStorage<'a, Confused>,
    ReadStorage<'a, DungeonLevel>,
    WriteStorage<'a, WantsToMove>,
  );
  // This is currently very limited. Monsters will only act if they can see a player, which means that they must
  // also be on the same level to act.
  fn run(&mut self, data: Self::SystemData) {
    let (
      mut dungeon,
      player_position,
      player_entity,
      entities,
      viewsheds,
      positions,
      monsters,
      mut wants_to_melee,
      mut confused,
      levels,
      mut wants_to_move,
    ) = data;
    let player_level = levels.get(*player_entity).unwrap();
    let level = dungeon.get_level_mut(player_level.level).unwrap();

    for (_monsters, entity, viewshed, position, dungeon_level) in
      (&monsters, &entities, &viewsheds, &positions, &levels).join()
    {
      if let Some(is_confused) = confused.get_mut(entity) {
        is_confused.turns -= 1;
        if is_confused.turns < 1 {
          confused.remove(entity);
        }
        continue;
      }
      if dungeon_level.level != player_level.level {
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
          .expect("Unable to insert attack intent");
      } else if viewshed.visible_tiles.contains(&*player_position) {
        let idx1 = level_utils::xy_idx(&level, position.x, position.y) as usize;
        let idx2 = level_utils::xy_idx(&level, player_position.x, player_position.y) as usize;
        let path = a_star_search(idx1, idx2, &mut *level);
        if path.success && path.steps.len() > 1 {
          let (x, y) = level_utils::idx_xy(&level, path.steps[1] as i32);
          wants_to_move
            .insert(entity, WantsToMove { x, y })
            .expect("couldn't insert move intent");
        }
      }
    }
  }
}
