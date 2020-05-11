use crate::credits_screen_action::CreditsScreenAction;
use crate::death_screen_action::DeathScreenAction;
#[cfg(debug_assertions)]
use crate::debug_menu_action::DebugMenuAction;
use crate::exit_game_menu_action::ExitGameMenuAction;
use crate::failure_screen_action::FailureScreenAction;
use crate::intro_screen_action::IntroScreenAction;
use crate::inventory_action::InventoryAction;
use crate::main_menu_action::MainMenuAction;
use crate::map_action::MapAction;
use crate::success_screen_action::SuccessScreenAction;
use crate::targeting_action::TargetingAction;
use rltk::{Point, Rltk, VirtualKeyCode};

pub fn map_input_to_map_action(ctx: &mut Rltk) -> MapAction {
  match ctx.key {
    None => MapAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => MapAction::Exit,
      VirtualKeyCode::A => MapAction::MoveLeft,
      VirtualKeyCode::D => MapAction::MoveRight,
      VirtualKeyCode::W => MapAction::MoveUp,
      VirtualKeyCode::X => MapAction::MoveDown,
      VirtualKeyCode::Q => MapAction::MoveUpLeft,
      VirtualKeyCode::E => MapAction::MoveUpRight,
      VirtualKeyCode::Z => MapAction::MoveDownLeft,
      VirtualKeyCode::C => MapAction::MoveDownRight,
      VirtualKeyCode::V => MapAction::SearchContainer,
      VirtualKeyCode::S => MapAction::StayStill,
      VirtualKeyCode::G => MapAction::OpenDoor,
      VirtualKeyCode::F => MapAction::PickupItem,
      VirtualKeyCode::I => MapAction::ShowInventoryMenu,
      VirtualKeyCode::R => MapAction::ShowDropMenu,
      #[cfg(debug_assertions)]
      VirtualKeyCode::Key8 => MapAction::ShowDebugMenu,
      VirtualKeyCode::Period => MapAction::GoDownStairs,
      VirtualKeyCode::Comma => MapAction::GoUpStairs,
      VirtualKeyCode::L => MapAction::LeaveDungeon,
      _ => MapAction::NoAction,
    },
  }
}

pub fn map_input_to_inventory_action(ctx: &mut Rltk, highlighted: usize) -> InventoryAction {
  match ctx.key {
    None => InventoryAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Up => InventoryAction::MoveHighlightUp,
      VirtualKeyCode::Down => InventoryAction::MoveHighlightDown,
      VirtualKeyCode::Left => InventoryAction::PreviousPage,
      VirtualKeyCode::Right => InventoryAction::NextPage,
      VirtualKeyCode::Return => InventoryAction::Select {
        option: highlighted,
      },
      VirtualKeyCode::Escape => InventoryAction::Exit,
      _ => InventoryAction::NoAction,
    },
  }
}

pub fn map_input_to_targeting_action(ctx: &mut Rltk, target: Option<&Point>) -> TargetingAction {
  if ctx.left_click {
    return match target {
      Some(point) => TargetingAction::Selected(*point),
      None => TargetingAction::Exit,
    };
  }
  match ctx.key {
    None => TargetingAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => TargetingAction::Exit,
      _ => TargetingAction::NoAction,
    },
  }
}

pub fn map_input_to_main_menu_action(ctx: &mut Rltk, highlighted: usize) -> MainMenuAction {
  match ctx.key {
    None => MainMenuAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => MainMenuAction::Exit,
      VirtualKeyCode::Left => MainMenuAction::MoveHighlightPrevious,
      VirtualKeyCode::Right => MainMenuAction::MoveHighlightNext,
      VirtualKeyCode::Return => MainMenuAction::Select {
        option: highlighted,
      },
      _ => MainMenuAction::NoAction,
    },
  }
}

pub fn map_input_to_death_screen_action(ctx: &mut Rltk) -> DeathScreenAction {
  match ctx.key {
    None => DeathScreenAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => DeathScreenAction::Exit,
      VirtualKeyCode::Return => DeathScreenAction::Exit,
      _ => DeathScreenAction::NoAction,
    }
  }
}

pub fn map_input_to_exit_game_action(ctx: &mut Rltk, highlighted: usize) -> ExitGameMenuAction {
  match ctx.key {
    None => ExitGameMenuAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => ExitGameMenuAction::Exit,
      VirtualKeyCode::Up => ExitGameMenuAction::MoveHighlightUp,
      VirtualKeyCode::Down => ExitGameMenuAction::MoveHighlightDown,
      VirtualKeyCode::Return => ExitGameMenuAction::Select {
        option: highlighted,
      },
      _ => ExitGameMenuAction::NoAction,
    },
  }
}

pub fn map_input_to_intro_screen_action(ctx: &mut Rltk) -> IntroScreenAction {
  match ctx.key {
    None => IntroScreenAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => IntroScreenAction::Exit,
      VirtualKeyCode::Return => IntroScreenAction::Continue,
      _ => IntroScreenAction::NoAction,
    },
  }
}

pub fn map_input_to_success_screen_action(ctx: &mut Rltk) -> SuccessScreenAction {
  match ctx.key {
    None => SuccessScreenAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => SuccessScreenAction::Exit,
      VirtualKeyCode::Return => SuccessScreenAction::Exit,
      _ => SuccessScreenAction::NoAction,
    },
  }
}

pub fn map_input_to_failure_screen_action(ctx: &mut Rltk) -> FailureScreenAction {
  match ctx.key {
    None => FailureScreenAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => FailureScreenAction::Exit,
      VirtualKeyCode::Return => FailureScreenAction::Exit,
      _ => FailureScreenAction::NoAction,
    },
  }
}

pub fn map_input_to_credits_screen_action(ctx: &mut Rltk) -> CreditsScreenAction {
  match ctx.key {
    None => CreditsScreenAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => CreditsScreenAction::Exit,
      VirtualKeyCode::Return => CreditsScreenAction::Exit,
      _ => CreditsScreenAction::NoAction,
    },
  }
}

#[cfg(debug_assertions)]
pub fn map_input_to_debug_menu_action(ctx: &mut Rltk, highlighted: usize) -> DebugMenuAction {
  match ctx.key {
    None => DebugMenuAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => DebugMenuAction::Exit,
      VirtualKeyCode::Up => DebugMenuAction::MoveHighlightUp,
      VirtualKeyCode::Down => DebugMenuAction::MoveHighlightDown,
      VirtualKeyCode::Return => DebugMenuAction::Select {
        option: highlighted,
      },
      _ => DebugMenuAction::NoAction,
    },
  }
}
