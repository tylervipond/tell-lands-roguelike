use rltk::RGB;

pub struct DebrisSpawnerRequest {
    pub x: i32,
    pub y: i32,
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
        x: i32,
        y: i32,
        fg: RGB,
        bg: RGB,
        glyph: u16,
        level: u8,
        name: String,
    ) {
        self.requests.push(DebrisSpawnerRequest {
            x,
            y,
            fg,
            bg,
            glyph,
            level,
            name,
        })
    }
}
