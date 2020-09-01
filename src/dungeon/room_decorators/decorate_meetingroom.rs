use super::{
    RoomPart,
    RoomPart::{Chair, Podium, Table, Wall},
};
use rltk::RandomNumberGenerator;
use stamp_rs::{
    Stamp, StampPart,
    StampPart::{Transparent, Use},
};

const PODIUM_STAMP_WIDTH: usize = 4;

fn stamp_tables(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    let height = (room_stamp.height() - PODIUM_STAMP_WIDTH) / 3 * 3;
    let width = room_stamp.width();
    let middle = width / 2;
    let desks_stamp = Stamp::new(
        (1..height - 1)
            .map(|y| match y % 3 {
                0 => (1..width - 1)
                    .map(|x| {
                        if x == middle
                            || room_stamp.get_at((x, y)) == Some(&Use(Wall))
                            || room_stamp.get_at((x, y - 1)) == Some(&Use(Wall))
                        {
                            return Transparent;
                        }
                        Use(Table)
                    })
                    .collect(),
                2 => (1..width - 1)
                    .map(|x| {
                        if x == middle
                            || room_stamp.get_at((x, y)) == Some(&Use(Wall))
                            || room_stamp.get_at((x, y + 1)) == Some(&Use(Wall))
                        {
                            return Transparent;
                        }
                        match rng.range(0, 3) {
                            0 => Transparent,
                            _ => Use(Chair),
                        }
                    })
                    .collect(),
                _ => (1..width - 1).map(|_| Transparent).collect(),
            })
            .collect(),
    );
    room_stamp.stamp(&desks_stamp, 1, 1);
}

fn stamp_podium(room_stamp: &mut Stamp<StampPart<RoomPart>>) {
    let stamp_width = room_stamp.width();
    let stamp_height = room_stamp.height();
    let teachers_set_stamp = Stamp::new(vec![vec![Use(Podium)]]);
    room_stamp.stamp(&teachers_set_stamp, stamp_width / 2, stamp_height - 3);
}

fn place_south(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    stamp_tables(room_stamp, rng);
    stamp_podium(room_stamp);
}

fn place_north(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    room_stamp.rotate_180();
    stamp_tables(room_stamp, rng);
    stamp_podium(room_stamp);
    room_stamp.rotate_180();
}

fn place_east(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    room_stamp.rotate_n90();
    stamp_tables(room_stamp, rng);
    stamp_podium(room_stamp);
    room_stamp.rotate_90();
}

fn place_west(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    room_stamp.rotate_90();
    stamp_tables(room_stamp, rng);
    stamp_podium(room_stamp);
    room_stamp.rotate_n90();
}
pub fn decorate_meetingroom(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    match rng.range(0, 4) {
        0 => {
            place_north(room_stamp, rng);
        }
        1 => {
            place_east(room_stamp, rng);
        }
        2 => {
            place_south(room_stamp, rng);
        }
        _ => {
            place_west(room_stamp, rng);
        }
    };
}
