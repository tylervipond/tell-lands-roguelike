use rltk::RGB;

pub struct DebrisSpawnerRequest {
    pub idx: usize,
    pub fg: RGB,
    pub bg: RGB,
    pub glyph: u16,
    pub level: u8,
    pub name: String,
}

pub struct DebrisSpawner {
    pub requests: Vec<DebrisSpawnerRequest>,
}

impl DebrisSpawner {
    pub fn new() -> Self {
        DebrisSpawner {
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
    ) {
        self.requests.push(DebrisSpawnerRequest {
            idx,
            fg,
            bg,
            glyph,
            level,
            name,
        })
    }
}
