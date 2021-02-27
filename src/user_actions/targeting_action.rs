use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum TargetingAction {
    Exit,
    Selected,
}

impl Display for TargetingAction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
          f,
          "{}",
          String::from(match self {
              TargetingAction::Exit => "Exit Screen",
              TargetingAction::Selected => "Select Target",
          })
      )
  }
}
