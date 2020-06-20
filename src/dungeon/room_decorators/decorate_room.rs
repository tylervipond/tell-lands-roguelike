use super::decorate_bedroom::decorate_bedroom;
use super::decorate_kitchen::decorate_kitchen;
use super::decorate_mess_hall::decorate_mess_hall;
use super::decorate_sitting_room::decorate_sitting_room;
use super::decorate_barracks::decorate_barracks;
use super::decorate_dining_room::decorate_dining_room;
use super::{RoomPart, RoomType};
use rltk::RandomNumberGenerator;
use stamp_rs::{Stamp, StampPart};

pub fn decorate_room(
    stamp: &mut Stamp<StampPart<RoomPart>>,
    room_type: &Option<RoomType>,
    rng: &mut RandomNumberGenerator,
) {
    match room_type {
        Some(RoomType::BedRoom) => decorate_bedroom(stamp, rng),
        Some(RoomType::MessHall) => decorate_mess_hall(stamp, rng),
        Some(RoomType::SittingRoom) => decorate_sitting_room(stamp, rng),
        Some(RoomType::Kitchen) => decorate_kitchen(stamp, rng),
        Some(RoomType::Barracks) => decorate_barracks(stamp, rng),
        Some(RoomType::DiningRoom) => decorate_dining_room(stamp, rng),
        _ => {}
    };
}
