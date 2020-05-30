use crate::components::viewshed::Viewshed;
use crate::screens::utils::get_render_offset;
use rltk::{DistanceAlg::Pythagoras, Point, Rltk};
use specs::{Entity, World, WorldExt};

pub fn get_visible_tiles_in_range(world: &World, range: i32) -> Vec<Point> {
    let player_ent = world.fetch::<Entity>();
    let player_pos = world.fetch::<Point>();
    let viewsheds = world.read_storage::<Viewshed>();
    if let Some(visible) = viewsheds.get(*player_ent) {
        return visible
            .visible_tiles
            .iter()
            .cloned()
            .filter(|tile_idx| Pythagoras.distance2d(*player_pos, *tile_idx) <= range as f32)
            .collect();
    }
    vec![]
}

pub fn get_target<'a>(world: &World, ctx: &mut Rltk, tiles: &'a Vec<Point>) -> Option<&'a Point> {
    let render_offset = get_render_offset(world);
    let (x, y) = ctx.mouse_pos();
    let offset_x = x + render_offset.0;
    let offset_y = y + render_offset.1;
    tiles.iter().find(|idx| idx.x == offset_x && idx.y == offset_y)
}
