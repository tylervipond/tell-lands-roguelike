use crate::ui_components::ui_paragraph::UIParagraph;
use rltk::{Console, Rltk};

pub struct ScreenSuccess {}

impl ScreenSuccess {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        let text = "\"Finally, the talisman is within my grasp, you have passed your final challenge as my apprentice. What magnificent adventures await you\"";
        UIParagraph::new(2, 2, 76, text).draw(ctx);
    }
}
