use rltk::{Point, Rltk, VirtualKeyCode};

pub enum TargetingAction {
  NoAction,
  Exit,
  Selected(Point)
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
