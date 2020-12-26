use super::{Style, UITextLine};
use rltk::{Rltk, BLACK, YELLOW};

pub struct UITextLineCTA<'a> {
    x: i32,
    y: i32,
    text: &'a str,
}

impl<'a> UITextLineCTA<'a> {
    pub fn new(x: i32, y: i32, text: &'a str) -> Self {
        Self { x, y, text }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        UITextLine::new(
            self.x,
            self.y,
            self.text,
            Some(Style {
                fg: YELLOW,
                bg: BLACK,
            }),
        )
        .draw(ctx);
    }
}
