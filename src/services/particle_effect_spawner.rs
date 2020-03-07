use rltk::RGB;

pub struct ParticleEffectSpawnerRequest {
    pub x: i32,
    pub y: i32,
    pub fg: RGB,
    pub bg: RGB,
    pub glyph: u8,
    pub lifetime: f32,
    pub level: i32,
}

pub struct ParticleEffectSpawner {
    pub requests: Vec<ParticleEffectSpawnerRequest>,
}

impl ParticleEffectSpawner {
    pub fn new() -> Self {
        ParticleEffectSpawner {
            requests: Vec::new(),
        }
    }

    pub fn request(
        &mut self,
        x: i32,
        y: i32,
        fg: RGB,
        bg: RGB,
        glyph: u8,
        lifetime: f32,
        level: i32,
    ) {
        self.requests.push(ParticleEffectSpawnerRequest {
            x,
            y,
            fg,
            bg,
            glyph,
            lifetime,
            level,
        })
    }
}
