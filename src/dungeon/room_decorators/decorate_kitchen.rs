use super::{
    common::{add_part_against_wall, add_table_to_room},
    RoomPart,
    RoomPart::{Barrel, Counter, Shelf, Stove, Cupboard},
};
use rltk::RandomNumberGenerator;
use stamp_rs::{Stamp, StampPart};

pub fn decorate_kitchen(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    for _ in 0..rng.range(1, 3) {
        add_table_to_room(room_stamp, rng);
    }
    for _ in 0..rng.range(1, 3) {
        add_part_against_wall(room_stamp, rng, Stove);
    }
    for _ in 0..rng.range(3, 7) {
        add_part_against_wall(room_stamp, rng, Counter);
    }
    for _ in 0..rng.range(1, 3) {
        add_part_against_wall(room_stamp, rng, Shelf);
    }
    for _ in 0..rng.range(1, 3) {
        add_part_against_wall(room_stamp, rng, Cupboard);
    }
    for _ in 0..rng.range(1, 9) {
        add_part_against_wall(room_stamp, rng, Barrel);
    }
}
