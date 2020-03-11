use crate::map::{idx_xy, rect::Rect, xy_idx, TileType, MAP_COUNT, MAP_HEIGHT, MAP_WIDTH};
use rltk::{Algorithm2D, BaseMap, DistanceAlg::Pythagoras, Point, RandomNumberGenerator};
use serde::{Deserialize, Serialize};
use specs::Entity;
use std::cmp::{max, min};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Map {
  pub height: i32,
  pub width: i32,
  pub tiles: Vec<TileType>,
  pub rooms: Vec<Rect>,
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

impl Map {
  pub fn xy_idx(&self, x: i32, y: i32) -> i32 {
    xy_idx(x, y)
  }

  pub fn idx_xy(&self, idx: i32) -> (i32, i32) {
    idx_xy(idx)
  }

  pub fn entities_at_xy(&self, x: i32, y: i32) -> Vec<Entity> {
    let idx = self.xy_idx(x, y);
    self.tile_content[idx as usize].to_vec()
  }

  pub fn populate_blocked(&mut self) {
    for (i, tile) in self.tiles.iter_mut().enumerate() {
      self.blocked[i] = *tile == TileType::Wall;
    }
  }

  pub fn point_not_in_map(&self, point: &Point) -> bool {
    point.x < 0 || point.x >= self.width || point.y < 0 || point.y >= self.height
  }

  fn is_exit_valid(&self, x: i32, y: i32) -> bool {
    if self.point_not_in_map(&Point::new(x, y)) {
      return false;
    }
    let idx = self.xy_idx(x, y);
    !self.blocked[idx as usize]
  }

  fn set_tile_to_floor(&mut self, idx: usize) {
    self.tiles[idx] = TileType::Floor;
  }

  fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
      let idx = self.xy_idx(x, y);
      if idx > 0 && idx < MAP_COUNT as i32 {
        self.set_tile_to_floor(idx as usize);
      }
    }
  }

  fn add_vertical_tunnel(&mut self, x: i32, y1: i32, y2: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
      let idx = self.xy_idx(x, y);
      if idx > 0 && idx < MAP_COUNT as i32 {
        self.set_tile_to_floor(idx as usize);
      }
    }
  }

  fn add_room_to_map(&mut self, room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
      for x in room.x1 + 1..=room.x2 {
        self.set_tile_to_floor(self.xy_idx(x, y) as usize);
      }
    }
  }

  pub fn clear_content_index(&mut self) {
    for content in self.tile_content.iter_mut() {
      content.clear();
    }
  }

  pub fn add_exit(&mut self) {
    let exit_position = self.rooms[0].center();
    let exit_idx = self.xy_idx(exit_position.0, exit_position.1);
    self.tiles[exit_idx as usize] = TileType::Exit;
    self.exit = Some(Point::new(exit_position.0, exit_position.1));
  }

  pub fn add_down_stairs(&mut self) {
    let stairs_position = self.rooms[self.rooms.len() - 1].center();
    let stairs_idx = self.xy_idx(stairs_position.0, stairs_position.1);
    self.tiles[stairs_idx as usize] = TileType::DownStairs;
    self.stairs_down = Some(Point::new(stairs_position.0, stairs_position.1));
  }

  pub fn add_up_stairs(&mut self) {
    let stairs_position = self.rooms[0].center();
    let stairs_idx = self.xy_idx(stairs_position.0, stairs_position.1);
    self.tiles[stairs_idx as usize] = TileType::UpStairs;
    self.stairs_up = Some(Point::new(stairs_position.0, stairs_position.1));
  }

  pub fn create_basic_map(depth: i32) -> Self {
    let mut map = Map {
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
    };

    const MAX_ROOMS: i32 = 30;
    const ROOM_MIN_SIZE: i32 = 6;
    const ROOM_MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..MAX_ROOMS {
      let w = rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
      let h = rng.range(ROOM_MIN_SIZE, ROOM_MAX_SIZE);
      let x = rng.roll_dice(1, (MAP_WIDTH as i32 - w - 1) - 1);
      let y = rng.roll_dice(1, (MAP_HEIGHT as i32 - h - 1) - 1);
      let new_room = Rect::new(x, y, w, h);
      let mut ok = true;
      for other_room in map.rooms.iter() {
        if new_room.intersect(other_room) {
          ok = false;
        }
      }
      if ok {
        map.add_room_to_map(&new_room);
        if !map.rooms.is_empty() {
          let (new_x, new_y) = new_room.center();
          let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
          if rng.range(0, 1) == 1 {
            map.add_horizontal_tunnel(prev_x as i32, new_x as i32, prev_y as i32);
            map.add_vertical_tunnel(new_x as i32, new_y as i32, prev_y as i32);
          } else {
            map.add_vertical_tunnel(prev_x as i32, prev_y as i32, new_y as i32);
            map.add_horizontal_tunnel(prev_x as i32, new_x as i32, new_y as i32);
          }
        }
        map.rooms.push(new_room);
      }
    }
    map
  }
}

impl BaseMap for Map {
  fn is_opaque(&self, idx: usize) -> bool {
    self.tiles[idx as usize] == TileType::Wall
  }

  fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
    let mut exits: Vec<(usize, f32)> = vec![];
    let (x, y) = self.idx_xy(idx as i32);
    if self.is_exit_valid(x - 1, y) {
      exits.push((idx - 1, 1.0))
    }
    if self.is_exit_valid(x + 1, y) {
      exits.push((idx + 1, 1.0))
    }
    if self.is_exit_valid(x, y - 1) {
      exits.push((idx - self.width as usize, 1.0))
    }
    if self.is_exit_valid(x, y + 1) {
      exits.push((idx + self.width as usize, 1.0))
    }
    if self.is_exit_valid(x - 1, y - 1) {
      exits.push(((idx - self.width as usize) - 1, 1.45))
    }
    if self.is_exit_valid(x + 1, y - 1) {
      exits.push(((idx - self.width as usize) + 1, 1.45))
    }
    if self.is_exit_valid(x - 1, y + 1) {
      exits.push(((idx + self.width as usize) - 1, 1.45))
    }
    if self.is_exit_valid(x + 1, y + 1) {
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

impl Algorithm2D for Map {
  fn dimensions(&self) -> Point {
    Point::new(self.width, self.height)
  }
}
