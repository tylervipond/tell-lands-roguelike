use rltk::{Rltk, VirtualKeyCode};

pub enum StaticAction {
    Exit,
    Continue,
    NoAction,
}

pub fn map_input_to_static_action(ctx: &mut Rltk) -> StaticAction {
    match ctx.key {
        None => StaticAction::NoAction,
        Some(key) => match key {
            VirtualKeyCode::Escape => StaticAction::Exit,
            VirtualKeyCode::Return => StaticAction::Continue,
            _ => StaticAction::NoAction,
        },
    }
}
