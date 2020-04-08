use super::ui::ui_map::RenderData;
use super::ui::ui_map_screen::UIMapScreen;
use crate::components::{
    combat_stats::CombatStats, dungeon_level::DungeonLevel, hidden::Hidden, name::Name,
    position::Position, renderable::Renderable,
};
use crate::dungeon::dungeon::Dungeon;
use crate::game_log::GameLog;
use rltk::{Console, Rltk};
use specs::{Entity, Join, World, WorldExt};

pub struct ScreenMapGeneric {}

impl ScreenMapGeneric {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Rltk, world: &mut World) {
        let log = world.fetch::<GameLog>();
        let player_ent = world.fetch::<Entity>();
        let levels = world.read_storage::<DungeonLevel>();
        let player_level = levels.get(*player_ent).unwrap();
        let combat_stats = world.read_storage::<CombatStats>();
        let player_stats = combat_stats.get(*player_ent).unwrap();

        let names = world.read_storage::<Name>();
        let positions = world.read_storage::<Position>();
        let hidden = world.read_storage::<Hidden>();
        let (mouse_x, mouse_y) = ctx.mouse_pos();
        let tool_tip_lines: Vec<String> =
            (&names, &positions, !&hidden)
                .join()
                .fold(vec![], |mut acc, (name, position, _)| {
                    if position.x == mouse_x && position.y == mouse_y {
                        acc.push(name.name.to_owned());
                    }
                    acc
                });
        let renderables = world.read_storage::<Renderable>();
        let dungeon = world.fetch::<Dungeon>();
        let map = dungeon.maps.get(&player_level.level).unwrap();
        let mut renderables = (&positions, &renderables, &levels, !&hidden)
            .join()
            .filter(|(p, _r, l, _h)| {
                return l.level == player_level.level
                    && map.visible_tiles[map.xy_idx(p.x, p.y) as usize];
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
        renderables.sort_unstable_by(|a, b| b.layer.cmp(&a.layer));
        ctx.cls();
        UIMapScreen::new(
            mouse_x,
            mouse_y,
            &tool_tip_lines,
            &log.entries,
            player_level.level,
            player_stats.hp,
            player_stats.max_hp,
            map,
            &renderables,
        )
        .draw(ctx);
    }
}
