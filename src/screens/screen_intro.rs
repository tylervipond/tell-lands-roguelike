use crate::ui_components::ui_paragraph::UIParagraph;
use rltk::Rltk;

pub struct ScreenIntro {}

impl ScreenIntro {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        let text = "\"The time has come, my adventuring days are over.The last adventure falls to you, my apprentice.
  It is said that within the halls of the ancient underground fortress below our feet lies the a treasure of great power,
  an ancient talisman. Legends say that this talisman protects against time. I suppose it is more than clear why an
  old man such as myself would seek such a thing. In this final act as my apprentice, you are to descend into the
  ancient fortress, retrieve the Talisman, and return it to me, so that I may ward off time.\"";
        UIParagraph::new(2, 2, 76, text).draw(ctx);
    }
}
