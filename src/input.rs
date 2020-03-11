use crate::death_screen_action::DeathScreenAction;
use crate::inventory_action::InventoryAction;
use crate::main_menu_action::MainMenuAction;
use crate::exit_game_menu_action::ExitGameMenuAction;
use crate::map_action::MapAction;
use crate::targeting_action::TargetingAction;
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::Entity;

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
      VirtualKeyCode::F => MapAction::PickupItem,
      VirtualKeyCode::I => MapAction::ShowInventoryMenu,
      VirtualKeyCode::R => MapAction::ShowDropMenu,
      VirtualKeyCode::Period => MapAction::GoDownStairs,
      VirtualKeyCode::Comma => MapAction::GoUpStairs,
      VirtualKeyCode::L => MapAction::LeaveDungeon,
      _ => MapAction::NoAction,
    },
  }
}

pub fn map_input_to_inventory_action(
  ctx: &mut Rltk,
  inventory: &mut Vec<Entity>,
) -> InventoryAction {
  match ctx.key {
    None => InventoryAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Escape => InventoryAction::Exit,
      _ => {
        let selection = rltk::letter_to_option(key);
        if selection > -1 && selection < inventory.len() as i32 {
          return InventoryAction::Selected(inventory.remove(selection as usize));
        }
        return InventoryAction::NoAction;
      }
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
      VirtualKeyCode::Up => MainMenuAction::MoveHighlightUp,
      VirtualKeyCode::Down => MainMenuAction::MoveHighlightDown,
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
      _ => DeathScreenAction::NoAction,
    },
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