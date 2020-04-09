use crate::ui_components::ui_paragraph::UIParagraph;
use rltk::Rltk;

pub struct ScreenFailure {}

impl ScreenFailure {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        let text = "\"You have failed me. I am doomed to grow old and die. I have no successor.\"";
        UIParagraph::new(2, 2, 76, text).draw(ctx);
    }
}
