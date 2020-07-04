use super::{rect::Rect, tile_type::TileType};
use rltk::RandomNumberGenerator;

fn get_single_line(start: i32, end: i32, spacing: i32) -> Vec<i32> {
    (start..end)
        .enumerate()
        .filter(|(idx, _num)| idx % (spacing as usize + 1) == 0)
        .map(|(_, num)| num)
        .collect()
}

fn get_columns_for_vertical(
    rect: &Rect,
    x: i32,
    space_between_columns: i32,
    space_between_column_and_wall: i32,
) -> Vec<((i32, i32), TileType)> {
    let first_y = rect.y1 + space_between_column_and_wall + 1;
    let last_y = rect.y2 - space_between_column_and_wall;
    get_single_line(first_y, last_y, space_between_columns)
        .iter()
        .map(|y| ((x, *y), TileType::Column))
        .collect()
}

fn get_columns_for_horizontal(
    rect: &Rect,
    y: i32,
    space_between_columns: i32,
    space_between_column_and_wall: i32,
) -> Vec<((i32, i32), TileType)> {
    let first_x = rect.x1 + space_between_column_and_wall + 1;
    let last_x = rect.x2 - space_between_column_and_wall;
    get_single_line(first_x, last_x, space_between_columns)
        .iter()
        .map(|x| ((*x, y), TileType::Column))
        .collect()
}

fn get_spacing(rng: &mut RandomNumberGenerator) -> (i32, i32, i32) {
    (rng.range(2, 3), rng.range(0, 2), rng.range(2, 3))
}

pub fn get_columns_top(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
    num_rows: usize,
) -> Vec<((i32, i32), TileType)> {
    let (space_between_columns, space_between_column_and_wall, space_between_rows) =
        get_spacing(rng);
    let y = rect.y1 + space_between_column_and_wall + 1;
    (0..num_rows)
        .map(|row_num| {
            get_columns_for_horizontal(
                rect,
                y + row_num as i32 * space_between_rows,
                space_between_columns,
                space_between_column_and_wall,
            )
        })
        .flatten()
        .collect()
}

pub fn get_columns_bottom(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
    num_rows: usize,
) -> Vec<((i32, i32), TileType)> {
    let (space_between_columns, space_between_column_and_wall, space_between_rows) =
        get_spacing(rng);
    let y = rect.y2 - space_between_column_and_wall - 1;
    (0..num_rows)
        .map(|row_num| {
            get_columns_for_horizontal(
                rect,
                y - row_num as i32 * space_between_rows,
                space_between_columns,
                space_between_column_and_wall,
            )
        })
        .flatten()
        .collect()
}

pub fn get_columns_left(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
    num_rows: usize,
) -> Vec<((i32, i32), TileType)> {
    let (space_between_columns, space_between_column_and_wall, space_between_rows) =
        get_spacing(rng);
    let x = rect.x1 + space_between_column_and_wall + 1;
    (0..num_rows)
        .map(|row_num| {
            get_columns_for_vertical(
                rect,
                x + row_num as i32 * space_between_rows,
                space_between_columns,
                space_between_column_and_wall,
            )
        })
        .flatten()
        .collect()
}

pub fn get_columns_right(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
    num_rows: usize,
) -> Vec<((i32, i32), TileType)> {
    let (space_between_columns, space_between_column_and_wall, space_between_rows) =
        get_spacing(rng);
    let x = rect.x2 - space_between_column_and_wall - 1;
    (0..num_rows)
        .map(|row_num| {
            get_columns_for_vertical(
                rect,
                x - row_num as i32 * space_between_rows,
                space_between_columns,
                space_between_column_and_wall,
            )
        })
        .flatten()
        .collect()
}

pub fn get_columns_vertical(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
    num_rows: usize,
) -> Vec<((i32, i32), TileType)> {
    let (space_between_columns, space_between_column_and_wall, space_between_rows) =
        get_spacing(rng);
    let x_right = rect.x2 - space_between_column_and_wall - 1;
    let x_left = rect.x1 + space_between_column_and_wall + 1;
    (0..num_rows)
        .map(|row_num| {
            let modifier = row_num as i32 * space_between_rows;
            let mut columns_right = get_columns_for_vertical(
                rect,
                x_right - modifier,
                space_between_columns,
                space_between_column_and_wall,
            );
            let mut columns_left = get_columns_for_vertical(
                rect,
                x_left + modifier,
                space_between_columns,
                space_between_column_and_wall,
            );
            columns_right.append(&mut columns_left);
            columns_right
        })
        .flatten()
        .collect()
}

pub fn get_columns_horizontal(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
    num_rows: usize,
) -> Vec<((i32, i32), TileType)> {
    let (space_between_columns, space_between_column_and_wall, space_between_rows) =
        get_spacing(rng);
    let y_bottom = rect.y2 - space_between_column_and_wall - 1;
    let y_top = rect.y1 + space_between_column_and_wall + 1;
    (0..num_rows)
        .map(|row_num| {
            let modifier = row_num as i32 * space_between_rows;
            let mut columns_bottom = get_columns_for_horizontal(
                rect,
                y_bottom - modifier,
                space_between_columns,
                space_between_column_and_wall,
            );
            let mut columns_top = get_columns_for_horizontal(
                rect,
                y_top + modifier,
                space_between_columns,
                space_between_column_and_wall,
            );
            columns_bottom.append(&mut columns_top);
            columns_bottom
        })
        .flatten()
        .collect()
}
pub fn get_columns_all(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
    num_rows: usize,
) -> Vec<((i32, i32), TileType)> {
    let (space_between_columns, space_between_column_and_wall, space_between_rows) =
        get_spacing(rng);
    let y_bottom = rect.y2 - space_between_column_and_wall - 1;
    let y_top = rect.y1 + space_between_column_and_wall + 1;
    let x_right = rect.x2 - space_between_column_and_wall - 1;
    let x_left = rect.x1 + space_between_column_and_wall + 1;
    (0..num_rows)
        .map(|row_num| {
            let modifier = row_num as i32 * space_between_rows;
            let mut columns_bottom = get_columns_for_horizontal(
                rect,
                y_bottom - modifier,
                space_between_columns,
                space_between_column_and_wall,
            );
            let mut columns_top = get_columns_for_horizontal(
                rect,
                y_top + modifier,
                space_between_columns,
                space_between_column_and_wall,
            );
            let mut columns_right = get_columns_for_vertical(
                rect,
                x_right - modifier,
                space_between_columns,
                space_between_column_and_wall,
            );
            let mut columns_left = get_columns_for_vertical(
                rect,
                x_left + modifier,
                space_between_columns,
                space_between_column_and_wall,
            );
            columns_bottom.append(&mut columns_top);
            columns_bottom.append(&mut columns_left);
            columns_bottom.append(&mut columns_right);
            columns_bottom
        })
        .flatten()
        .collect()
}

pub fn get_columns_single_middle(rect: &Rect) -> Vec<((i32, i32), TileType)> {
    let x = rect.x1 + (rect.x2 - rect.x1) / 2;
    let y = rect.y1 + (rect.y2 - rect.y1) / 2;
    vec![((x, y), TileType::Column)]
}

pub fn get_columns_double_middle(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
) -> Vec<((i32, i32), TileType)> {
    let space_between_columns = rng.range(0, 3);
    let x = rect.x1 + (rect.x2 - rect.x1) / 2 - space_between_columns / 2;
    let y = rect.y1 + (rect.y2 - rect.y1) / 2 - space_between_columns / 2;
    vec![
        ((x, y), TileType::Column),
        ((x + space_between_columns, y), TileType::Column),
        ((x, y + space_between_columns), TileType::Column),
        (
            (x + space_between_columns, y + space_between_columns),
            TileType::Column,
        ),
    ]
}
