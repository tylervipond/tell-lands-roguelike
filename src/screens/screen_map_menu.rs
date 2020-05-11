use super::ui::{
    ui_hud::UIHud,
    ui_map::{RenderData, UIMap},
};
use crate::components::{
    combat_stats::CombatStats, dungeon_level::DungeonLevel, hidden::Hidden, position::Position,
    renderable::Renderable,
};
use crate::dungeon::{
    constants::{MAP_HEIGHT, MAP_WIDTH},
    dungeon::Dungeon,
    level_utils,
};
use crate::menu_option::MenuOption;
use crate::services::GameLog;
use crate::ui_components::ui_dynamic_menu::UIDynamicMenu;
use rltk::Rltk;
use specs::{Entity, Join, World, WorldExt};

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

        let positions = world.read_storage::<Position>();
        let hidden = world.read_storage::<Hidden>();
        let renderables = world.read_storage::<Renderable>();
        let dungeon = world.fetch::<Dungeon>();
        let level = dungeon.levels.get(&player_level.level).unwrap();
        let renderables = (&positions, &renderables, &levels, !&hidden)
            .join()
            .filter(|(p, _r, l, _h)| {
                let idx = level_utils::xy_idx(&level, p.x, p.y) as usize;
                return l.level == player_level.level && level.visible_tiles[idx];
            })
            .map(|(p, r, _l, _h)| RenderData {
                x: p.x,
                y: p.y,
                fg: r.fg,
                bg: r.bg,
                glyph: r.glyph,
                layer: r.layer,
            })
            .collect::<Vec<RenderData>>();
        ctx.cls();
        UIMap::new(level, &renderables).draw(ctx);
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
