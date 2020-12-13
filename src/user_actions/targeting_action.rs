use rltk::{Rltk, VirtualKeyCode};

pub enum TargetingAction {
  NoAction,
  Exit,
  Selected(i32)
}

pub fn map_input_to_targeting_action(ctx: &mut Rltk, target: Option<i32>) -> TargetingAction {
  if ctx.left_click {
    return match target {
      Some(idx) => TargetingAction::Selected(idx),
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
