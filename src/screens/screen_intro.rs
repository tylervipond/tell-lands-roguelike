use super::constants::{SCREEN_PADDING, SCREEN_WIDTH};
use crate::artwork::SCENARIO_ARTWORK;
use crate::ui_components::{ui_paragraph::UIParagraph, ui_text_line::UITextLine};
use rltk::{Rltk, BLACK, WHITE};

pub struct ScreenIntro {}

impl ScreenIntro {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        SCENARIO_ARTWORK.lines().enumerate().for_each(|(idx, line)| {
            UITextLine::new(0, idx as i32 + 1, WHITE, BLACK, line).draw(ctx)
        });
        let text = "\"The time has come, my adventuring days are over.The last adventure falls to you, my apprentice.
  It is said that within the halls of the ancient underground fortress below our feet lies the a treasure of great power,
  an ancient talisman. Legends say that this talisman protects against time. I suppose it is more than clear why an
  old man such as myself would seek such a thing. In this final act as my apprentice, you are to descend into the
  ancient fortress, retrieve the Talisman, and return it to me, so that I may ward off time.\"";
        UIParagraph::new(
            SCREEN_PADDING as i32,
            SCREEN_PADDING as i32,
            (SCREEN_WIDTH - SCREEN_PADDING * 2) as u32,
            text,
        )
        .draw(ctx);
    }
}
