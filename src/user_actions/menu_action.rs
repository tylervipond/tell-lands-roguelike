use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum MenuAction {
    Exit,
    Select,
    Delete,
    SelectAll,
    MoveHighlightNext,
    MoveHighlightPrev,
    NextMenu,
    PreviousMenu,
    NextPage,
    PreviousPage,
}

impl Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                MenuAction::Exit => "Exit Menu",
                MenuAction::Select => "Select",
                MenuAction::Delete => "Delete",
                MenuAction::SelectAll => "Select All",
                MenuAction::MoveHighlightNext => "Next",
                MenuAction::MoveHighlightPrev => "Previous",
                MenuAction::NextMenu => "Next Menu",
                MenuAction::PreviousMenu => "Previous Menu",
                MenuAction::NextPage => "Next Page",
                MenuAction::PreviousPage => "Previous Page",
            })
        )
    }
}
