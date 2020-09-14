use super::{
    constants::{MAP_HEIGHT, MAP_WIDTH},
    ui::ui_map::RenderData,
};
use crate::components::{DungeonLevel, Hidden, Hiding, OnFire, Position, Renderable};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use rltk::{Point, GREY, ORANGE, RGB};
use specs::{Entity, Join, World, WorldExt};

pub fn get_render_data(world: &World) -> Vec<RenderData> {
    let positions = world.read_storage::<Position>();
    let hidden = world.read_storage::<Hidden>();
    let renderables = world.read_storage::<Renderable>();
    let levels = world.read_storage::<DungeonLevel>();
    let on_fire = world.read_storage::<OnFire>();
    let hiding = world.read_storage::<Hiding>();
    let entities = world.entities();
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
        (&hiding).maybe(),
        &entities,
    )
        .join()
        .filter(|(p, _r, l, _f, h, hiding, entity)| {
            let idx = level_utils::xy_idx(&level, p.x, p.y) as usize;
            let is_visible = match h {
                Some(h) => h.found_by.contains(&*player_ent),
                None => true,
            };
            let hiding = match hiding {
                Some(_) => *entity != *player_ent,
                _ => false,
            };
            return l.level == player_level.level
                && level.visible_tiles[idx]
                && is_visible
                && !hiding;
        })
        .map(|(p, r, _l, f, _h, hiding, entity)| {
            let mut fg = r.fg;
            if f.is_some() {
                fg = RGB::named(ORANGE);
            }
            if hiding.is_some() && entity == *player_ent {
                fg = RGB::named(GREY);
            }
            RenderData {
                x: p.x,
                y: p.y,
                fg,
                bg: r.bg,
                glyph: r.glyph,
                layer: r.layer,
            }
        })
        .collect();
    render_data.sort_unstable_by(|a, b| b.layer.cmp(&a.layer));
    return render_data;
}

pub fn get_render_offset(world: &World) -> (i32, i32) {
    let player_position = world.fetch::<Point>();
    let offset_x = player_position.x - MAP_WIDTH as i32 / 2;
    let offset_y = player_position.y - MAP_HEIGHT as i32 / 2;
    (offset_x, offset_y)
}

pub fn get_render_offset_for_xy(world: &World, xy: (i32, i32)) -> (i32, i32) {
    let render_offset = get_render_offset(world);
    let offset_x = xy.0 + render_offset.0;
    let offset_y = xy.1 + render_offset.1;
    (offset_x, offset_y)
}
