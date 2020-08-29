use super::{level::Level, rect::Rect, tile_type::TileType};
use rltk::{DistanceAlg::Pythagoras, Point, RandomNumberGenerator};
use specs::Entity;

pub fn xy_idx(level: &Level, x: i32, y: i32) -> i32 {
    y * level.width as i32 + x
}

pub fn idx_xy(level: &Level, idx: i32) -> (i32, i32) {
    (idx % level.width as i32, idx / level.width as i32)
}

pub fn get_tile_at_xy(level: &Level, x: i32, y: i32) -> Option<&TileType> {
    level.tiles.get(xy_idx(level, x, y) as usize)
}

pub fn tile_at_xy_is_wall(level: &Level, x: i32, y: i32) -> bool {
    get_tile_at_xy(level, x, y) == Some(&TileType::Wall)
}

pub fn tile_at_xy_is_door(level: &Level, x: i32, y: i32) -> bool {
    get_tile_at_xy(level, x, y) == Some(&TileType::Door)
}

pub fn tile_is_door_adjacent(level: &Level, x: i32, y: i32) -> bool {
    tile_at_xy_is_door(level, x - 1, y)
        || tile_at_xy_is_door(level, x, y - 1)
        || tile_at_xy_is_door(level, x + 1, y)
        || tile_at_xy_is_door(level, x, y + 1)
}

pub fn tile_is_between_walls_vertical(level: &Level, x: i32, y: i32) -> bool {
    tile_at_xy_is_wall(level, x, y - 1) && tile_at_xy_is_wall(level, x, y + 1)
}

pub fn tile_is_between_walls_horizontal(level: &Level, x: i32, y: i32) -> bool {
    tile_at_xy_is_wall(level, x - 1, y) && tile_at_xy_is_wall(level, x + 1, y)
}

pub fn tile_is_between_walls(level: &Level, x: i32, y: i32) -> bool {
    tile_is_between_walls_horizontal(level, x, y) || tile_is_between_walls_vertical(level, x, y)
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
        level.blocked[i] = *tile == TileType::Wall
            || *tile == TileType::Door
            || *tile == TileType::Column
            || *tile == TileType::Ledge;
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
        .filter(|idx| {
            level.tiles[*idx as usize] == TileType::Floor && !tile_is_blocked(*idx, level)
        })
        .collect()
}

pub fn filter_water_from_tiles(tiles: Vec<i32>, level: &Level) -> Vec<i32> {
    tiles
        .iter()
        .filter(|idx| level.tiles[**idx as usize] != TileType::WaterDeep)
        .map(|idx| idx.to_owned())
        .collect()
}

pub fn get_random_spawn_point(rect: &Rect, level: &Level, rng: &mut RandomNumberGenerator) -> u16 {
    let walkable_tiles_in_rect =
        filter_water_from_tiles(get_walkable_tiles_in_rect(rect, level), level);
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

pub fn get_all_unblocked_tiles_in_radius(
    level: &Level,
    center_idx: i32,
    radius_length: i32,
) -> Vec<i32> {
    let center_xy = idx_xy(level, center_idx);
    let center_point = Point::new(center_xy.0, center_xy.1);
    return level
        .tiles
        .iter()
        .enumerate()
        .map(|(tile_idx, _tile_type)| tile_idx as i32)
        .filter(|tile_idx| {
            let this_xy = idx_xy(level, *tile_idx);
            let this_point = Point::new(this_xy.0, this_xy.1);
            Pythagoras.distance2d(center_point, this_point) <= radius_length as f32
                && !tile_is_blocked(*tile_idx, level)
        })
        .collect();
}

pub fn get_all_spawnable_tiles_in_radius(
    level: &Level,
    center_idx: i32,
    radius_length: i32,
) -> Vec<i32> {
    filter_water_from_tiles(
        get_all_unblocked_tiles_in_radius(level, center_idx, radius_length),
        level,
    )
}
