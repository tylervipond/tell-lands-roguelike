use rltk::{Console, Rltk, MAGENTA, RGB};

pub struct UIMousePos {
    x: i32,
    y: i32,
}

impl UIMousePos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.set_bg(self.x, self.y, RGB::named(MAGENTA));
    }
}
