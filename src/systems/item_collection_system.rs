use specs::{Entity, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage, Join};
use crate::components::{wants_to_pick_up_item::WantsToPickUpItem, position::Position, name::Name, in_backpack::InBackpack};
use crate::game_log::GameLog;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
  type SystemData = (
    ReadExpect<'a, Entity>,
    WriteExpect<'a, GameLog>,
    WriteStorage<'a, WantsToPickUpItem>,
    WriteStorage<'a, Position>,
    ReadStorage<'a, Name>,
    WriteStorage<'a, InBackpack>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (player_entity, mut game_log, mut wants_to_pick_up, mut positions, names, mut backpack) = data;
    for pick_up in wants_to_pick_up.join() {
      positions.remove(pick_up.item);
      backpack.insert(pick_up.item, InBackpack {owner: pick_up.collected_by}).expect("failed to insert item in backpack");
      if pick_up.collected_by == *player_entity {
        game_log.add(format!("you pick up the {}", names.get(pick_up.item).unwrap().name))
      }
    }
    wants_to_pick_up.clear();
  }
}
