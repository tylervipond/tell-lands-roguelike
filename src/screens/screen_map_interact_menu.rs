use super::ui::ui_map::UIMap;
use super::utils::{get_render_data, get_render_offset};
use crate::components::Position;
use crate::dungeon::{
    constants::{MAP_HEIGHT, MAP_WIDTH},
    dungeon::Dungeon,
    level_utils,
};
use crate::menu_option::MenuOption;
use crate::ui_components::ui_dynamic_menu::UIDynamicMenu;
use rltk::Rltk;
use specs::{Entity, World, WorldExt};

pub struct ScreenMapInteractMenu<'a> {
    menu_options: Box<[&'a MenuOption<'a>]>,
    title: Option<&'a str>,
    cta: Option<&'a str>,
}

impl<'a> ScreenMapInteractMenu<'a> {
    pub fn new(
        menu_options: Box<[&'a MenuOption<'a>]>,
        title: Option<&'a str>,
        cta: Option<&'a str>,
    ) -> Self {
        Self {
            menu_options,
            title,
            cta,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk, world: &mut World) {
        ctx.cls();
        let player_ent = world.fetch::<Entity>();
        let positions = world.read_storage::<Position>();
        let player_position = positions.get(*player_ent).unwrap();

        let dungeon = world.fetch::<Dungeon>();
        let level = dungeon.levels.get(&player_position.level).unwrap();
        let render_data = get_render_data(world);
        let positions = world.read_storage::<Position>();
        let player_position = positions.get(*player_ent).unwrap();
        let (center_x, center_y) =
            level_utils::idx_xy(level.width as i32, player_position.idx as i32);
        let render_offset = get_render_offset(center_x, center_y);
        UIMap::new(level, &render_data, render_offset).draw(ctx);

        let mut menu = UIDynamicMenu::new(0, 0, &self.menu_options, self.cta, self.title);
        menu.y = (MAP_HEIGHT / 2 - menu.height / 2) as i32;
        menu.x = (MAP_WIDTH / 2 - menu.width / 2) as i32;
        menu.draw(ctx);
    }
}
