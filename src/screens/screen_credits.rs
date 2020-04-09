use crate::ui_components::ui_text_line_centered::UITextLineCentered;
use rltk::{Rltk, BLACK, WHITE};

pub struct ScreenCredits {}

impl ScreenCredits {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        UITextLineCentered::new(2, 5, 76, WHITE, BLACK, "Created By").draw(ctx);
        UITextLineCentered::new(2, 6, 76, WHITE, BLACK, "Tyler Vipond").draw(ctx);
    }
}
