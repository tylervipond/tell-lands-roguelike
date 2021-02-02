use crate::components::{Container, Inventory, Name, Position, WantsToPickUpItem};
use crate::services::GameLog;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToPickUpItem>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Container>,
        WriteStorage<'a, Inventory>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player_entity,
            mut game_log,
            mut wants_to_pick_up,
            mut positions,
            names,
            mut containers,
            mut inventories,
        ) = data;

        for (ent, pick_up, inventory) in (&entities, &wants_to_pick_up, &mut inventories).join() {
            for item in pick_up.items.iter() {
                positions.remove(*item);
                if let Some(container_ent) = pick_up.container {
                    if let Some(container) = containers.get_mut(container_ent) {
                      container.items.remove(&item);
                    }
                }
                inventory.items.insert(*item);
                if ent == *player_entity {
                    game_log.add(format!(
                        "you pick up the {}",
                        names.get(*item).unwrap().name
                    ))
                }
            }
        }
        wants_to_pick_up.clear();
    }
}
