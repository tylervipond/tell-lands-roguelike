use super::{ui_box::UIBox, ui_list::UIList, utils::get_longest_line_length};
use rltk::{Rltk, BLACK, WHITE};
use super::constants::{INTERNAL_PADDING};

pub struct UIDynamicBoxList<'a, 'b> {
    pub x: i32,
    pub y: i32,
    pub width: u8,
    pub height: u8,
    pub list_items: &'b Box<[&'a str]>,
}

impl<'a, 'b> UIDynamicBoxList<'a, 'b> {
    pub fn new(x: i32, y: i32, list_items: &'b Box<[&'a str]>) -> Self {
        let height = list_items.len() as u8 + INTERNAL_PADDING * 2;
        Self {
            x,
            y,
            width: get_longest_line_length(&list_items) as u8 + INTERNAL_PADDING * 2,
            height,
            list_items,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        UIBox::new(self.x, self.y, self.width, self.height, WHITE, BLACK).draw(ctx);
        let item_group_x = self.x + INTERNAL_PADDING as i32;
        let item_group_y = self.y + INTERNAL_PADDING as i32;
        UIList::new(item_group_x, item_group_y, self.list_items.clone()).draw(ctx);
    }
}
