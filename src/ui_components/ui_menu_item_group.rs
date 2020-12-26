use super::{style::Style, utils, UITextLine};
use crate::menu_option::{MenuOption, MenuOptionState};
use rltk::{Rltk, BLACK, GREY, YELLOW1, WHITE, YELLOW4};

pub struct UIMenuItemGroup<'a> {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    menu_options: &'a Vec<MenuOption<'a>>,
    active: bool,
}

impl<'a> UIMenuItemGroup<'a> {
    pub fn new(x: i32, y: i32, menu_options: &'a Vec<MenuOption<'a>>, active: bool) -> Self {
        let lines: Vec<String> = menu_options.iter().map(|o| o.text.to_string()).collect();
        let width = utils::get_longest_line_length(&lines) as u32;
        let height = menu_options.len() as u32;
        Self {
            x,
            y,
            height,
            width,
            menu_options,
            active,
        }
    }
    pub fn draw(&self, ctx: &mut Rltk) {
        for (i, menu_option) in self.menu_options.iter().enumerate() {
            let this_text_y = self.y + i as i32;
            let fg = match menu_option.state {
                MenuOptionState::Highlighted => match self.active {
                    true => YELLOW1,
                    false => YELLOW4,
                },
                MenuOptionState::Normal => WHITE,
                MenuOptionState::Disabled => GREY,
            };
            let style = Style { fg, bg: BLACK };
            UITextLine::new(self.x, this_text_y, menu_option.text, Some(style)).draw(ctx);
        }
    }
}
