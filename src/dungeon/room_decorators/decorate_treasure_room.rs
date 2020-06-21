use super::{
    common::replace_middle_3x3,
    utils::find_and_replace,
    RoomPart,
    RoomPart::{Chest, Floor, Wall},
};
use rltk::RandomNumberGenerator;
use stamp_rs::{
    QueryStampPart::{Is, Any},
    Stamp, StampPart,
};

fn add_chest_to_room(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Floor, Wall])),
            Is(Box::new([Floor, Chest])),
            Is(Box::new([Floor])),
        ],
        vec![
            Is(Box::new([Floor, Wall])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![
            Any,
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
    ]);
    let mut replace_stamp = replace_middle_3x3(Chest);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn decorate_treasure_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    for _ in 0..rng.range(5, 16) {
        add_chest_to_room(room_stamp, rng);
    }
}
