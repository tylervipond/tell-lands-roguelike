use rltk::Rltk;
use super::ui_text_line::UITextLine;
use super::utils::get_offset_from_center;

pub struct UITextLineCentered<'a> {
    x: i32,
    y: i32,
    width: u32,
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
    text: &'a str,
}

impl<'a> UITextLineCentered<'a> {
    pub fn new(x: i32, y: i32, width: u32, fg: (u8, u8, u8), bg: (u8, u8, u8), text: &'a str) -> Self {
        Self {
            x,
            y,
            width,
            fg,
            bg,
            text,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {

        let text_x = get_offset_from_center(self.width as usize, self.text.chars().count());
        UITextLine::new(self.x + text_x as i32, self.y,self.fg, self.bg, self.text).draw(ctx);
    }
}
