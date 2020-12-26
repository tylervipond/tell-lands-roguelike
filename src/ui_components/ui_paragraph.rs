use super::{ui_text_line::UITextLine, utils::split_to_lines};
use rltk::Rltk;

pub struct UIParagraph {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    lines: Vec<String>,
}

impl UIParagraph {
    pub fn new(x: i32, y: i32, width: u32, text: &str) -> Self {
        let lines = split_to_lines(text, width);
        let height = lines.len() as u32;
        Self {
            x,
            y,
            width,
            height,
            lines,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        for (i, line) in self.lines.iter().enumerate() {
            let y = self.y + i as i32;
            UITextLine::new(self.x, y, line, None).draw(ctx);
        }
    }
}
