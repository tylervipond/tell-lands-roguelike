use std::fmt::Display;

use super::{
    ui_menu_box::UIMenuBox, ui_menu_item_group::UIMenuItemGroup, utils::get_longest_line_length,
};
use crate::menu::MenuOption;
use rltk::Rltk;

const INTERNAL_PADDING: u8 = 2;

pub struct UIDynamicMenu<'a, 'b, T: Display + Copy> {
    pub x: i32,
    pub y: i32,
    pub width: u8,
    pub height: u8,
    pub menu_options: &'b Box<[&'a MenuOption<T>]>,
    pub cta: Option<&'a str>,
    pub title: Option<&'a str>,
}

impl<'a, 'b, T: Display + Copy> UIDynamicMenu<'a, 'b, T> {
    pub fn new(
        x: i32,
        y: i32,
        menu_options: &'b Box<[&'a MenuOption<T>]>,
        cta: Option<&'a str>,
        title: Option<&'a str>,
    ) -> Self {
        let lines: Box<[T]> = menu_options.iter().map(|o| o.text).collect();
        let longest_line_length = get_longest_line_length(&lines);
        let title_length = match &title {
            Some(title) => title.chars().count(),
            None => 0,
        };
        let cta_length = match &cta {
            Some(cta) => cta.chars().count(),
            None => 0,
        };
        let inner_width = match [longest_line_length, title_length, cta_length].iter().max() {
            Some(max) => *max,
            None => 0,
        };
        Self {
            x,
            y,
            width: inner_width as u8 + INTERNAL_PADDING * 2,
            height: menu_options.len() as u8 + INTERNAL_PADDING * 2 - 1,
            menu_options,
            cta,
            title,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        UIMenuBox::new(
            self.x,
            self.y,
            self.width,
            self.height,
            self.cta,
            self.title,
        )
        .draw(ctx);
        let item_group_x = self.x + INTERNAL_PADDING as i32;
        let item_group_y = self.y + INTERNAL_PADDING as i32;
        UIMenuItemGroup::new(item_group_x, item_group_y, self.menu_options, true).draw(ctx);
    }
}
