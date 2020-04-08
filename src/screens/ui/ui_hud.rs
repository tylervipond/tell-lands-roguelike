use crate::ui_components::{ui_box::UIBox, ui_text_line::UITextLine};
use rltk::{Console, Rltk, BLACK, RED, RGB, WHITE, YELLOW};

const GUI_LEFT: i32 = 0;
const GUI_TOP: i32 = 43;
const GUI_WIDTH: u8 = 79;
const GUI_HEIGHT: u8 = 6;
const GUI_HEALTH_LEFT: i32 = 12;
const GUI_HEALTH_BAR_LEFT: i32 = 28;
const GUI_HEALTH_BAR_WIDTH: i32 = GUI_WIDTH as i32 - GUI_HEALTH_BAR_LEFT;
const MESSAGES_TOP: i32 = GUI_TOP + 1;
const MESSAGES_LEFT: i32 = GUI_LEFT + 1;
const MESSAGE_COUNT: u8 = GUI_HEIGHT - 2;

pub struct UIHud<'a> {
    depth: i32,
    hp: i32,
    max_hp: i32,
    messages: &'a Vec<String>,
}

impl<'a> UIHud<'a> {
    pub fn new(depth: i32, hp: i32, max_hp: i32, messages: &'a Vec<String>) -> Self {
        Self {
            depth,
            hp,
            max_hp,
            messages,
        }
    }

    pub fn draw(&self, ctx: &mut Rltk) {
        UIBox::new(GUI_LEFT, GUI_TOP, GUI_WIDTH, GUI_HEIGHT, WHITE, BLACK).draw(ctx);
        UITextLine::new(2, GUI_TOP, YELLOW, BLACK, &format!("Depth: {}", self.depth)).draw(ctx);
        let health = format!("HP: {} / {}", self.hp, self.max_hp);
        UITextLine::new(GUI_HEALTH_LEFT, GUI_TOP, YELLOW, BLACK, &health).draw(ctx);
        ctx.draw_bar_horizontal(
            GUI_HEALTH_BAR_LEFT,
            GUI_TOP,
            GUI_HEALTH_BAR_WIDTH,
            self.hp,
            self.max_hp,
            RGB::named(RED),
            RGB::named(BLACK),
        );
        for (i, message) in self
            .messages
            .iter()
            .enumerate()
            .take(MESSAGE_COUNT as usize)
        {
            ctx.print(MESSAGES_LEFT, MESSAGES_TOP + i as i32, &message.to_string());
        }
    }
}
