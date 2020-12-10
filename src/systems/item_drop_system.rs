use crate::components::{InBackpack, Name, Position, WantsToDropItem};
use crate::services::GameLog;
use specs::{Entity, Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
  type SystemData = (
    Entities<'a>,
    ReadExpect<'a, Entity>,
    WriteExpect<'a, GameLog>,
    WriteStorage<'a, WantsToDropItem>,
    ReadStorage<'a, Name>,
    WriteStorage<'a, Position>,
    WriteStorage<'a, InBackpack>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (
      entities,
      player_entity,
      mut game_log,
      mut wants_to_drop,
      names,
      mut positions,
      mut backpacks,
    ) = data;
      (&wants_to_drop, &entities).join()
        .for_each(|(to_drop, dropping_ent)| {
          let dropped_ent = to_drop.item;
          let (dropping_ent_idx, dropping_ent_level) = {
            let pos = positions.get(dropping_ent).unwrap();
            (pos.idx, pos.level)
          };
          positions
            .insert(dropped_ent, Position {
              idx: dropping_ent_idx,
              level: dropping_ent_level
            })
            .expect("failed to add dropped_ent to positions");
          backpacks
            .remove(dropped_ent)
            .expect("failed to remove dropped_ent from backpacks");
          if dropping_ent == *player_entity {
            game_log
              .entries
              .insert(0, format!("You drop the {}.", names.get(dropped_ent).unwrap().name))
          }
       });
    wants_to_drop.clear();
  }
}
