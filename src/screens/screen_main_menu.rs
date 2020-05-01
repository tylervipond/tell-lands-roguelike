use super::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::artwork::INTRO_ARTWORK;
use crate::menu_option::MenuOption;
use crate::ui_components::{
    ui_menu_item_group_horizontal::UIMenuItemGroupHorizontal, ui_text_line::UITextLine,
};
use rltk::{Rltk, BLACK, WHITE};

pub struct ScreenMainMenu<'a> {
    menu_options: &'a Vec<MenuOption<'a>>,
}

impl<'a> ScreenMainMenu<'a> {
    pub fn new(menu_options: &'a Vec<MenuOption<'a>>) -> Self {
        Self { menu_options }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.cls();
        let half_screen_width = SCREEN_WIDTH / 2;
        INTRO_ARTWORK.lines().enumerate().for_each(|(idx, line)| {
            UITextLine::new(
                (half_screen_width - line.chars().count() as u8 / 2) as i32,
                idx as i32 + 1,
                WHITE,
                BLACK,
                line,
            )
            .draw(ctx)
        });
        let menu_y = SCREEN_HEIGHT as i32 - 3;
        let mut menu = UIMenuItemGroupHorizontal::new(0, menu_y, self.menu_options);
        menu.x = SCREEN_WIDTH as i32 / 2 - menu.width as i32 / 2;
        menu.draw(ctx);
    }
}
