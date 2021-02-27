use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum StaticAction {
    Exit,
    Continue,
}

impl Display for StaticAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                StaticAction::Exit => "Exit Screen",
                StaticAction::Continue => "Continue",
            })
        )
    }
}
