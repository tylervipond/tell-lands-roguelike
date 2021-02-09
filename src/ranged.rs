use std::collections::HashSet;

use crate::components::{Position, Viewshed};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use crate::screens::utils::get_render_offset_for_xy;
use rltk::Rltk;
use specs::{Entity, World, WorldExt};

pub fn get_visible_tiles_in_range(world: &World, range: u32) -> HashSet<usize> {
    let player_ent = world.fetch::<Entity>();
    let positions = world.read_storage::<Position>();
    let player_position = positions.get(*player_ent).unwrap();
    let viewsheds = world.read_storage::<Viewshed>();
    let dungeon = world.fetch::<Dungeon>();
    let level = dungeon.get_level(player_position.level).unwrap();
    let player_viewshed = viewsheds.get(*player_ent).unwrap();
    player_viewshed
        .visible_tiles
        .iter()
        .filter(|tile_idx| {
            level_utils::get_distance_between_idxs(
                level,
                player_position.idx,
                *(*tile_idx) as usize,
            ) <= range as f32
        })
        .cloned()
        .collect()
}

pub fn get_target<'a>(world: &World, ctx: &mut Rltk, tiles: &'a HashSet<usize>) -> Option<usize> {
    let player_ent = world.fetch::<Entity>();
    let dungeon = world.fetch::<Dungeon>();
    let positions = world.read_storage::<Position>();
    let player_position = positions.get(*player_ent).unwrap();
    let level_width = dungeon.get_level(player_position.level).unwrap().width;
    let (center_x, center_y) = level_utils::idx_xy(level_width as u32, player_position.idx);
    let (mouse_x, mouse_y) = ctx.mouse_pos();
    let (offset_x, offset_y) = get_render_offset_for_xy(center_x, center_y, mouse_x, mouse_y);
    let idx = level_utils::xy_idx(level_width as u32, offset_x, offset_y);
    match tiles.contains(&idx) {
        true => Some(idx),
        false => None,
    }
}
