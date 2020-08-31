use super::decorate_barracks::decorate_barracks;
use super::decorate_baths::decorate_baths;
use super::decorate_bedroom::decorate_bedroom;
use super::decorate_collapsed_room::decorate_collapsed_room;
use super::decorate_courtyard::decorate_courtyard;
use super::decorate_dining_room::decorate_dining_room;
use super::decorate_kitchen::decorate_kitchen;
use super::decorate_mess_hall::decorate_mess_hall;
use super::decorate_sitting_room::decorate_sitting_room;
use super::decorate_store_room::decorate_store_room;
use super::decorate_treasure_room::decorate_treasure_room;
use super::decorate_throneroom::decorate_throneroom;
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
        Some(RoomType::TreasureRoom) => decorate_treasure_room(stamp, rng),
        Some(RoomType::StoreRoom) => decorate_store_room(stamp, rng),
        Some(RoomType::Collapsed) => decorate_collapsed_room(stamp, rng),
        Some(RoomType::Baths) => decorate_baths(stamp, rng),
        Some(RoomType::Courtyard) => decorate_courtyard(stamp, rng),
        Some(RoomType::ThroneRoom) => decorate_throneroom(stamp, rng),
        _ => {}
    };
}
