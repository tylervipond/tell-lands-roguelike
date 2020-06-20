use super::{
    room_part::{
        RoomPart,
        RoomPart::{Chair, Desk, Door, Floor, Wall, Table},
    },
    utils::find_and_replace,
};
use rltk::RandomNumberGenerator;
use stamp_rs::{
    QueryStampPart,
    QueryStampPart::{Any, Is, Not},
    Stamp, StampPart,
    StampPart::{Transparent, Use},
};

pub fn replace_middle_3x3(room_part: RoomPart) -> Stamp<StampPart<RoomPart>> {
    Stamp::new(vec![
        vec![Transparent, Transparent, Transparent],
        vec![Transparent, Use(room_part), Transparent],
        vec![Transparent, Transparent, Transparent],
    ])
}

pub fn query_middle_3x3_against_wall() -> Stamp<QueryStampPart<RoomPart>> {
    Stamp::new(vec![
        vec![Any, Is(Box::new([Wall])), Any],
        vec![
            Not(Box::new([Door])),
            Is(Box::new([Floor])),
            Not(Box::new([Door])),
        ],
        vec![Any, Is(Box::new([Floor])), Any],
    ])
}

pub fn add_part_against_wall(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
    room_part: RoomPart
) {
    let mut query_stamp = query_middle_3x3_against_wall();
    let mut replace_stamp = replace_middle_3x3(room_part);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}

pub fn add_desk_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
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

pub fn add_table_to_room(
    room_stamp: &mut Stamp<StampPart<RoomPart>>,
    rng: &mut RandomNumberGenerator,
) {
    let mut query_stamp = Stamp::new(vec![
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor, Table])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor, Table])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![
            Is(Box::new([Floor, Table])),
            Is(Box::new([Floor, Table])),
            Is(Box::new([Floor, Table])),
            Is(Box::new([Floor, Table])),
            Is(Box::new([Floor, Table])),
        ],
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor, Table])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
        vec![
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor, Table])),
            Is(Box::new([Floor])),
            Is(Box::new([Floor])),
        ],
    ]);
    let mut replace_stamp = Stamp::new(vec![
        vec![
            Transparent,
            Transparent,
            Transparent,
            Transparent,
            Transparent,
        ],
        vec![
            Transparent,
            Transparent,
            Transparent,
            Transparent,
            Transparent,
        ],
        vec![
            Transparent,
            Transparent,
            Use(Table),
            Transparent,
            Transparent,
        ],
        vec![
            Transparent,
            Transparent,
            Transparent,
            Transparent,
            Transparent,
        ],
        vec![
            Transparent,
            Transparent,
            Transparent,
            Transparent,
            Transparent,
        ],
    ]);
    find_and_replace(room_stamp, rng, &mut query_stamp, &mut replace_stamp);
}
