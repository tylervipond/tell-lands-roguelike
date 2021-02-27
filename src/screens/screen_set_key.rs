use crate::ui_components::ui_paragraph::UIParagraph;
use crate::{
    screens::constants::{SCREEN_PADDING, SCREEN_WIDTH},
    user_actions::MapAction,
};
use rltk::Rltk;

pub struct ScreenSetKey<'a> {
    action: &'a MapAction,
}

impl<'a> ScreenSetKey<'a> {
    pub fn new(action: &'a MapAction) -> Self {
        Self { action }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        let text = format!("Set key for {}. Escape to exit.", self.action);
        UIParagraph::new(
            SCREEN_PADDING as i32,
            SCREEN_PADDING as i32,
            (SCREEN_WIDTH - SCREEN_PADDING * 2) as u32,
            text.as_str(),
        )
        .draw(ctx);
    }
}
