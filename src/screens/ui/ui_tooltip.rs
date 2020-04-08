use crate::ui_components::{ui_dynamic_box_list::UIDynamicBoxList, ui_text_line::UITextLine};
use rltk::{Rltk, BLACK, WHITE};

pub enum UIToolTipPosition {
    Left,
    Right,
}

pub struct UIToolTip<'a> {
    x: i32,
    y: i32,
    position: UIToolTipPosition,
    lines: &'a Vec<String>,
}

impl<'a> UIToolTip<'a> {
    pub fn new(x: i32, y: i32, position: UIToolTipPosition, lines: &'a Vec<String>) -> Self {
        Self {
            x,
            y,
            position,
            lines,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        let mut box_list = UIDynamicBoxList::new(0, self.y-1, self.lines);
        match self.position {
            UIToolTipPosition::Left => {
                let box_x = self.x - box_list.width as i32 - 3;
                box_list.x = box_x;
                UITextLine::new(self.x - 2, self.y, WHITE, BLACK, "->").draw(ctx);
            }
            UIToolTipPosition::Right => {
                let box_x = self.x + 3;
                box_list.x = box_x;
                UITextLine::new(self.x + 1, self.y, WHITE, BLACK, "<-").draw(ctx);
            }
        };
        box_list.draw(ctx);
    }
}
