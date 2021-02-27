use std::fmt::Display;

use super::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::artwork::INTRO_ARTWORK;
use crate::menu::MenuOption;
use crate::ui_components::{UIMenuItemGroupHorizontal, UITextLine};
use rltk::Rltk;

pub struct ScreenMainMenu<'a, T: Display + Copy> {
    menu_options: Box<[&'a MenuOption<T>]>,
}

impl<'a, T: Display + Copy> ScreenMainMenu<'a, T> {
    pub fn new(menu_options: Box<[&'a MenuOption<T>]>) -> Self {
        Self { menu_options }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        let half_screen_width = SCREEN_WIDTH / 2;
        INTRO_ARTWORK.lines().enumerate().for_each(|(idx, line)| {
            UITextLine::new(
                (half_screen_width - line.chars().count() as u8 / 2) as i32,
                idx as i32 + 1,
                line,
                None,
            )
            .draw(ctx)
        });
        let menu_y = SCREEN_HEIGHT as i32 - 3;
        let mut menu = UIMenuItemGroupHorizontal::new(0, menu_y, &self.menu_options);
        menu.x = SCREEN_WIDTH as i32 / 2 - menu.width as i32 / 2;
        menu.draw(ctx);
    }
}
