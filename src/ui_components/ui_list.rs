use super::UITextLine;
use rltk::Rltk;

pub struct UIList<'a> {
    list_items: Box<[&'a str]>,
    x: i32,
    y: i32,
}

impl<'a> UIList<'a> {
    pub fn new(x: i32, y: i32, list_items: Box<[&'a str]>) -> Self {
        Self { list_items, x, y }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        for (i, list_item) in self.list_items.iter().enumerate() {
            let this_text_y = self.y + i as i32;
            UITextLine::new(self.x, this_text_y, list_item, None).draw(ctx);
        }
    }
}
