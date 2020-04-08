use super::{ui_text_line::UITextLine, utils::split_to_lines};
use rltk::{Rltk, BLACK, WHITE};

pub struct UIParagraph<'a> {
    x: i32,
    y: i32,
    width: u32,
    text: &'a str,
}

impl<'a> UIParagraph<'a> {
    pub fn new(x: i32, y: i32, width: u32, text: &'a str) -> Self {
        Self { x, y, width, text }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        let lines = split_to_lines(self.text, self.width);
        for (i, line) in lines.iter().enumerate() {
            let y = self.y + i as i32;
            UITextLine::new(self.x, y, WHITE, BLACK, line).draw(ctx);
        }
    }
}
