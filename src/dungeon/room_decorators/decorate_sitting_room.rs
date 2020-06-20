use super::{
    common::{add_part_against_wall, replace_middle_3x3},
    utils::find_and_replace,
    RoomPart,
    RoomPart::{Chair, Chest, Floor, Shelf, Table, Wall},
};
use rltk::RandomNumberGenerator;
use stamp_rs::{
    QueryStampPart::{Any, Is},
    Stamp, StampPart,
};

fn add_table_to_sitting_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Floor, Wall])),
            Is(Box::new([Floor, Wall])),
            Is(Box::new([Floor, Wall])),
        ],
        vec![
            Is(Box::new([Floor, Wall])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor, Wall])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ]);
    let mut replace_stamp = replace_middle_3x3(Table);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

fn add_chair_to_sitting_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Wall, Floor, Chair])),
            Is(Box::new([Table])),
            Is(Box::new([Wall, Floor, Chair])),
        ],
        vec![
            Is(Box::new([Wall, Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Wall, Floor])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ]);
    let mut replace_stamp = replace_middle_3x3(Chair);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn decorate_sitting_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    add_table_to_sitting_room(room_stamp, rng);
    for _ in 0..rng.range(1, 3) {
        add_chair_to_sitting_room(room_stamp, rng);
    }
    for _ in 0..rng.range(1, 3) {
        add_part_against_wall(room_stamp, rng, Shelf);
    }
    match rng.range(0, 2) {
        0 => add_part_against_wall(room_stamp, rng, Chest),
        _ => {}
    };
}
