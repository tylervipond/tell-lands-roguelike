use super::{
    common::replace_middle_3x3,
    utils::find_and_replace,
    RoomPart,
    RoomPart::{Bed, Chest, Floor, Wall, WeaponRack},
};
use rltk::RandomNumberGenerator;
use stamp_rs::{
    QueryStampPart::{Any, Is},
    Stamp, StampPart,
    StampPart::{Transparent, Use},
};

fn add_bed_to_room(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    let mut query_stamp = Stamp::new(vec![
        vec![Any, Is(Box::new([Wall])), Any],
        vec![
            Is(Box::new([Floor, Wall])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor, Wall])),
        ],
        vec![
            Is(Box::new([Floor, Wall])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor, Wall])),
        ],
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
    ]);
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(Bed), Transparent],
        vec![Transparent, Use(Chest), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

fn add_weapons_rack_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor, WeaponRack])),
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
    let mut replace_stamp = replace_middle_3x3(WeaponRack);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn decorate_barracks(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    for _ in 0..rng.range(10, 20) {
        add_bed_to_room(room_stamp, rng);
    }
    for _ in 0..rng.range(5, 10) {
        add_weapons_rack_to_room(room_stamp, rng);
    }
}
