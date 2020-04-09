use crate::ui_components::ui_text_line_centered::UITextLineCentered;
use rltk::{Rltk, BLACK, WHITE};

pub struct ScreenDeath {}

impl ScreenDeath {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        UITextLineCentered::new(2, 5, 76, WHITE, BLACK, "You Died").draw(ctx);
    }
}
