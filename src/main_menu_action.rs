pub enum MainMenuAction {
  Select {option: usize},
  MoveHighlightUp,
  MoveHighlightDown,
  Exit,
  NoAction
}