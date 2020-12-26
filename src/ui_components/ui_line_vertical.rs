use rltk::{Rltk, BLACK, WHITE};

pub struct UILineVertical {
    pub x: i32,
    pub y: i32,
    pub height: u32,
}

impl UILineVertical {
    pub fn new(x: i32, y: i32, height: u32) -> Self {
        Self { x, y, height }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        for y in self.y..=self.y + self.height as i32 {
            ctx.print_color(self.x, y, WHITE, BLACK, '│');
        }
    }
}
