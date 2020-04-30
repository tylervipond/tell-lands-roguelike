use crate::ui_components::ui_text_line_centered::UITextLineCentered;
use rltk::{Rltk, BLACK, WHITE};
use super::constants::{SCREEN_PADDING, SCREEN_WIDTH};
pub struct ScreenCredits {}

impl ScreenCredits {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        let line_width = SCREEN_WIDTH - SCREEN_PADDING * 2;
        UITextLineCentered::new(SCREEN_PADDING as i32, 5, line_width as u32, WHITE, BLACK, "Created By").draw(ctx);
        UITextLineCentered::new(SCREEN_PADDING as i32, 6, line_width as u32, WHITE, BLACK, "Tyler Vipond").draw(ctx);
    }
}
