use super::constants::{SCREEN_PADDING, SCREEN_WIDTH};
use crate::menu_option::MenuOption;
use crate::ui_components::ui_menu_item_group_centered::UIMenuItemGroupCentered;
use rltk::Rltk;

pub struct ScreenMainMenu<'a> {
    menu_options: &'a Vec<MenuOption<'a>>,
}

impl<'a> ScreenMainMenu<'a> {
    pub fn new(menu_options: &'a Vec<MenuOption<'a>>) -> Self {
        Self { menu_options }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        UIMenuItemGroupCentered::new(
            SCREEN_PADDING as i32,
            SCREEN_PADDING as i32,
            (SCREEN_WIDTH - SCREEN_PADDING * 2) as u32,
            self.menu_options,
        )
        .draw(ctx);
    }
}
