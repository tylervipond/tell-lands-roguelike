use rltk::RGB;

pub struct BloodSpawnerRequest {
    pub x: i32,
    pub y: i32,
    pub fg: RGB,
    pub bg: RGB,
    pub glyph: u8,
    pub level: i32,
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

    pub fn request(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8, level: i32) {
        self.requests.push(BloodSpawnerRequest {
            x,
            y,
            fg,
            bg,
            glyph,
            level,
        })
    }
}
