use super::{ui_box::UIBox, ui_text_line::UITextLine, ui_text_line_cta::UITextLineCTA};
use rltk::{Rltk, BLACK, WHITE};

pub struct UIMenuBox<'a> {
    x: i32,
    y: i32,
    width: u8,
    height: u8,
    cta: Option<&'a str>,
    title: Option<&'a str>,
}

impl<'a> UIMenuBox<'a> {
    pub fn new(
        x: i32,
        y: i32,
        width: u8,
        height: u8,
        cta: Option<&'a str>,
        title: Option<&'a str>,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            cta,
            title,
        }
    }
    pub fn draw(&self, ctx: &mut Rltk) {
        UIBox::new(self.x, self.y, self.width, self.height, WHITE, BLACK).draw(ctx);
        if let Some(title) = self.title {
            UITextLine::new(self.x + 1, self.y, &title, None).draw(ctx);
        }
        if let Some(cta) = self.cta {
            UITextLineCTA::new(self.x + 1, self.y + self.height as i32, &cta).draw(ctx);
        }
    }
}
