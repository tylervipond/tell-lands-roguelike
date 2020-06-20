use super::room_part::RoomPart;
use crate::utils::get_random_element;
use rltk::RandomNumberGenerator;
use stamp_rs::{QueryStampPart, Stamp, StampPart};

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

pub fn find_and_replace(
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
