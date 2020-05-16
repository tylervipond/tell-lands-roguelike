use super::ui::ui_map::RenderData;
use crate::components::{
    dungeon_level::DungeonLevel, hidden::Hidden, on_fire::OnFire, position::Position,
    renderable::Renderable,
};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use rltk::{BLACK, ORANGE, RGB};
use specs::{Entity, Join, World, WorldExt};

pub fn get_render_data(world: &World) -> Vec<RenderData> {
    let positions = world.read_storage::<Position>();
    let hidden = world.read_storage::<Hidden>();
    let renderables = world.read_storage::<Renderable>();
    let levels = world.read_storage::<DungeonLevel>();
    let on_fire = world.read_storage::<OnFire>();
    let player_ent = world.fetch::<Entity>();
    let player_level = levels.get(*player_ent).unwrap();
    let dungeon = world.fetch::<Dungeon>();
    let level = dungeon.levels.get(&player_level.level).unwrap();
    let mut render_data: Vec<RenderData> = (
        &positions,
        &renderables,
        &levels,
        (&on_fire).maybe(),
        (&hidden).maybe(),
    )
        .join()
        .filter(|(p, _r, l, _f, h)| {
            let idx = level_utils::xy_idx(&level, p.x, p.y) as usize;
            let is_visible = match h {
                Some(h) => h.found_by.contains(&*player_ent),
                None => true,
            };
            return l.level == player_level.level && level.visible_tiles[idx] && is_visible;
        })
        .map(|(p, r, _l, f, _h)| {
            let (fg, bg) = match f {
                Some(_) => (RGB::named(BLACK), RGB::named(ORANGE)),
                None => (r.fg, r.bg),
            };
            RenderData {
                x: p.x,
                y: p.y,
                fg,
                bg,
                glyph: r.glyph,
                layer: r.layer,
            }
        })
        .collect();
    render_data.sort_unstable_by(|a, b| b.layer.cmp(&a.layer));
    return render_data;
}
