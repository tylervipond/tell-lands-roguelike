use crate::types::TrapType;
use specs::Entity;

pub struct TrapSpawnerRequest {
    pub x: i32,
    pub y: i32,
    pub level: u8,
    pub set_by: Entity,
    pub trap_type: TrapType,
}

pub struct TrapSpawner {
    pub requests: Vec<TrapSpawnerRequest>,
}

impl TrapSpawner {
    pub fn new() -> Self {
        TrapSpawner {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, level: u8, set_by: Entity, trap_type: TrapType) {
        self.requests.push(TrapSpawnerRequest {
            x,
            y,
            level,
            set_by,
            trap_type,
        })
    }
}
