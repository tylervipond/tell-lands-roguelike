use super::ui_text_line_menu_item::UITextLineMenuItem;
use crate::menu_option::MenuOption;
use rltk::Rltk;

const SPACE_BETWEEN: i32 = 3;

pub struct UIMenuItemGroupHorizontal<'a> {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub menu_options: &'a Vec<MenuOption<'a>>,
}

impl<'a> UIMenuItemGroupHorizontal<'a> {
    pub fn new(x: i32, y: i32, menu_options: &'a Vec<MenuOption<'a>>) -> Self {
        let white_space = (menu_options.len() - 1) * SPACE_BETWEEN as usize;
        let width = menu_options
            .iter()
            .fold(0, |acc, o| acc + o.text.chars().count())
            + white_space;
        Self {
            x,
            y,
            width: width as u32,
            menu_options,
        }
    }
    pub fn draw(&self, ctx: &mut Rltk) {
        let mut this_text_x = self.x;
        for menu_option in self.menu_options.iter() {
            UITextLineMenuItem::new(this_text_x, self.y, menu_option).draw(ctx);
            this_text_x += menu_option.text.chars().count() as i32 + SPACE_BETWEEN;
        }
    }
}
