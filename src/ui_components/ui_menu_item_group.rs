use super::ui_text_line_menu_item::UITextLineMenuItem;
use crate::menu_option::MenuOption;
use rltk::Rltk;

pub struct UIMenuItemGroup<'a> {
    x: i32,
    y: i32,
    menu_options: &'a Vec<MenuOption<'a>>,
}

impl<'a> UIMenuItemGroup<'a> {
    pub fn new(x: i32, y: i32, menu_options: &'a Vec<MenuOption<'a>>) -> Self {
        Self { x, y, menu_options }
    }
    pub fn draw(&self, ctx: &mut Rltk) {
        for (i, menu_option) in self.menu_options.iter().enumerate() {
            let this_text_y = self.y + i as i32;
            UITextLineMenuItem::new(self.x, this_text_y, &menu_option).draw(ctx);
        }
    }
}
