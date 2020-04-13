use super::constants::{MAP_COUNT, MAP_HEIGHT, MAP_WIDTH};
use super::operations::{idx_xy, is_exit_valid};
use super::room::Room;
use super::tile_type::TileType;
use rltk::{Algorithm2D, BaseMap, DistanceAlg::Pythagoras, Point, SmallVec};
use serde::{Deserialize, Serialize};
use specs::Entity;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Level {
    pub height: i32,
    pub width: i32,
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Room>,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub stairs_down: Option<Point>,
    pub stairs_up: Option<Point>,
    pub exit: Option<Point>,
    #[serde(skip_serializing, skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Level {
    pub fn new(depth: i32) -> Self {
        Self {
            tiles: vec![TileType::Wall; MAP_COUNT],
            rooms: vec![],
            width: MAP_WIDTH as i32,
            height: MAP_HEIGHT as i32,
            revealed_tiles: vec![false; MAP_COUNT],
            visible_tiles: vec![false; MAP_COUNT],
            blocked: vec![false; MAP_COUNT],
            tile_content: vec![vec![]; MAP_COUNT],
            stairs_down: None,
            stairs_up: None,
            exit: None,
            depth,
        }
    }
}

impl BaseMap for Level {
    fn is_opaque(&self, idx: usize) -> bool {
        let tile = self.tiles[idx as usize];
        tile == TileType::Wall || tile == TileType::Door
    }
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let (x, y) = idx_xy(self, idx as i32);
        if is_exit_valid(self, x - 1, y) {
            exits.push((idx - 1, 1.0))
        }
        if is_exit_valid(self, x + 1, y) {
            exits.push((idx + 1, 1.0))
        }
        if is_exit_valid(self, x, y - 1) {
            exits.push((idx - self.width as usize, 1.0))
        }
        if is_exit_valid(self, x, y + 1) {
            exits.push((idx + self.width as usize, 1.0))
        }
        if is_exit_valid(self, x - 1, y - 1) {
            exits.push(((idx - self.width as usize) - 1, 1.45))
        }
        if is_exit_valid(self, x + 1, y - 1) {
            exits.push(((idx - self.width as usize) + 1, 1.45))
        }
        if is_exit_valid(self, x - 1, y + 1) {
            exits.push(((idx + self.width as usize) - 1, 1.45))
        }
        if is_exit_valid(self, x + 1, y + 1) {
            exits.push(((idx + self.width as usize) + 1, 1.45))
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
