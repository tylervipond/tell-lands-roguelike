use rltk::{Rltk, VirtualKeyCode};

pub enum MapAction {
  MoveLeft,
  MoveRight,
  MoveUp,
  MoveDown,
  MoveUpLeft,
  MoveUpRight,
  MoveDownLeft,
  MoveDownRight,
  SearchContainer,
  StayStill,
  OpenDoor,
  PickupItem,
  ShowInventoryMenu,
  ShowDropMenu,
  ShowActionMenu,
  SearchHidden,
  DisarmTrap,
  #[cfg(debug_assertions)]
  ShowDebugMenu,
  NoAction,
  GoDownStairs,
  GoUpStairs,
  Exit,
  LeaveDungeon,
}

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
      VirtualKeyCode::Tab => MapAction::ShowActionMenu,
      VirtualKeyCode::H => MapAction::SearchHidden,
      VirtualKeyCode::T => MapAction::DisarmTrap,
      #[cfg(debug_assertions)]
      VirtualKeyCode::Key8 => MapAction::ShowDebugMenu,
      VirtualKeyCode::Period => MapAction::GoDownStairs,
      VirtualKeyCode::Comma => MapAction::GoUpStairs,
      VirtualKeyCode::L => MapAction::LeaveDungeon,
      _ => MapAction::NoAction,
    },
  }
}
