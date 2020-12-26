use rltk::{Rltk, VirtualKeyCode};

pub enum MenuAction {
  NoAction,
  Exit,
  Select { option: usize },
  MoveHighlightNext,
  MoveHighlightPrev,
  NextMenu,
  PreviousMenu,
  NextPage,
  PreviousPage,
}

pub fn map_input_to_menu_action(ctx: &mut Rltk, highlighted: usize) -> MenuAction {
  match ctx.key {
    None => MenuAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Up => MenuAction::MoveHighlightPrev,
      VirtualKeyCode::Down => MenuAction::MoveHighlightNext,
      VirtualKeyCode::Comma => MenuAction::PreviousPage,
      VirtualKeyCode::Period => MenuAction::NextPage,
      VirtualKeyCode::Left => MenuAction::PreviousMenu,
      VirtualKeyCode::Right => MenuAction::NextMenu,
      VirtualKeyCode::Return => MenuAction::Select {
        option: highlighted,
      },
      VirtualKeyCode::Escape => MenuAction::Exit,
      _ => MenuAction::NoAction,
    },
  }
}

pub fn map_input_to_horizontal_menu_action(ctx: &mut Rltk, highlighted: usize) -> MenuAction {
  match ctx.key {
    None => MenuAction::NoAction,
    Some(key) => match key {
      VirtualKeyCode::Left => MenuAction::MoveHighlightPrev,
      VirtualKeyCode::Right => MenuAction::MoveHighlightNext,
      VirtualKeyCode::Return => MenuAction::Select {
        option: highlighted,
      },
      VirtualKeyCode::Escape => MenuAction::Exit,
      _ => MenuAction::NoAction,
    },
  }
}
