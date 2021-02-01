use rltk::{RGB, to_cp437, DARK_RED, BLACK};

pub struct CorpseSpawnerRequest {
    pub idx: usize,
    pub fg: RGB,
    pub bg: RGB,
    pub glyph: u16,
    pub level: u8,
    pub name: String,
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

    pub fn request(&mut self, idx: usize, fg: RGB, bg: RGB, glyph: u16, level: u8, name: String) {
        self.requests.push(CorpseSpawnerRequest {
            idx,
            fg,
            bg,
            glyph,
            level,
            name,
        });
    }

    pub fn request_goblin_corpse(&mut self, idx: usize, level: u8, cause_of_death: String) {
        self.request(
            idx,
            RGB::named(BLACK),
            RGB::named(DARK_RED),
            to_cp437('g'),
            level,
            format!("goblin corpse, {}", cause_of_death)
        );
    }
}
