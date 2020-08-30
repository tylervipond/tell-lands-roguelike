use super::super::room_feature::RoomFeature;
use super::{
    common::replace_middle_3x3,
    utils::find_and_replace,
    RoomPart,
    RoomPart::{Chest, Column, Floor, TowelRack, Wall, WaterDeep},
};
use rltk::{DistanceAlg::Pythagoras, Point, RandomNumberGenerator};
use stamp_rs::{
    QueryStampPart::{Any, Is},
    Stamp, StampPart,
    StampPart::{Transparent, Use},
};

fn add_circle_bath_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let size = rng.range(1, 4) * 2 + 1;
    let stamp_size = size + 2;
    let range_size = 0..stamp_size;

    let mut query_stamp = Stamp::new(
        range_size
            .clone()
            .map(|_| range_size.clone().map(|__| Is(Box::new([Floor]))).collect())
            .collect(),
    );
    let middle = stamp_size / 2;
    let middle_point = Point::new(middle, middle);
    let radius = Pythagoras.distance2d(middle_point, Point::new(1, middle));
    let mut replace_stamp = Stamp::new(
        range_size
            .clone()
            .map(|y| {
                range_size
                    .clone()
                    .map(|x| {
                        if Pythagoras.distance2d(middle_point, Point::new(x, y)) <= radius {
                            return Use(WaterDeep);
                        }
                        Transparent
                    })
                    .collect()
            })
            .collect(),
    );
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

fn add_rectangle_bath_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let width = rng.range(3, 6);
    let height = rng.range(3, 6);
    let stamp_height = height + 2;
    let stamp_width = width + 2;
    let range_height = 0..stamp_height;
    let range_width = 0..stamp_width;
    let mut query_stamp = Stamp::new(
        range_height
            .clone()
            .map(|_| {
                range_width
                    .clone()
                    .map(|__| Is(Box::new([Floor])))
                    .collect()
            })
            .collect(),
    );

    let mut replace_stamp = Stamp::new(
        range_height
            .map(|h| match h {
                0 => range_width.clone().map(|_| Transparent).collect(),
                y if y == stamp_height - 1 => range_width.clone().map(|__| Transparent).collect(),
                _ => range_width
                    .clone()
                    .map(|w| match w {
                        0 => Transparent,
                        x if x == stamp_width - 1 => Transparent,
                        _ => Use(WaterDeep),
                    })
                    .collect(),
            })
            .collect(),
    );
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

fn add_towel_racks_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![Any, Is(Box::new([Wall, Column])), Any],
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
    let mut replace_stamp = replace_middle_3x3(TowelRack);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}
fn add_chests_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![Any, Is(Box::new([Wall, Column])), Any],
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
    let mut replace_stamp = replace_middle_3x3(Chest);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}
pub fn decorate_baths(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let bath_style = match rng.range(0, 2) {
        0 => RoomFeature::BathSquare,
        _ => RoomFeature::BathCircular,
    };
    for _ in 0..rng.range(10, 20) {
        match bath_style {
            RoomFeature::BathSquare => {
                add_rectangle_bath_to_room(room_stamp, rng);
            }
            _ => {
                add_circle_bath_to_room(room_stamp, rng);
            }
        }
    }
    for _ in 0..rng.range(2, 8) {
        add_towel_racks_to_room(room_stamp, rng);
    }
    for _ in 0..rng.range(2, 10) {
        add_chests_to_room(room_stamp, rng);
    }
}
