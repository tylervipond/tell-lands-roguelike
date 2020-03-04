use crate::components::{
  combat_stats::CombatStats, dungeon_level::DungeonLevel, name::Name, player::Player,
  position::Position,
};
use crate::dungeon::dungeon::Dungeon;
use crate::game_log::GameLog;
use crate::main_menu_option::MainMenuOption;
use crate::sizes;
use rltk::{
  console::Console, Point, Rltk, BLACK, BLUE, CYAN, GREY, LIGHT_GRAY, MAGENTA, RED, RGB, WHITE,
  YELLOW,
};
use specs::{Entity, Join, World, WorldExt};

const GUI_LEFT: i32 = 0;
const GUI_TOP: i32 = 43;
const GUI_WIDTH: i32 = 79;
const GUI_HEIGHT: i32 = 6;
const GUI_HEALTH_LEFT: i32 = 12;
const GUI_HEALTH_BAR_LEFT: i32 = 28;
const GUI_HEALTH_BAR_WIDTH: i32 = GUI_WIDTH - GUI_HEALTH_BAR_LEFT;
const MESSAGES_TOP: i32 = GUI_TOP + 1;
const MESSAGES_LEFT: i32 = GUI_LEFT + 1;
const MESSAGES_WIDTH: i32 = GUI_WIDTH - 2;
const MESSAGES_HEIGHT: i32 = GUI_HEIGHT - 2;
const MESSAGE_COUNT: i32 = MESSAGES_HEIGHT;

fn print_yellow_on_black(ctx: &mut Rltk, x: i32, y: i32, s: &str) {
  ctx.print_color(x, y, RGB::named(YELLOW), RGB::named(BLACK), s);
}

fn print_white_on_black(ctx: &mut Rltk, x: i32, y: i32, s: &str) {
  ctx.print_color(x, y, RGB::named(WHITE), RGB::named(BLACK), s);
}

fn print_light_grey_on_black(ctx: &mut Rltk, x: i32, y: i32, s: &str) {
  ctx.print_color(x, y, RGB::named(LIGHT_GRAY), RGB::named(BLACK), s);
}

fn char_white_on_black(ctx: &mut Rltk, x: i32, y: i32, c: u8) {
  ctx.set(x, y, RGB::named(WHITE), RGB::named(BLACK), c);
}

fn char_yellow_on_black(ctx: &mut Rltk, x: i32, y: i32, c: u8) {
  ctx.set(x, y, RGB::named(YELLOW), RGB::named(BLACK), c);
}

fn draw_white_on_black_box(ctx: &mut Rltk, x: i32, y: i32, width: i32, height: i32) {
  ctx.draw_box(x, y, width, height, RGB::named(WHITE), RGB::named(BLACK));
}

pub fn draw_tooltip(ecs: &World, ctx: &mut Rltk) {
  let mut dungeon = ecs.write_resource::<Dungeon>();
  let levels = ecs.read_storage::<DungeonLevel>();
  let player_ent = ecs.read_resource::<Entity>();
  let player_level = levels.get(*player_ent).unwrap();
  let map = dungeon.get_map(player_level.level).unwrap();
  let names = ecs.read_storage::<Name>();
  let positions = ecs.read_storage::<Position>();

  let mouse_pos = ctx.mouse_pos();
  if mouse_pos.0 >= map.width as i32 || mouse_pos.1 >= map.height as i32 {
    return;
  }
  let tooltip: Vec<String> =
    (&names, &positions)
      .join()
      .fold(vec![], |mut acc, (name, position)| {
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 {
          acc.push(name.name.to_owned());
        }
        acc
      });
  if !tooltip.is_empty() {
    let width: i32 = tooltip.iter().max_by_key(|x| x.len()).unwrap().len() as i32 + 3;
    let foreground = RGB::named(WHITE);
    let background = RGB::named(GREY);
    if mouse_pos.0 > 40 {
      let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);
      let left_x = mouse_pos.0 - width;
      for (index, tip) in tooltip.iter().enumerate() {
        let x = left_x;
        let y = mouse_pos.1 + index as i32;
        ctx.print_color(x, y, foreground, background, &tip.to_string());
        let padding = width - tip.len() as i32 - 1;
        for i in 0..padding {
          let x = arrow_pos.x - i;
          ctx.print_color(x, y, foreground, background, &" ".to_string())
        }
      }
      let Point { x, y } = arrow_pos;
      ctx.print_color(x, y, foreground, background, &"->".to_string())
    } else {
      let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
      let left_x = mouse_pos.0 + 3;
      for (index, tip) in tooltip.iter().enumerate() {
        let x = left_x;
        let y = mouse_pos.1 + index as i32;
        ctx.print_color(x, y, foreground, background, &tip.to_string());
        let padding = width - tip.len() as i32 - 1;
        for i in 0..padding {
          let x = left_x + tip.len() as i32 + i;
          ctx.print_color(x, y, foreground, background, &" ".to_string());
        }
      }
      let Point { x, y } = arrow_pos;
      ctx.print_color(x, y, foreground, background, &"<-".to_string());
    }
  }
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
  draw_white_on_black_box(ctx, GUI_LEFT, GUI_TOP, GUI_WIDTH, GUI_HEIGHT);
  let player = ecs.read_storage::<Player>();
  let combat_stats = ecs.read_storage::<CombatStats>();
  let log = ecs.fetch::<GameLog>();
  let mut dungeon = ecs.fetch_mut::<Dungeon>();
  let player_ent = ecs.fetch::<Entity>();
  let levels = ecs.read_storage::<DungeonLevel>();
  let player_level = levels.get(*player_ent).unwrap();
  let map = dungeon.get_map(player_level.level).unwrap();
  let depth = format!("Depth: {}", map.depth);
  print_yellow_on_black(ctx, 2, 43, &depth);
  for (_player, stats) in (&player, &combat_stats).join() {
    let health = format!("HP: {} / {}", stats.hp, stats.max_hp);
    print_yellow_on_black(ctx, GUI_HEALTH_LEFT, GUI_TOP, &health);
    ctx.draw_bar_horizontal(
      GUI_HEALTH_BAR_LEFT,
      GUI_TOP,
      GUI_HEALTH_BAR_WIDTH,
      stats.hp,
      stats.max_hp,
      RGB::named(RED),
      RGB::named(BLACK),
    );
  }
  for (i, message) in log.entries.iter().enumerate().take(MESSAGE_COUNT as usize) {
    ctx.print(MESSAGES_LEFT, MESSAGES_TOP + i as i32, &message.to_string());
  }
  let mouse_pos = ctx.mouse_pos();
  ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(MAGENTA));
}

pub fn show_inventory(ctx: &mut Rltk, inventory: Vec<String>, title: &str) {
  let inventory_count = inventory.iter().count() as i32;
  let y = 25 - (inventory_count / 2);
  draw_white_on_black_box(ctx, 15, y - 2, 31, inventory_count + 3);
  print_yellow_on_black(ctx, 18, y - 2, title);
  print_yellow_on_black(ctx, 18, y + inventory_count + 1, "Escape to cancel");
  for (i, name) in inventory.iter().enumerate() {
    let new_y = y + i as i32;
    char_white_on_black(ctx, 17, new_y, rltk::to_cp437('('));
    char_yellow_on_black(ctx, 18, new_y, 97 + i as u8);
    char_white_on_black(ctx, 19, new_y, rltk::to_cp437(')'));
    ctx.print(21, new_y, name);
  }
}

pub fn show_valid_targeting_area(ctx: &mut Rltk, tiles: &Vec<Point>) {
  print_yellow_on_black(ctx, 5, 0, "Select Target");
  tiles
    .iter()
    .for_each(|tile| ctx.set_bg(tile.x, tile.y, RGB::named(BLUE)))
}

pub fn show_current_target(ctx: &mut Rltk, tile: &Point) {
  ctx.set_bg(tile.x, tile.y, RGB::named(CYAN))
}

fn get_offset_from_center(context_size: usize, content_size: usize) -> usize {
  return context_size / 2 - content_size / 2;
}

fn print_main_menu_option(
  ctx: &mut Rltk,
  option: &MainMenuOption,
  highighted: bool,
  vertical_offset: usize,
) {
  let copy_horizontal_offset =
    get_offset_from_center(sizes::CHAR_COUNT_HORIZONTAL, option.text.len());
  if option.disabled {
    print_light_grey_on_black(
      ctx,
      copy_horizontal_offset as i32,
      vertical_offset as i32,
      option.text,
    )
  } else if highighted {
    print_yellow_on_black(
      ctx,
      copy_horizontal_offset as i32,
      vertical_offset as i32,
      option.text,
    )
  } else {
    print_white_on_black(
      ctx,
      copy_horizontal_offset as i32,
      vertical_offset as i32,
      option.text,
    );
  }
}

pub fn show_main_menu(ctx: &mut Rltk, menu: &Vec<MainMenuOption>, highighted: usize) {
  menu
    .iter()
    .enumerate()
    .for_each(|(i, o)| print_main_menu_option(ctx, o, highighted == i, (i + 1) * 2));
}
