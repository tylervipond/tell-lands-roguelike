use super::ui_text_line::UITextLine;
use crate::menu_option::{MenuOption, MenuOptionState};
use rltk::{Rltk, BLACK, GREY, WHITE, YELLOW};

pub struct UITextLineMenuItem<'a> {
    x: i32,
    y: i32,
    menu_option: &'a MenuOption<'a>,
}

impl<'a> UITextLineMenuItem<'a> {
    pub fn new(x: i32, y: i32, menu_option: &'a MenuOption<'a>) -> Self {
        Self { x, y, menu_option }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        let fg = match self.menu_option.state {
            MenuOptionState::Highlighted => YELLOW,
            MenuOptionState::Normal => WHITE,
            MenuOptionState::Disabled => GREY,
        };
        UITextLine::new(self.x, self.y, fg, BLACK, self.menu_option.text).draw(ctx);
    }
}
