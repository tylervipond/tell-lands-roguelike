use crate::components::{contained::Contained, in_backpack::InBackpack, name::Name};
use specs::{Entity, Join, World, WorldExt};

pub type InventoryList = Vec<(Entity, String)>;

pub fn get_player_inventory_list(ecs: &mut World) -> InventoryList {
    let player_entity = ecs.fetch::<Entity>();
    let names = ecs.read_storage::<Name>();
    let backpack = ecs.read_storage::<InBackpack>();
    let entities = ecs.entities();
    (&backpack, &entities, &names)
        .join()
        .filter(|i| i.0.owner == *player_entity)
        .map(|i| (i.1, i.2.name.to_string()))
        .collect()
}

pub fn get_container_inventory_list(ecs: &mut World, container_entity: &Entity) -> InventoryList {
    let names = ecs.read_storage::<Name>();
    let containeds = ecs.read_storage::<Contained>();
    let entities = ecs.entities();
    (&containeds, &entities, &names)
        .join()
        .filter(|i| i.0.container == *container_entity)
        .map(|i| (i.1, i.2.name.to_string()))
        .collect()
}
