use super::{ui_box::UIBox, ui_list::UIList, utils::get_longest_line_length};
use rltk::{Rltk, BLACK, WHITE};
use super::constants::{INTERNAL_PADDING};

pub struct UIDynamicBoxList<'a> {
    pub x: i32,
    pub y: i32,
    pub width: u8,
    pub height: u8,
    pub list_items: &'a Vec<String>,
}

impl<'a> UIDynamicBoxList<'a> {
    pub fn new(x: i32, y: i32, list_items: &'a Vec<String>) -> Self {
        Self {
            x,
            y,
            width: get_longest_line_length(&list_items) as u8 + INTERNAL_PADDING * 2,
            height: list_items.len() as u8 + INTERNAL_PADDING * 2,
            list_items,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        UIBox::new(self.x, self.y, self.width, self.height, WHITE, BLACK).draw(ctx);
        let item_group_x = self.x + INTERNAL_PADDING as i32;
        let item_group_y = self.y + INTERNAL_PADDING as i32;
        UIList::new(item_group_x, item_group_y, self.list_items).draw(ctx);
    }
}
