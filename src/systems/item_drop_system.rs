use crate::components::{
  in_backpack::InBackpack, name::Name, position::Position, wants_to_drop_item::WantsToDropItem,
};
use crate::game_log::GameLog;
use specs::{Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct ItemDropSystem {}

impl<'a> System<'a> for ItemDropSystem {
  type SystemData = (
    ReadExpect<'a, Entity>,
    WriteExpect<'a, GameLog>,
    WriteStorage<'a, WantsToDropItem>,
    ReadStorage<'a, Name>,
    WriteStorage<'a, Position>,
    WriteStorage<'a, InBackpack>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (player_entity, mut game_log, mut wants_to_drop, names, mut positions, mut backpacks) =
      data;
    let mut entities_with_positions: Vec<(Entity, Position)> = (&wants_to_drop, &positions)
      .join()
      .map(|(to_drop, pos)| (to_drop.item, pos.clone()))
      .collect();

    entities_with_positions.drain(0..).for_each(|(ent, pos)| {
      positions
        .insert(ent, pos)
        .expect("failed to add ent to positions");
      backpacks
        .remove(ent)
        .expect("failed to remove ent from backpacks");
      if ent == *player_entity {
        game_log
          .entries
          .insert(0, format!("You drop the {}.", names.get(ent).unwrap().name))
      }
    });
    wants_to_drop.clear();
  }
}
