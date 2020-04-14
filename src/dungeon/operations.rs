use super::level::Level;
use super::rect::Rect;
use super::tile_type::TileType;
use rltk::{DistanceAlg::Pythagoras, Point, RandomNumberGenerator};
use specs::Entity;
use std::cmp::{max, min};
use std::collections::HashMap;

pub fn xy_idx(level: &Level, x: i32, y: i32) -> i32 {
    y * level.width + x
}

pub fn idx_xy(level: &Level, idx: i32) -> (i32, i32) {
    (idx % level.width as i32, idx / level.width as i32)
}

fn set_tile_to_floor(level: &mut Level, idx: usize) {
    level.tiles[idx] = TileType::Floor;
}

pub fn add_room_to_map(level: &mut Level, room: &Rect) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            let idx = xy_idx(level, x, y) as usize;
            set_tile_to_floor(level, idx);
        }
    }
}

pub fn add_horizontal_tunnel(level: &mut Level, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(level, x, y);
        if idx > 0 && idx < level.tiles.len() as i32 {
            set_tile_to_floor(level, idx as usize);
        }
    }
}

pub fn add_vertical_tunnel(level: &mut Level, x: i32, y1: i32, y2: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(level, x, y);
        if idx > 0 && idx < level.tiles.len() as i32 {
            set_tile_to_floor(level, idx as usize);
        }
    }
}

pub fn add_exit(level: &mut Level) {
    let exit_position = level.rooms[0].center();
    let exit_idx = xy_idx(level, exit_position.0, exit_position.1);
    level.tiles[exit_idx as usize] = TileType::Exit;
    level.exit = Some(Point::new(exit_position.0, exit_position.1));
}

pub fn add_down_stairs(level: &mut Level) {
    let stairs_position = level.rooms[level.rooms.len() - 1].center();
    let stairs_idx = xy_idx(level, stairs_position.0, stairs_position.1);
    level.tiles[stairs_idx as usize] = TileType::DownStairs;
    level.stairs_down = Some(Point::new(stairs_position.0, stairs_position.1));
}

pub fn add_up_stairs(level: &mut Level) {
    let stairs_position = level.rooms[0].center();
    let stairs_idx = xy_idx(level, stairs_position.0, stairs_position.1);
    level.tiles[stairs_idx as usize] = TileType::UpStairs;
    level.stairs_up = Some(Point::new(stairs_position.0, stairs_position.1));
}

pub fn entities_at_xy(level: &Level, x: i32, y: i32) -> Vec<Entity> {
    let idx = xy_idx(level, x, y);
    level.tile_content[idx as usize].to_vec()
}

pub fn populate_blocked(level: &mut Level) {
    for (i, tile) in level.tiles.iter_mut().enumerate() {
        level.blocked[i] = *tile == TileType::Wall;
    }
}

pub fn point_not_in_map(level: &Level, point: &Point) -> bool {
    point.x < 0 || point.x >= level.width || point.y < 0 || point.y >= level.height
}

pub fn is_exit_valid(level: &Level, x: i32, y: i32) -> bool {
    if point_not_in_map(level, &Point::new(x, y)) {
        return false;
    }
    let idx = xy_idx(level, x, y);
    !level.blocked[idx as usize]
}

pub fn clear_content_index(level: &mut Level) {
    for content in level.tile_content.iter_mut() {
        content.clear();
    }
}

pub fn add_corridor(level: &mut Level, rng: &mut RandomNumberGenerator, from: Point, to: Point) {
    if rng.range(0, 2) == 1 {
        add_horizontal_tunnel(level, from.x, to.x, from.y);
        add_vertical_tunnel(level, to.x, to.y, from.y);
    } else {
        add_vertical_tunnel(level, from.x, from.y, to.y);
        add_horizontal_tunnel(level, from.x, to.x, to.y);
    }
}

pub fn add_nearest_neighbor_corridors(level: &mut Level, rng: &mut RandomNumberGenerator) {
    let mut connected: HashMap<usize, Vec<(Point, Point)>> = HashMap::new();
    for (i, room) in level.rooms.iter().enumerate() {
        let room_center_point = Point::from(room.center());
        let mut room_distance: Vec<(usize, f32, Point)> = level
            .rooms
            .iter()
            .enumerate()
            .filter(|(j, _)| &i != j && !connected.contains_key(&j))
            .map(|(j, other_room)| {
                let other_room_center_point = Point::from(other_room.center());
                let distance = Pythagoras.distance2d(room_center_point, other_room_center_point);
                (j, distance, other_room_center_point)
            })
            .collect();
        room_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let points: Vec<(Point, Point)> = room_distance
            .iter()
            .take(2)
            .map(|(_, __, other_room_center_point)| (room_center_point, *other_room_center_point))
            .collect();
        connected.insert(i, points);
    }
    for points_pairs in connected.values() {
        for point_pair in points_pairs {
            add_corridor(level, rng, point_pair.0, point_pair.1);
        }
    }
}

pub fn create_bsp_level(depth: i32) -> Level {
    const MIN_ROOM_SIZE: i32 = 5;
    let mut level = Level::new(depth);
    let mut rng = RandomNumberGenerator::new();
    let mut rects = Vec::new();
    rects.push(Rect::new(2, 2, level.width - 4, level.height - 4));
    for _ in 0..100 {
        let random_index = rng.range(0, rects.len() as i32);
        let rect = rects[random_index as usize];
        let width = rect.x2 - rect.x1;
        let height = rect.y2 - rect.y1;
        if width > MIN_ROOM_SIZE * 2 && width > height {
            let x = max(
                rect.x1 + MIN_ROOM_SIZE,
                rng.range(rect.x1, rect.x2 - MIN_ROOM_SIZE),
            );
            rects.remove(random_index as usize);
            rects.push(Rect::new(rect.x1, rect.y1, x - rect.x1, rect.y2 - rect.y1));
            rects.push(Rect::new(x, rect.y1, rect.x2 - x, rect.y2 - rect.y1));
        } else if height > MIN_ROOM_SIZE * 2 && height > width {
            let y = max(
                rect.y1 + MIN_ROOM_SIZE,
                rng.range(rect.y1, rect.y2 - MIN_ROOM_SIZE),
            );
            rects.remove(random_index as usize);
            rects.push(Rect::new(rect.x1, rect.y1, rect.x2 - rect.x1, y - rect.y1));
            rects.push(Rect::new(rect.x1, y, rect.x2 - rect.x1, rect.y2 - y));
        }
    }
    let room_count = rng.range(2, rects.len() as i32);
    let rooms: Vec<Rect> = (0..room_count)
        .map(|_| {
            let idx = rng.range(0, rects.len() as i32);
            rects.remove(idx as usize)
        })
        .collect();
    rooms.iter().for_each(|r| add_room_to_map(&mut level, r));
    level.rooms = rooms;
    add_nearest_neighbor_corridors(&mut level, &mut rng);
    level
}
