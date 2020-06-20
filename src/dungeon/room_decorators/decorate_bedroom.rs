use super::{
    common::{add_desk_to_room, add_part_against_wall, replace_middle_3x3},
    utils::find_and_replace,
    RoomPart,
    RoomPart::{Armoire, Bed, BedsideTable, Chest, Dresser, Floor, Shelf, Wall},
};
use rltk::RandomNumberGenerator;
use stamp_rs::{
    QueryStampPart::{Any, Is},
    Stamp, StampPart,
};

fn add_bed_to_room(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Wall])),
            Is(Box::new([Wall])),
            Is(Box::new([Wall])),
        ],
        vec![
            Is(Box::new([Floor, Wall])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![Any, Is(Box::new([Floor])), Is(Box::new([Floor]))],
    ]);
    let mut replace_stamp = replace_middle_3x3(Bed);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn add_bedside_table_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![Is(Box::new([Wall])), Is(Box::new([Wall])), Any],
        vec![
            Is(Box::new([Bed])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
    ]);
    let mut replace_stamp = replace_middle_3x3(BedsideTable);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn decorate_bedroom(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    add_bed_to_room(room_stamp, rng);
    add_bedside_table_to_room(room_stamp, rng);
    match rng.range(0, 2) {
        0 => add_desk_to_room(room_stamp, rng),
        _ => {}
    };
    match rng.range(0, 2) {
        0 => add_part_against_wall(room_stamp, rng, Armoire),
        _ => {}
    };
    match rng.range(0, 2) {
        0 => add_part_against_wall(room_stamp, rng, Dresser),
        _ => {}
    };
    match rng.range(0, 2) {
        0 => add_part_against_wall(room_stamp, rng, Chest),
        _ => {}
    };
    match rng.range(0, 2) {
        0 => add_part_against_wall(room_stamp, rng, Shelf),
        _ => {}
    };
}
