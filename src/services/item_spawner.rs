use crate::types::ItemType;

pub struct ItemSpawnerRequest {
    pub x: i32,
    pub y: i32,
    pub level: u8,
    pub item_type: ItemType,
}

pub struct ItemSpawner {
    pub requests: Vec<ItemSpawnerRequest>,
}

impl ItemSpawner {
    pub fn new() -> Self {
        ItemSpawner {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, level: u8, item_type: ItemType) {
        self.requests.push(ItemSpawnerRequest {
            x,
            y,
            level,
            item_type,
        })
    }
}
