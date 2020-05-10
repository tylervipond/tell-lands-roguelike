use super::{level::Level, rect::Rect, tile_type::TileType};
use rltk::{Point, RandomNumberGenerator};
use specs::Entity;

pub fn xy_idx(level: &Level, x: i32, y: i32) -> i32 {
    y * level.width as i32 + x
}

pub fn idx_xy(level: &Level, idx: i32) -> (i32, i32) {
    (idx % level.width as i32, idx / level.width as i32)
}

pub fn get_cardinal_idx(idx: i32, level: &Level) -> (i32, i32, i32, i32) {
    let (x, y) = idx_xy(level, idx);
    let n = xy_idx(level, x, y - 1);
    let e = xy_idx(level, x + 1, y);
    let s = xy_idx(level, x, y + 1);
    let w = xy_idx(level, x - 1, y);
    (n, e, s, w)
}

pub fn get_ordinal_idx(idx: i32, level: &Level) -> (i32, i32, i32, i32) {
    let (x, y) = idx_xy(level, idx);
    let ne = xy_idx(level, x + 1, y - 1);
    let se = xy_idx(level, x + 1, y + 1);
    let sw = xy_idx(level, x - 1, y + 1);
    let nw = xy_idx(level, x - 1, y - 1);
    (ne, se, sw, nw)
}

pub fn tile_is_wall_adjacent(idx: i32, level: &Level) -> bool {
    let (x, y) = idx_xy(level, idx);
    tile_at_xy_is_wall(level, x - 1, y)
        || tile_at_xy_is_wall(level, x + 1, y)
        || tile_at_xy_is_wall(level, x, y - 1)
        || tile_at_xy_is_wall(level, x, y + 1)
}

pub fn get_tile_at_xy(level: &Level, x: i32, y: i32) -> TileType {
    level.tiles[xy_idx(level, x, y) as usize]
}

pub fn tile_at_xy_is_wall(level: &Level, x: i32, y: i32) -> bool {
    get_tile_at_xy(level, x, y) == TileType::Wall
}

pub fn set_tile_to_floor(level: &mut Level, idx: usize) {
    level.tiles[idx] = TileType::Floor;
}

pub fn set_tile_to_door(level: &mut Level, idx: usize) {
    level.tiles[idx] = TileType::Door;
}

pub fn entities_at_xy(level: &Level, x: i32, y: i32) -> Vec<Entity> {
    let idx = xy_idx(level, x, y);
    level.tile_content[idx as usize].to_vec()
}

pub fn populate_blocked(level: &mut Level) {
    for (i, tile) in level.tiles.iter_mut().enumerate() {
        level.blocked[i] = *tile == TileType::Wall || *tile == TileType::Door;
    }
}

pub fn tile_is_blocked(idx: i32, level: &Level) -> bool {
    level.blocked[idx as usize]
}

pub fn point_not_in_map(level: &Level, point: &Point) -> bool {
    point.x < 0 || point.x >= level.width as i32 || point.y < 0 || point.y >= level.height as i32
}

pub fn is_exit_valid(level: &Level, x: i32, y: i32) -> bool {
    if point_not_in_map(level, &Point::new(x, y)) {
        return false;
    }
    let idx = xy_idx(level, x, y);
    !tile_is_blocked(idx, level)
}

pub fn clear_content_index(level: &mut Level) {
    for content in level.tile_content.iter_mut() {
        content.clear();
    }
}

pub fn get_walkable_tiles_in_rect(rect: &Rect, level: &Level) -> Vec<i32> {
    (rect.y1 + 1..rect.y2)
        .map(|y| {
            (rect.x1 + 1..rect.x2)
                .map(|x| (x, y.clone()))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .map(|(x, y)| xy_idx(level, x, y))
        .filter(|idx| level.tiles[*idx as usize] == TileType::Floor && !tile_is_blocked(*idx, level))
        .collect()
}

pub fn get_random_wall_adjacent_spawn_point(
    rect: &Rect,
    level: &Level,
    rng: &mut RandomNumberGenerator,
) -> u16 {
    let walkable_tiles_in_rect_with_wall: Vec<i32> = get_walkable_tiles_in_rect(rect, level)
        .into_iter()
        .filter(|idx| tile_is_wall_adjacent(*idx, level))
        .collect();
    let selected_index = rng.range(0, walkable_tiles_in_rect_with_wall.len());
    walkable_tiles_in_rect_with_wall[selected_index] as u16
}

pub fn get_random_spawn_point(rect: &Rect, level: &Level, rng: &mut RandomNumberGenerator) -> u16 {
    let walkable_tiles_in_rect = get_walkable_tiles_in_rect(rect, level);
    let selected_index = rng.range(0, walkable_tiles_in_rect.len());
    walkable_tiles_in_rect[selected_index] as u16
}

pub fn get_spawn_points(
    rect: &Rect,
    level: &Level,
    rng: &mut RandomNumberGenerator,
    count: i32,
) -> Vec<u16> {
    (0..count)
        .map(|_| get_random_spawn_point(rect, level, rng))
        .collect()
}

pub fn get_wall_adjacent_spawn_points(
    rect: &Rect,
    level: &Level,
    rng: &mut RandomNumberGenerator,
    count: i32,
) -> Vec<u16> {
    (0..count)
        .map(|_| get_random_wall_adjacent_spawn_point(rect, level, rng))
        .collect()
}
