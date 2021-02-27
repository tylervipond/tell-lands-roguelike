use std::fmt::Display;

use crate::ui_components::{ui_paragraph::UIParagraph, UIMenuItemGroup};
use crate::{
    menu::MenuOption,
    screens::constants::{SCREEN_PADDING, SCREEN_WIDTH},
};
use rltk::Rltk;

pub struct ScreenOptions<'a, T: Display + Copy> {
    title: &'a str,
    cta: &'a str,
    control_options: Box<[&'a MenuOption<T>]>,
}

impl<'a, T: Display + Copy> ScreenOptions<'a, T> {
    pub fn new(title: &'a str, cta: &'a str, control_options: Box<[&'a MenuOption<T>]>) -> Self {
        Self {
            title,
            cta,
            control_options,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        UIParagraph::new(
            SCREEN_PADDING as i32,
            SCREEN_PADDING as i32,
            (SCREEN_WIDTH - SCREEN_PADDING * 2) as u32,
            self.title,
        )
        .draw(ctx);

        UIParagraph::new(
            SCREEN_PADDING as i32,
            SCREEN_PADDING as i32 + 1,
            (SCREEN_WIDTH - SCREEN_PADDING * 2) as u32,
            self.cta,
        )
        .draw(ctx);

        UIMenuItemGroup::new(
            SCREEN_PADDING as i32,
            SCREEN_PADDING as i32 + 3,
            &self.control_options,
            true,
        )
        .draw(ctx);
    }
}
