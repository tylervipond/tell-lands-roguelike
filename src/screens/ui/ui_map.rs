use crate::dungeon::{level::Level, level_utils, tile_type::TileType};
use crate::screens::constants::MAP_WIDTH;
use rltk::{Rltk, RGB};

pub struct RenderData {
    pub x: i32,
    pub y: i32,
    pub fg: RGB,
    pub bg: RGB,
    pub layer: i32,
    pub glyph: u16,
}

pub fn is_revealed_and_wall(level: &Level, x: i32, y: i32) -> bool {
    let idx = level_utils::xy_idx(level, x, y) as usize;
    match level.tiles.get(idx) {
        Some(tile) => *tile == TileType::Wall && level.revealed_tiles[idx],
        None => false,
    }
}

pub fn get_wall_tile(level: &Level, x: i32, y: i32) -> u16 {
    let mut mask: u8 = 0;
    if is_revealed_and_wall(level, x, y - 1) {
        mask += 1;
    }
    if is_revealed_and_wall(level, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(level, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(level, x + 1, y) {
        mask += 8;
    }
    match mask {
        0 => 9,
        1 | 2 | 3 => 186,
        4 | 8 | 12 => 205,
        5 => 188,
        6 => 187,
        7 => 185,
        9 => 200,
        10 => 201,
        11 => 204,
        13 => 202,
        14 => 203,
        15 => 206,
        _ => 35,
    }
}

pub struct UIMap<'a> {
    level: &'a Level,
    renderables: &'a Vec<RenderData>,
}

impl<'a> UIMap<'a> {
    pub fn new(level: &'a Level, renderables: &'a Vec<RenderData>) -> Self {
        Self { level, renderables }
    }

    pub fn draw(&mut self, ctx: &mut Rltk) {
        for (i, tile) in self.level.tiles.iter().enumerate() {
            if self.level.revealed_tiles[i] {
                let x = i % MAP_WIDTH as usize;
                let y = (i - (i % MAP_WIDTH as usize)) / MAP_WIDTH as usize;
                let character = match tile {
                    TileType::Floor => rltk::to_cp437('.'),
                    TileType::Wall => get_wall_tile(&self.level, x as i32, y as i32),
                    TileType::DownStairs => rltk::to_cp437('>'),
                    TileType::UpStairs => rltk::to_cp437('<'),
                    TileType::Door => rltk::to_cp437('D'),
                    TileType::Exit => 219,
                };
                let foreground_color = match self.level.visible_tiles[i] {
                    true => rltk::GREEN,
                    false => rltk::WHITE,
                };
                ctx.set(
                    x as i32,
                    y as i32,
                    RGB::named(foreground_color),
                    RGB::named(rltk::BLACK),
                    character,
                )
            }
        }
        for r in self.renderables.iter() {
            ctx.set(r.x, r.y, r.fg, r.bg, r.glyph);
        }
    }
}
