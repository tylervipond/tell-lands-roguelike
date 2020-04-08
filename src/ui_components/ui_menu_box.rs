use super::{ui_box::UIBox, ui_text_line_cta::UITextLineCTA, ui_text_line::UITextLine};
use rltk::{Rltk, BLACK, WHITE};

pub struct UIMenuBox<'a> {
    x: i32,
    y: i32,
    width: u8,
    height: u8,
    cta: &'a Option<String>,
    title: &'a Option<String>,
}

impl<'a> UIMenuBox<'a> {
    pub fn new(x: i32, y: i32, width: u8, height: u8, cta: &'a Option<String>, title: &'a Option<String>,) -> Self {
        Self {
            x,
            y,
            width,
            height,
            cta,
            title
        }
    }
    pub fn draw(&self, ctx: &mut Rltk) {
        UIBox::new(self.x, self.y, self.width, self.height, WHITE, BLACK).draw(ctx);
        if let Some(title) = self.title {
            UITextLine::new(self.x + 1, self.y, WHITE, BLACK, &title).draw(ctx);
        }
        if let Some(cta) = self.cta {
            UITextLineCTA::new(self.x + 1, self.y + self.height as i32, &cta).draw(ctx);
        }
    }
}
