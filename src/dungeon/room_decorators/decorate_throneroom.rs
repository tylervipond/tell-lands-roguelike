use super::{RoomPart, RoomPart::Throne};
use rltk::RandomNumberGenerator;
use stamp_rs::{Stamp, StampPart, StampPart::Use};

pub fn decorate_throneroom(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    _: &mut RandomNumberGenerator,
) {
    let x = room_stamp.width() / 2;
    let y = room_stamp.height() / 2;
    room_stamp.set_at((x, y), Use(Throne))
}
