use super::ui::ui_hud::UIHud;
use super::ui::ui_map::{RenderData, UIMap};
use super::ui::ui_mouse_pos::UIMousePos;
use crate::components::{
    combat_stats::CombatStats, dungeon_level::DungeonLevel, hidden::Hidden, position::Position,
    renderable::Renderable,
};
use crate::dungeon::dungeon::Dungeon;
use crate::game_log::GameLog;
use crate::ranged;
use rltk::{Point, Rltk, BLUE, CYAN, RGB};
use specs::{Entity, Join, World, WorldExt};

pub struct ScreenMapTargeting<'a> {
    range: i32,
    target: Option<&'a Point>,
}

impl<'a> ScreenMapTargeting<'a> {
    pub fn new(range: i32, target: Option<&'a Point>) -> Self {
        Self { range, target }
    }
    pub fn draw(&self, ctx: &mut Rltk, world: &mut World) {
        let log = world.fetch::<GameLog>();
        let player_ent = world.fetch::<Entity>();
        let levels = world.read_storage::<DungeonLevel>();
        let player_level = levels.get(*player_ent).unwrap();
        let combat_stats = world.read_storage::<CombatStats>();
        let player_stats = combat_stats.get(*player_ent).unwrap();

        let positions = world.read_storage::<Position>();
        let hidden = world.read_storage::<Hidden>();
        let (mouse_x, mouse_y) = ctx.mouse_pos();
        let renderables = world.read_storage::<Renderable>();
        let dungeon = world.fetch::<Dungeon>();
        let map = dungeon.maps.get(&player_level.level).unwrap();
        let renderables = (&positions, &renderables, &levels, !&hidden)
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
        ctx.cls();
        UIMap::new(map, &renderables).draw(ctx);
        UIHud::new(
            player_level.level,
            player_stats.hp,
            player_stats.max_hp,
            &log.entries,
        )
        .draw(ctx);
        let visible_tiles = ranged::get_visible_tiles_in_range(world, self.range);
        visible_tiles
            .iter()
            .for_each(|tile| ctx.set_bg(tile.x, tile.y, RGB::named(BLUE)));
        UIMousePos::new(mouse_x, mouse_y).draw(ctx);
        if let Some(target) = self.target {
            ctx.set_bg(target.x, target.y, RGB::named(CYAN))
        }
    }
}
