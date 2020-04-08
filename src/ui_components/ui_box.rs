use rltk::{Rltk, Console, RGB};
pub struct UIBox {
    x: i32,
    y: i32,
    width: u8,
    height: u8,
    fg: (u8, u8, u8),
    bg: (u8, u8, u8),
}

impl UIBox {
    pub fn new(x: i32, y: i32, width: u8, height: u8, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Self {
        Self {
            x,
            y,
            width,
            height,
            fg,
            bg,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        ctx.draw_box(self.x, self.y, self.width as i32, self.height as i32, RGB::named(self.fg), RGB::named(self.bg));
    }
}
