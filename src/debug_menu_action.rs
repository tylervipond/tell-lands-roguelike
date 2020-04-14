pub enum DebugMenuAction {
    Exit,
    NoAction,
    MoveHighlightDown,
    MoveHighlightUp,
    Select { option: usize },
}
