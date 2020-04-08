pub enum InventoryAction {
  NoAction,
  Exit,
  Select { option: usize },
  MoveHighlightDown,
  MoveHighlightUp,
}
