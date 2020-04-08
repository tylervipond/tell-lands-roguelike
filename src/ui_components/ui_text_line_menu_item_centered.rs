use super::ui_text_line_centered::UITextLineCentered;
use rltk::{Rltk, BLACK, WHITE, YELLOW, GREY};
use crate::menu_option::{MenuOption, MenuOptionState};

pub struct UITextLineMenuItemCentered<'a> {
    x: i32,
    y: i32,
    width: u32,
    menu_option: &'a MenuOption<'a>,
}

impl<'a> UITextLineMenuItemCentered<'a> {
    pub fn new(x: i32, y: i32, width: u32, menu_option: &'a MenuOption<'a>) -> Self {
        Self {
            x,
            y,
            width,
            menu_option
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        let fg = match self.menu_option.state {
            MenuOptionState::Highlighted => YELLOW,
            MenuOptionState::Normal => WHITE,
            MenuOptionState::Disabled => GREY,
        };
        UITextLineCentered::new(self.x, self.y, self.width, fg, BLACK, self.menu_option.text).draw(ctx);
    }
}
