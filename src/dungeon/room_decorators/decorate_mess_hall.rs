use super::{
    common::{add_table_to_room, replace_middle_3x3},
    utils::find_and_replace,
    RoomPart,
    RoomPart::{Chair, Floor, Table},
};
use rltk::RandomNumberGenerator;
use stamp_rs::{
    QueryStampPart::{Any, Is},
    Stamp, StampPart,
};

fn add_chair_to_mess_hall(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Table, Floor, Chair])),
            Is(Box::new([Table])),
            Is(Box::new([Table, Floor, Chair])),
        ],
        vec![
            Is(Box::new([Table, Floor, Chair])),
            Is(Box::new([Floor])),
            Is(Box::new([Table, Floor, Chair])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ]);
    let mut replace_stamp = replace_middle_3x3(Chair);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

// This should likely be fixed to not take quite as long... it doesn't take that long, but still
pub fn decorate_mess_hall(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    for _ in 0..rng.range(20, 100) {
        add_table_to_room(room_stamp, rng);
    }
    for _ in 0..rng.range(30, 110) {
        add_chair_to_mess_hall(room_stamp, rng);
    }
}
