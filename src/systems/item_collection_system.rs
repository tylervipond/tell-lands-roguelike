use crate::components::{
  contained::Contained, in_backpack::InBackpack, name::Name, position::Position,
  wants_to_pick_up_item::WantsToPickUpItem,
};
use crate::services::GameLog;
use specs::{Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
  type SystemData = (
    ReadExpect<'a, Entity>,
    WriteExpect<'a, GameLog>,
    WriteStorage<'a, WantsToPickUpItem>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Name>,
    WriteStorage<'a, InBackpack>,
    WriteStorage<'a, Contained>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (
      player_entity,
      mut game_log,
      mut wants_to_pick_up,
      mut positions,
      names,
      mut backpack,
      mut contained,
    ) = data;
    for pick_up in wants_to_pick_up.join() {
      positions.remove(pick_up.item);
      contained.remove(pick_up.item);
      backpack
        .insert(
          pick_up.item,
          InBackpack {
            owner: pick_up.collected_by,
          },
        )
        .expect("failed to insert item in backpack");
      if pick_up.collected_by == *player_entity {
        game_log.add(format!(
          "you pick up the {}",
          names.get(pick_up.item).unwrap().name
        ))
      }
    }
    wants_to_pick_up.clear();
  }
}
