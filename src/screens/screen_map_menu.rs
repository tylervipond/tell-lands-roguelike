use super::ui::{ui_hud::UIHud, ui_map::UIMap};
use super::utils::get_render_data;
use crate::components::{CombatStats, DungeonLevel};
use crate::dungeon::{
    constants::{MAP_HEIGHT, MAP_WIDTH},
    dungeon::Dungeon,
};
use crate::menu_option::MenuOption;
use crate::services::GameLog;
use crate::ui_components::ui_dynamic_menu::UIDynamicMenu;
use rltk::Rltk;
use specs::{Entity, World, WorldExt};

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
        let log = world.fetch::<GameLog>();
        let player_ent = world.fetch::<Entity>();
        let levels = world.read_storage::<DungeonLevel>();
        let player_level = levels.get(*player_ent).unwrap();
        let combat_stats = world.read_storage::<CombatStats>();
        let player_stats = combat_stats.get(*player_ent).unwrap();

        let dungeon = world.fetch::<Dungeon>();
        let level = dungeon.levels.get(&player_level.level).unwrap();
        let render_data = get_render_data(world);
        ctx.cls();
        UIMap::new(level, &render_data).draw(ctx);
        UIHud::new(
            player_level.level,
            player_stats.hp,
            player_stats.max_hp,
            &log.entries,
        )
        .draw(ctx);
        let title = self.title.to_owned();
        let cta = self.cta.to_owned();
        let mut menu = UIDynamicMenu::new(0, 0, &self.menu_options, Some(cta), Some(title));
        menu.y = (MAP_HEIGHT / 2 - menu.height / 2) as i32;
        menu.x = (MAP_WIDTH / 2 - menu.width / 2) as i32;
        menu.draw(ctx);
    }
}
