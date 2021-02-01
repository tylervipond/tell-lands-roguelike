use rltk::{to_cp437, BLACK, DARK_RED, RGB};

use crate::entity_set::EntitySet;

pub struct CorpseSpawnerRequest {
    pub idx: usize,
    pub fg: RGB,
    pub bg: RGB,
    pub glyph: u16,
    pub level: u8,
    pub name: String,
    pub items: EntitySet,
}

pub struct CorpseSpawner {
    pub requests: Vec<CorpseSpawnerRequest>,
}

impl CorpseSpawner {
    pub fn new() -> Self {
        CorpseSpawner {
            requests: Vec::new(),
        }
    }

    pub fn request(
        &mut self,
        idx: usize,
        fg: RGB,
        bg: RGB,
        glyph: u16,
        level: u8,
        name: String,
        items: EntitySet,
    ) {
        self.requests.push(CorpseSpawnerRequest {
            idx,
            fg,
            bg,
            glyph,
            level,
            name,
            items,
        });
    }

    pub fn request_goblin_corpse(
        &mut self,
        idx: usize,
        level: u8,
        cause_of_death: String,
        items: EntitySet,
    ) {
        self.request(
            idx,
            RGB::named(BLACK),
            RGB::named(DARK_RED),
            to_cp437('g'),
            level,
            format!("goblin corpse, {}", cause_of_death),
            items,
        );
    }
}
