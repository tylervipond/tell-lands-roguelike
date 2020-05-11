use super::ui::ui_map_screen::UIMapScreen;
use super::utils::get_render_data;
use crate::components::{
    combat_stats::CombatStats, dungeon_level::DungeonLevel, hidden::Hidden, name::Name,
    position::Position,
};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use crate::services::GameLog;
use rltk::Rltk;
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
        let dungeon = world.fetch::<Dungeon>();
        let level = dungeon.levels.get(&player_level.level).unwrap();
        let tool_tip_lines = match level
            .visible_tiles
            .get(level_utils::xy_idx(&level, mouse_x, mouse_y) as usize)
        {
            Some(visible) => match visible {
                true => (&names, &positions, &levels, !&hidden).join().fold(
                    Vec::new(),
                    |mut acc, (name, position, level, _)| {
                        if level.level == player_level.level
                            && position.x == mouse_x
                            && position.y == mouse_y
                        {
                            acc.push(name.name.to_owned());
                        }
                        acc
                    },
                ),
                false => Vec::new(),
            },
            None => Vec::new(),
        };
        let render_data = get_render_data(world);
        ctx.cls();
        UIMapScreen::new(
            mouse_x,
            mouse_y,
            &tool_tip_lines,
            &log.entries,
            player_level.level,
            player_stats.hp,
            player_stats.max_hp,
            level,
            &render_data,
        )
        .draw(ctx);
    }
}
