use super::constants::{MAP_HEIGHT, MAP_WIDTH};
use super::screen_map_generic::ScreenMapGeneric;
use crate::menu_option::MenuOption;
use crate::ui_components::ui_dynamic_menu::UIDynamicMenu;
use rltk::{Console, Rltk};
use specs::World;

pub struct ScreenMapMenu<'a> {
    menu_options: &'a Vec<MenuOption<'a>>,
    title: &'a str,
    cta: &'a str,
}

impl<'a> ScreenMapMenu<'a> {
    pub fn new(menu_options: &'a Vec<MenuOption<'a>>, title: &'a str, cta: &'a str) -> Self {
        Self {
            menu_options,
            title,
            cta,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk, world: &mut World) {
        ctx.cls();
        ScreenMapGeneric::new().draw(ctx, world);
        let title = self.title.to_owned();
        let cta = self.cta.to_owned();
        let mut menu = UIDynamicMenu::new(0, 0, &self.menu_options, Some(cta), Some(title));
        menu.y = (MAP_HEIGHT / 2 - menu.height / 2) as i32;
        menu.x = (MAP_WIDTH / 2 - menu.width / 2) as i32;
        menu.draw(ctx);
    }
}
