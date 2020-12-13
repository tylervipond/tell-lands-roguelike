use super::constants::{MAP_COUNT, MAP_HEIGHT, MAP_WIDTH};
use super::level_utils;
use super::room::Room;
use super::tile_type::TileType;
use rltk::{Algorithm2D, BaseMap, DistanceAlg::Pythagoras, Point, SmallVec};
use serde::{Deserialize, Serialize};
use specs::Entity;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Level {
    pub height: u8,
    pub width: u8,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Room>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub lit_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: u8,
    pub stairs_down: Option<usize>,
    pub stairs_up: Option<usize>,
    pub exit: Option<usize>,
    #[serde(skip_serializing, skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Level {
    pub fn new(depth: u8) -> Self {
        Self {
            tiles: vec![TileType::Wall; MAP_COUNT],
            rooms: vec![],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            revealed_tiles: vec![false; MAP_COUNT],
            visible_tiles: vec![false; MAP_COUNT],
            lit_tiles: vec![false; MAP_COUNT],
            blocked: vec![false; MAP_COUNT],
            tile_content: vec![vec![]; MAP_COUNT],
            stairs_down: None,
            stairs_up: None,
            exit: None,
            depth,
        }
    }
    pub fn get_costs_for_tile(&self, idx: usize, diagonal: bool) -> f32 {
        let cost = match diagonal {
            true => 1.45,
            false => 1.0,
        };
        match self.blocked[idx] {
            true => match self.tiles[idx] {
                TileType::Door => cost + 1.0,
                _ => cost + 25.0,
            },
            false => cost,
        }
    }
}

impl BaseMap for Level {
    fn is_opaque(&self, idx: usize) -> bool {
        let tile = self.tiles[idx as usize];
        tile == TileType::Wall || tile == TileType::Door || tile == TileType::Column
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        for (idx, diagonal) in [
            (idx -1, false),
            (idx + 1, false),
            (idx - self.width as usize, false),
            (idx + self.width as usize, false),
            (idx - self.width as usize -1, true),
            (idx - self.width as usize + 1, true),
            (idx + self.width as usize -1, true),
            (idx + self.width as usize + 1, true),
        ]
        .iter()
        {
            if level_utils::is_exit_valid(self, *idx) {
                exits.push((*idx, self.get_costs_for_tile(*idx, *diagonal)))
            }
        }
        exits
    }
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = self.index_to_point2d(idx1);
        let p2 = self.index_to_point2d(idx2);
        Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for Level {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
