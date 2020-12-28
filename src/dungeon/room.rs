use super::{
    rect::Rect,
    room_decorators::{RoomPart, RoomPart::Floor, RoomType},
    room_feature::RoomFeature,
};
use crate::utils::get_random_element;
use rltk::RandomNumberGenerator;
use serde::{Deserialize, Serialize};
use stamp_rs::{Stamp, StampPart, StampPart::Use};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    pub rect: Rect,
    pub room_type: Option<RoomType>,
    pub stamp: Stamp<StampPart<RoomPart>>,
    pub features: Vec<Option<RoomFeature>>,
}

impl Room {
    pub fn new(rect: Rect) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let room_type = match rect.area() {
            0..=8 => None,
            9..=75 => {
                let choices = vec![
                    Some(RoomType::SittingRoom),
                    Some(RoomType::TreasureRoom),
                    Some(RoomType::Collapsed),
                    Some(RoomType::StoreRoom),
                    Some(RoomType::BedRoom),
                    Some(RoomType::Kitchen),
                    None,
                ];
                get_random_element(&mut rng, &choices).to_owned()
            }
            76..=100 => {
                let choices = vec![
                    Some(RoomType::TreasureRoom),
                    Some(RoomType::Collapsed),
                    Some(RoomType::StoreRoom),
                    Some(RoomType::DiningRoom),
                    None,
                ];
                get_random_element(&mut rng, &choices).to_owned()
            }
            101..=200 => {
                let mut choices = vec![Some(RoomType::MessHall), Some(RoomType::Barracks), None];
                if rect.height() >= 8 && rect.width() >= 8 {
                    choices.push(Some(RoomType::ClassRoom));
                    choices.push(Some(RoomType::MeetingRoom));
                }
                get_random_element(&mut rng, &choices).to_owned()
            }
            _ => {
                let choices = vec![
                    Some(RoomType::Courtyard),
                    Some(RoomType::Courtyard),
                    Some(RoomType::Courtyard),
                    Some(RoomType::Courtyard),
                    Some(RoomType::Baths),
                    Some(RoomType::Baths),
                    Some(RoomType::ThroneRoom),
                ];
                get_random_element(&mut rng, &choices).to_owned()
            }
        };
        let mut features = vec![];
        match &room_type {
            Some(RoomType::MessHall) => {
                let column_choices = vec![
                    Some(RoomFeature::ColumnsSingleAll),
                    Some(RoomFeature::ColumnsSingleTop),
                    Some(RoomFeature::ColumnsSingleLeft),
                    Some(RoomFeature::ColumnsSingleRight),
                    Some(RoomFeature::ColumnsSingleVertical),
                    Some(RoomFeature::ColumnsSingleHorizontal),
                    Some(RoomFeature::ColumnsSingleMiddle),
                    Some(RoomFeature::ColumnsSingleBottom),
                    Some(RoomFeature::ColumnsDoubleAll),
                    Some(RoomFeature::ColumnsDoubleTop),
                    Some(RoomFeature::ColumnsDoubleLeft),
                    Some(RoomFeature::ColumnsDoubleRight),
                    Some(RoomFeature::ColumnsDoubleVertical),
                    Some(RoomFeature::ColumnsDoubleHorizontal),
                    Some(RoomFeature::ColumnsDoubleMiddle),
                    Some(RoomFeature::ColumnsDoubleBottom),
                    None,
                ];
                features.push(get_random_element(&mut rng, &column_choices).to_owned());
            }
            Some(RoomType::Barracks) => {
                let column_choices = vec![
                    Some(RoomFeature::ColumnsSingleAll),
                    Some(RoomFeature::ColumnsSingleTop),
                    Some(RoomFeature::ColumnsSingleLeft),
                    Some(RoomFeature::ColumnsSingleRight),
                    Some(RoomFeature::ColumnsSingleVertical),
                    Some(RoomFeature::ColumnsSingleHorizontal),
                    Some(RoomFeature::ColumnsSingleMiddle),
                    Some(RoomFeature::ColumnsDoubleMiddle),
                    Some(RoomFeature::ColumnsSingleBottom),
                    None,
                ];
                features.push(get_random_element(&mut rng, &column_choices).to_owned());
            }
            Some(RoomType::BedRoom) | Some(RoomType::SittingRoom) | Some(RoomType::DiningRoom) => {
                let column_choices = vec![
                    Some(RoomFeature::ColumnsSingleTop),
                    Some(RoomFeature::ColumnsSingleLeft),
                    Some(RoomFeature::ColumnsSingleRight),
                    Some(RoomFeature::ColumnsSingleBottom),
                    None,
                ];
                features.push(get_random_element(&mut rng, &column_choices).to_owned());
            }
            Some(RoomType::ThroneRoom) => {
                let column_choices = vec![
                    Some(RoomFeature::ColumnsDoubleVertical),
                    Some(RoomFeature::ColumnsDoubleHorizontal),
                    Some(RoomFeature::ColumnsDoubleAll),
                    Some(RoomFeature::ColumnsTripleVertical),
                    Some(RoomFeature::ColumnsTripleHorizontal),
                    Some(RoomFeature::ColumnsTripleAll),
                ];
                features.push(get_random_element(&mut rng, &column_choices).to_owned());
            }
            Some(RoomType::Courtyard) | Some(RoomType::Baths) => {
                let column_choices = vec![
                    Some(RoomFeature::ColumnsDoubleVertical),
                    Some(RoomFeature::ColumnsDoubleHorizontal),
                    Some(RoomFeature::ColumnsDoubleAll),
                    Some(RoomFeature::ColumnsTripleVertical),
                    Some(RoomFeature::ColumnsTripleHorizontal),
                    Some(RoomFeature::ColumnsTripleAll),
                ];
                features.push(get_random_element(&mut rng, &column_choices).to_owned());
                let middle_column_choices = vec![
                    Some(RoomFeature::ColumnsDoubleMiddle),
                    Some(RoomFeature::ColumnsSingleMiddle),
                    None,
                ];
                features.push(get_random_element(&mut rng, &middle_column_choices).to_owned());
            }
            _ => {}
        };

        // the stamp is 1 tile larger in each direction than the rect, this is to accomodate walls that are part of other rects in the stamp.
        // TODO: this should be removed as part of a general refactor of the level generation
        let stamp = Stamp::new(
            (0..=rect.height())
                .map(|_| (0..=rect.width()).map(|_| Use(Floor)).collect())
                .collect(),
        );
        Room {
            rect,
            room_type,
            stamp,
            features,
        }
    }
}
