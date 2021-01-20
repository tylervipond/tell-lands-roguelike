use crate::ui_components::{UIDynamicBoxList, UITextLine};
use rltk::Rltk;

pub enum UIToolTipPosition {
    Left,
    Right,
}

pub struct UIToolTip<'a, 'b> {
    x: i32,
    y: i32,
    position: UIToolTipPosition,
    lines: &'b Box<[&'a str]>,
}

impl<'a, 'b> UIToolTip<'a, 'b> {
    pub fn new(x: i32, y: i32, position: UIToolTipPosition, lines: &'b Box<[&'a str]>) -> Self {
        Self {
            x,
            y,
            position,
            lines,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        let mut box_list = UIDynamicBoxList::new(0, self.y - 1, self.lines);
        match self.position {
            UIToolTipPosition::Left => {
                let box_x = self.x - box_list.width as i32 - 3;
                box_list.x = box_x;
                UITextLine::new(self.x - 2, self.y, "->", None).draw(ctx);
            }
            UIToolTipPosition::Right => {
                let box_x = self.x + 3;
                box_list.x = box_x;
                UITextLine::new(self.x + 1, self.y, "<-", None).draw(ctx);
            }
        };
        box_list.draw(ctx);
    }
}
