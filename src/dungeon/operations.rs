use super::level::Level;
use super::rect::Rect;
use super::tile_type::TileType;
use rltk::{Point, RandomNumberGenerator};
use specs::Entity;
use std::cmp::{max, min};

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
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
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

pub fn create_basic_map(depth: i32) -> Level {
    let mut level = Level::new(depth);

    const MAX_ROOMS: i32 = 30;
    const ROOM_MIN_SIZE: i32 = 6;
    const ROOM_MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..MAX_ROOMS {
        let w = rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
        let h = rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
        let x = rng.roll_dice(1, (level.width - w - 1) - 1);
        let y = rng.roll_dice(1, (level.height - h - 1) - 1);
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in level.rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false;
            }
        }
        if ok {
            add_room_to_map(&mut level, &new_room);
            if !level.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = level.rooms[level.rooms.len() - 1].center();
                if rng.range(0, 1) == 1 {
                    add_horizontal_tunnel(&mut level, prev_x as i32, new_x as i32, prev_y as i32);
                    add_vertical_tunnel(&mut level, new_x as i32, new_y as i32, prev_y as i32);
                } else {
                    add_vertical_tunnel(&mut level, prev_x as i32, prev_y as i32, new_y as i32);
                    add_horizontal_tunnel(&mut level, prev_x as i32, new_x as i32, new_y as i32);
                }
            }
            level.rooms.push(new_room);
        }
    }
    level
}
