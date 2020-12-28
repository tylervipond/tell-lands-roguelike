use super::RoomPart;
use rltk::RandomNumberGenerator;
use stamp_rs::{QueryStampPart::Is, Stamp, StampPart};

fn get_top_placement_patterns(spots: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut patterns = vec![];
    for i in 4..=6 {
        let mut pattern = vec![];
        for spot in spots.iter() {
            if spot.0 % i == 0 {
                pattern.push((spot.0, spot.1 + 1));
            }
        }
        if pattern.len() > 0 {
            patterns.push(pattern);
        }
    }
    patterns
}

fn get_bottom_placement_patterns(spots: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut patterns = vec![];
    for i in 4..=6 {
        let mut pattern = vec![];
        for spot in spots.iter() {
            if spot.0 % i == 0 {
                pattern.push((spot.0, spot.1));
            }
        }
        if pattern.len() > 0 {
            patterns.push(pattern);
        }
    }
    patterns
}

fn get_left_placement_patterns(spots: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut patterns = vec![];
    for i in 4..=6 {
        let mut pattern = vec![];
        for spot in spots.iter() {
            if spot.1 % i == 0 {
                pattern.push((spot.0 + 1, spot.1));
            }
        }
        if pattern.len() > 0 {
            patterns.push(pattern);
        }
    }
    patterns
}

fn get_right_placement_patterns(spots: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut patterns = vec![];
    for i in 4..=6 {
        let mut pattern = vec![];
        for spot in spots.iter() {
            if spot.1 % i == 0 {
                pattern.push((spot.0, spot.1));
            }
        }
        if pattern.len() > 0 {
            patterns.push(pattern);
        }
    }
    patterns
}

fn combine_patterns(
    patterns_a: &Vec<Vec<(usize, usize)>>,
    patterns_b: &Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<(usize, usize)>> {
    patterns_a
        .iter()
        .filter(|a_pattern| a_pattern.len() > 0)
        .map(|a_pattern| {
            patterns_b
                .iter()
                .filter(|b_pattern| b_pattern.len() > 0)
                .map(|b_pattern| {
                    let mut b_pattern = b_pattern.clone();
                    let mut a_pattern = a_pattern.clone();
                    a_pattern.append(&mut b_pattern);
                    a_pattern
                })
                .collect::<Vec<Vec<(usize, usize)>>>()
        })
        .flatten()
        .collect::<Vec<Vec<(usize, usize)>>>()
}

pub fn place_sconces(room_stamp: &mut Stamp<StampPart<RoomPart>>, rng: &mut RandomNumberGenerator) {
    let mut query_stamp = Stamp::new(vec![vec![
        Is(Box::new([RoomPart::Wall])),
        Is(Box::new([RoomPart::Floor])),
    ]]);
    let open_wall_spots_left = room_stamp.find(&query_stamp);
    query_stamp.rotate_90();
    let open_wall_spots_top = room_stamp.find(&query_stamp);
    query_stamp.rotate_90();
    let open_wall_spots_right = room_stamp.find(&query_stamp);
    query_stamp.rotate_90();
    let open_wall_spots_bottom = room_stamp.find(&query_stamp);
    let top_patterns = get_top_placement_patterns(&open_wall_spots_top);
    let bottom_patterns = get_bottom_placement_patterns(&open_wall_spots_bottom);
    let left_patterns = get_left_placement_patterns(&open_wall_spots_left);
    let right_patterns = get_right_placement_patterns(&open_wall_spots_right);
    let vertical_patterns = combine_patterns(&top_patterns, &bottom_patterns);
    let horizontal_patterns = combine_patterns(&left_patterns, &right_patterns);
    let all_patterns = combine_patterns(&horizontal_patterns, &vertical_patterns);
    let patterns = top_patterns
        .iter()
        .chain(bottom_patterns.iter())
        .chain(left_patterns.iter())
        .chain(right_patterns.iter())
        .chain(vertical_patterns.iter())
        .chain(horizontal_patterns.iter())
        .chain(all_patterns.iter())
        .filter(|pattern| pattern.len() > 0)
        .cloned()
        .collect::<Vec<Vec<(usize, usize)>>>();
    if patterns.len() > 0 {
        let choice = rng.range(0, patterns.len());
        match patterns.get(choice) {
            Some(pattern) => pattern.iter().for_each(|position| {
                room_stamp.set_at(*position, StampPart::Use(RoomPart::Sconce))
            }),
            _ => {}
        }
    }
}
