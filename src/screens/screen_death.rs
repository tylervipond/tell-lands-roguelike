use super::constants::{SCREEN_PADDING, SCREEN_WIDTH};
use crate::ui_components::ui_text_line_centered::UITextLineCentered;
use rltk::{Rltk, BLACK, WHITE};

pub struct ScreenDeath {}

impl ScreenDeath {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        UITextLineCentered::new(
            SCREEN_PADDING as i32,
            5,
            (SCREEN_WIDTH - SCREEN_PADDING * 2) as u32,
            WHITE,
            BLACK,
            "You Died",
        )
        .draw(ctx);
    }
}
