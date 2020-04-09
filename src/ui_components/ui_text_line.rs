use rltk::{Rltk, RGB};

pub struct UITextLine<'a> {
    x: i32,
    y: i32,
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
    text: &'a str,
}

impl<'a> UITextLine<'a> {
    pub fn new(x: i32, y: i32, fg: (u8, u8, u8), bg: (u8, u8, u8), text: &'a str) -> Self {
        Self { x, y, fg, bg, text }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.print_color(self.x, self.y, RGB::named(self.fg), RGB::named(self.bg), self.text);
    }
}
