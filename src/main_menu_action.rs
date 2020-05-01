pub enum MainMenuAction {
  Select {option: usize},
  MoveHighlightNext,
  MoveHighlightPrevious,
  Exit,
  NoAction
}