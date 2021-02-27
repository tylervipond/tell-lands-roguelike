use core::fmt;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum InteractionTargetingAction {
    Exit,
    Selected,
    Next,
    Previous,
}

impl Display for InteractionTargetingAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = String::from(match self{
            InteractionTargetingAction::Exit => "Exit Targeting Mode",
            InteractionTargetingAction::Selected => "Select Target",
            InteractionTargetingAction::Next => "Next Target",
            InteractionTargetingAction::Previous => "Previous Target",
        });
        write!(f, "{}", name)
    }
}