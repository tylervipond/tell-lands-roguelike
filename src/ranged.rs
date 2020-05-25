use crate::components::viewshed::Viewshed;
use rltk::{DistanceAlg::Pythagoras, Point, Rltk};
use specs::{Entity, World, WorldExt};

pub fn get_visible_tiles_in_range(ecs: &World, range: i32) -> Vec<Point> {
    let player_ent = ecs.fetch::<Entity>();
    let player_pos = ecs.fetch::<Point>();
    let viewsheds = ecs.read_storage::<Viewshed>();
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

pub fn get_target<'a>(ctx: &mut Rltk, tiles: &'a Vec<Point>) -> Option<&'a Point> {
    let (x, y) = ctx.mouse_pos();
    tiles.iter().find(|idx| idx.x == x && idx.y == y)
}
