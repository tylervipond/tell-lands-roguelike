use super::{
    common::{add_chair_next_to_table, add_part_against_wall},
    utils::find_and_replace,
    RoomPart,
    RoomPart::{Floor, Table, Cupboard, Shelf},
};
use rltk::RandomNumberGenerator;
use stamp_rs::{
    QueryStampPart::Is,
    Stamp, StampPart,
    StampPart::{Transparent, Use},
};

fn add_table_to_room(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
    ]);
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(Table), Transparent],
        vec![Transparent, Use(Table), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn decorate_dining_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    add_table_to_room(room_stamp, rng);
    for _ in 0..rng.range(3, 6) {
        add_chair_next_to_table(room_stamp, rng);
    }
    for _ in 0..rng.range(1, 2) {
        add_part_against_wall(room_stamp, rng, Cupboard);
    }
    for _ in 0..rng.range(1, 2) {
        add_part_against_wall(room_stamp, rng, Shelf);
    }
}
