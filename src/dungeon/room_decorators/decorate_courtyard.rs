use super::super::room_feature::RoomFeature;
use super::{
    utils::find_and_replace,
    RoomPart,
    RoomPart::{Floor, Ledge, WaterDeep},
};
use rltk::{DistanceAlg::Pythagoras, Point, RandomNumberGenerator};
use stamp_rs::{
    QueryStampPart::Is,
    Stamp, StampPart,
    StampPart::{Transparent, Use},
};
use std::cmp;

fn add_circle_fountain_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let max_size = cmp::max(1, room_stamp.width() / 4);
    let size = match max_size <= 1 {
        true => 1,
        false => rng.range(1, max_size),
    } * 2
        + 1;
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
                        let distance = Pythagoras.distance2d(middle_point, Point::new(x, y));
                        if distance <= radius - 1.0 {
                            return Use(WaterDeep);
                        }
                        // 0.5 needs to be added to get all of the edges of th circle
                        if distance <= radius + 0.5 {
                            return Use(Ledge);
                        }
                        Transparent
                    })
                    .collect()
            })
            .collect(),
    );
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

fn add_rectangle_fountain_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let room_width = room_stamp.width() / 2;
    let width = match room_width > 3 {
        true => rng.range(3, room_width),
        false => 3,
    };
    let room_height = room_stamp.height() / 2;
    let height = match room_height > 3 {
        true => rng.range(3, room_height),
        false => 3,
    };
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
                1 => range_width
                    .clone()
                    .map(|w| match w {
                        0 => Transparent,
                        x if x == stamp_width - 1 => Transparent,
                        _ => Use(Ledge),
                    })
                    .collect(),
                y if y == stamp_height - 1 => range_width.clone().map(|_| Transparent).collect(),
                y if y == stamp_height - 2 => range_width
                    .clone()
                    .map(|w| match w {
                        0 => Transparent,
                        x if x == stamp_width - 1 => Transparent,
                        _ => Use(Ledge),
                    })
                    .collect(),
                _ => range_width
                    .clone()
                    .map(|w| match w {
                        0 => Transparent,
                        1 => Use(Ledge),
                        x if x == stamp_width - 1 => Transparent,
                        x if x == stamp_width - 2 => Use(Ledge),
                        _ => Use(WaterDeep),
                    })
                    .collect(),
            })
            .collect(),
    );
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn decorate_courtyard(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let fountain_style = match rng.range(0, 2) {
        0 => RoomFeature::FountainSquare,
        _ => RoomFeature::BathCircular,
    };
    for _ in 0..rng.range(10, 20) {
        match fountain_style {
            RoomFeature::FountainSquare => {
                add_rectangle_fountain_to_room(room_stamp, rng);
            }
            _ => {
                add_circle_fountain_to_room(room_stamp, rng);
            }
        }
    }
}
