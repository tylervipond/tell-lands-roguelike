use super::{
    room::Room,
    room_stamp_parts::{
        RoomPart,
        RoomPart::{
            Armoire, Bed, BedsideTable, Chair, Chest, Desk, Door, Dresser, Floor, Shelf, Wall,
        },
    },
    room_type::RoomType,
};
use crate::utils::get_random_element;
use rltk::RandomNumberGenerator;
use stamp_rs::{
    QueryStampPart,
    QueryStampPart::{Any, Is, Not},
    Stamp, StampPart,
    StampPart::{Transparent, Use},
};

#[derive(Clone)]
pub enum RoomStampRotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
    VFlipZero,
    VFlipNinety,
    VFlipOneEighty,
    VFlipTwoSeventy,
    HFlipZero,
    HFlipNinety,
    HFlipOneEighty,
    HFlipTwoSeventy,
}

struct RoomStampPosition {
    coord: (usize, usize),
    rotation: RoomStampRotation,
}

fn get_possible_positions_for_query_stamp(
    target_stamp: &Stamp<StampPart<RoomPart>>,
    query_stamp: &mut Stamp<QueryStampPart<RoomPart>>,
) -> Vec<RoomStampPosition> {
    let mut ninety_query_stamp = query_stamp.clone();
    ninety_query_stamp.rotate_90();
    let mut two_seventy_query_stamp = query_stamp.clone();
    two_seventy_query_stamp.rotate_n90();
    let mut one_eighty_query_stamp = query_stamp.clone();
    one_eighty_query_stamp.rotate_180();
    let mut h_flipped_query_stamp = query_stamp.clone();
    h_flipped_query_stamp.flip_horizontal();
    let mut h_flipped_ninety_query_stamp = h_flipped_query_stamp.clone();
    h_flipped_ninety_query_stamp.rotate_90();
    let mut h_flipped_two_seventy_query_stamp = h_flipped_query_stamp.clone();
    h_flipped_two_seventy_query_stamp.rotate_n90();
    let mut h_flipped_one_eighty_query_stamp = h_flipped_query_stamp.clone();
    h_flipped_one_eighty_query_stamp.rotate_180();
    let mut v_flipped_query_stamp = query_stamp.clone();
    v_flipped_query_stamp.flip_vertical();
    let mut v_flipped_ninety_query_stamp = v_flipped_query_stamp.clone();
    v_flipped_ninety_query_stamp.rotate_90();
    let mut v_flipped_two_seventy_query_stamp = v_flipped_query_stamp.clone();
    v_flipped_two_seventy_query_stamp.rotate_n90();
    let mut v_flipped_one_eighty_query_stamp = v_flipped_query_stamp.clone();
    v_flipped_one_eighty_query_stamp.rotate_180();

    [
        (query_stamp, RoomStampRotation::Zero),
        (&mut ninety_query_stamp, RoomStampRotation::Ninety),
        (&mut two_seventy_query_stamp, RoomStampRotation::TwoSeventy),
        (&mut one_eighty_query_stamp, RoomStampRotation::OneEighty),
        (&mut h_flipped_query_stamp, RoomStampRotation::HFlipZero),
        (
            &mut h_flipped_ninety_query_stamp,
            RoomStampRotation::HFlipNinety,
        ),
        (
            &mut h_flipped_two_seventy_query_stamp,
            RoomStampRotation::HFlipTwoSeventy,
        ),
        (
            &mut h_flipped_one_eighty_query_stamp,
            RoomStampRotation::HFlipOneEighty,
        ),
        (&mut v_flipped_query_stamp, RoomStampRotation::VFlipZero),
        (
            &mut v_flipped_ninety_query_stamp,
            RoomStampRotation::VFlipNinety,
        ),
        (
            &mut v_flipped_two_seventy_query_stamp,
            RoomStampRotation::VFlipTwoSeventy,
        ),
        (
            &mut v_flipped_one_eighty_query_stamp,
            RoomStampRotation::VFlipOneEighty,
        ),
    ]
    .iter()
    .map(move |(query_stamp, rotation)| {
        let new_target_stamp = target_stamp.clone();
        let positions = new_target_stamp.find(&query_stamp);

        let positions: Vec<RoomStampPosition> = positions
            .iter()
            .map(move |coord| RoomStampPosition {
                coord: *coord,
                rotation: rotation.clone(),
            })
            .collect();
        positions
    })
    .flatten()
    .collect()
}

fn apply_stamp_to_room_with_position(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    replace_stamp: &mut Stamp<StampPart<RoomPart>>,
    position: &RoomStampPosition,
) {
    let mut replace_stamp = replace_stamp.clone();
    match position.rotation {
        RoomStampRotation::Ninety => {
            replace_stamp.rotate_90();
        }
        RoomStampRotation::OneEighty => {
            replace_stamp.rotate_180();
        }
        RoomStampRotation::TwoSeventy => {
            replace_stamp.rotate_n90();
        }
        RoomStampRotation::VFlipZero => {
            replace_stamp.flip_vertical();
        }
        RoomStampRotation::VFlipNinety => {
            replace_stamp.flip_vertical();
            replace_stamp.rotate_90();
        }
        RoomStampRotation::VFlipOneEighty => {
            replace_stamp.flip_vertical();
            replace_stamp.rotate_180();
        }
        RoomStampRotation::VFlipTwoSeventy => {
            replace_stamp.flip_vertical();
            replace_stamp.rotate_n90();
        }
        RoomStampRotation::HFlipZero => {
            replace_stamp.flip_horizontal();
        }
        RoomStampRotation::HFlipNinety => {
            replace_stamp.flip_horizontal();
            replace_stamp.rotate_90();
        }
        RoomStampRotation::HFlipOneEighty => {
            replace_stamp.flip_horizontal();
            replace_stamp.rotate_180();
        }
        RoomStampRotation::HFlipTwoSeventy => {
            replace_stamp.flip_horizontal();
            replace_stamp.rotate_n90();
        }
        _ => {}
    };
    let (x, y) = position.coord;
    room_stamp.stamp(&replace_stamp, x, y);
}

fn find_and_replace(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
    query_stamp: &mut Stamp<QueryStampPart<RoomPart>>,
    replace_stamp: &mut Stamp<StampPart<RoomPart>>,
) {
    let possible_positions = get_possible_positions_for_query_stamp(room_stamp, query_stamp);
    if possible_positions.len() > 0 {
        let position = get_random_element(rng, &possible_positions);
        apply_stamp_to_room_with_position(room_stamp, replace_stamp, &position)
    }
}

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
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(Bed), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
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
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(BedsideTable), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

fn add_desk_to_room(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Wall])),
            Is(Box::new([Wall])),
            Is(Box::new([Wall])),
        ],
        vec![
            Is(Box::new([Wall, Floor])),
            Is(Box::new([Floor])),
            Not(Box::new([Door])),
        ],
        vec![
            Is(Box::new([Wall, Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ]);
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(Desk), Transparent],
        vec![Transparent, Use(Chair), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn add_armoire_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![Any, Is(Box::new([Wall])), Any],
        vec![
            Not(Box::new([Door])),
            Is(Box::new([Floor])),
            Not(Box::new([Door])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ]);
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(Armoire), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}
pub fn add_dresser_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![Any, Is(Box::new([Wall])), Any],
        vec![
            Not(Box::new([Door])),
            Is(Box::new([Floor])),
            Not(Box::new([Door])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ]);
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(Dresser), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}
pub fn add_chest_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![Any, Is(Box::new([Wall])), Any],
        vec![
            Not(Box::new([Door])),
            Is(Box::new([Floor])),
            Not(Box::new([Door])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ]);
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(Chest), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}
pub fn add_shelf_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![Any, Is(Box::new([Wall])), Any],
        vec![
            Not(Box::new([Door])),
            Is(Box::new([Floor])),
            Not(Box::new([Door])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ]);
    let mut replace_stamp = Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(Shelf), Transparent],
        vec![Transparent, Transparent, Transparent],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn stamp_bedroom(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    add_bed_to_room(room_stamp, rng);
    add_bedside_table_to_room(room_stamp, rng);
    match rng.range(0, 2) {
        0 => add_desk_to_room(room_stamp, rng),
        _ => {}
    };
    match rng.range(0, 2) {
        0 => add_armoire_to_room(room_stamp, rng),
        _ => {}
    };
    match rng.range(0, 2) {
        0 => add_dresser_to_room(room_stamp, rng),
        _ => {}
    };
    match rng.range(0, 2) {
        0 => add_chest_to_room(room_stamp, rng),
        _ => {}
    };
    match rng.range(0, 2) {
        0 => add_shelf_to_room(room_stamp, rng),
        _ => {}
    };
}

pub fn stamp_room(room: &mut Room, rng: &mut RandomNumberGenerator) {
    match room.room_type {
        Some(RoomType::BedRoom) => stamp_bedroom(&mut room.stamp, rng),
        _ => {}
    };
}
