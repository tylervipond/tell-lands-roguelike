use crate::screens::constants::{SCREEN_PADDING, SCREEN_WIDTH};
use crate::ui_components::ui_paragraph::UIParagraph;
use rltk::Rltk;

pub struct ScreenSaving {}

impl ScreenSaving {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        let text = "Saving game...";
        UIParagraph::new(
            SCREEN_PADDING as i32,
            SCREEN_PADDING as i32,
            (SCREEN_WIDTH - SCREEN_PADDING * 2) as u32,
            text,
        )
        .draw(ctx);
    }
}
