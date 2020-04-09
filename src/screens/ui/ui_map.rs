use crate::map::{tile_type::TileType, Map};
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

pub fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let map_idx = map.xy_idx(x, y) as usize;
    match map.tiles.get(map_idx) {
        Some(tile) => *tile == TileType::Wall && map.revealed_tiles[map_idx],
        None => false,
    }
}

pub fn get_wall_tile(map: &Map, x: i32, y: i32) -> u16 {
    let mut mask: u8 = 0;
    if is_revealed_and_wall(map, x, y - 1) {
        mask += 1;
    }
    if is_revealed_and_wall(map, x, y + 1) {
        mask += 2;
    }
    if is_revealed_and_wall(map, x - 1, y) {
        mask += 4;
    }
    if is_revealed_and_wall(map, x + 1, y) {
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
    map: &'a Map,
    renderables: &'a Vec<RenderData>,
}

impl<'a> UIMap<'a> {
    pub fn new(map: &'a Map, renderables: &'a Vec<RenderData>) -> Self {
        Self { map, renderables }
    }

    pub fn draw(&mut self, ctx: &mut Rltk) {
        for (i, tile) in self.map.tiles.iter().enumerate() {
            if self.map.revealed_tiles[i] {
                let x = i % MAP_WIDTH as usize;
                let y = (i - (i % MAP_WIDTH as usize)) / MAP_WIDTH as usize;
                let character = match tile {
                    TileType::Floor => rltk::to_cp437('.'),
                    TileType::Wall => get_wall_tile(&self.map, x as i32, y as i32),
                    TileType::DownStairs => rltk::to_cp437('>'),
                    TileType::UpStairs => rltk::to_cp437('<'),
                    TileType::Exit => 219,
                };
                let foreground_color = match self.map.visible_tiles[i] {
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
