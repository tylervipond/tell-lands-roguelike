use crate::artwork::DEATH_ARTWORK;
use crate::ui_components::ui_text_line::UITextLine;
use rltk::{Rltk, BLACK, WHITE};

pub struct ScreenDeath {}

impl ScreenDeath {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        DEATH_ARTWORK.lines().enumerate().for_each(|(idx, line)| {
            UITextLine::new(5, idx as i32 + 1, WHITE, BLACK, line).draw(ctx)
        });
        UITextLine::new(70, 5, WHITE, BLACK, "You Died").draw(ctx);
    }
}
