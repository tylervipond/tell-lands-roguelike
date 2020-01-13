use specs::{Entity, World, WorldExt, Join};
use crate::components::{name::Name, in_backpack::InBackpack};

pub type InventoryList = Vec<(Entity, String)>;

pub fn get_inventory_list(ecs: &mut World, entity: Entity) -> InventoryList {
  let names = ecs.read_storage::<Name>();
  let backpack = ecs.read_storage::<InBackpack>();
  let entities = ecs.entities();
  (&backpack, &entities, &names)
    .join()
    .filter(|i| i.0.owner == entity)
    .map(|i| (i.1, i.2.name.to_string()))
    .collect()
}