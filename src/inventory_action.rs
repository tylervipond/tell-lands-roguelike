use specs::Entity;

pub enum InventoryAction {
  NoAction,
  Exit,
  Selected(Entity)
}