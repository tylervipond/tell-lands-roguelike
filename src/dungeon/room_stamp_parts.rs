use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RoomPart {
    Floor = 0,
    Wall = 1,
    Door = 2,
    DownStairs = 3,
    UpStairs = 4,
    Exit = 5,
    Bed = 6,
    Armoire = 7,
    Dresser = 8,
    BedsideTable = 9,
    Chest = 10,
    Chair = 11,
    Desk = 12,
    Shelf = 13,
}
