pub enum ExitGameMenuAction {
    Exit,
    NoAction,
    MoveHighlightDown,
    MoveHighlightUp,
    Select { option: usize },
}
