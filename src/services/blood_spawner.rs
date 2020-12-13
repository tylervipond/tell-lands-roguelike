use rltk::RGB;

pub struct BloodSpawnerRequest {
    pub idx: usize,
    pub fg: RGB,
    pub bg: RGB,
    pub glyph: u16,
    pub level: u8,
}

pub struct BloodSpawner {
    pub requests: Vec<BloodSpawnerRequest>,
}

impl BloodSpawner {
    pub fn new() -> Self {
        BloodSpawner {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, idx: usize, fg: RGB, bg: RGB, glyph: u16, level: u8) {
        self.requests.push(BloodSpawnerRequest {
            idx,
            fg,
            bg,
            glyph,
            level,
        })
    }
}
