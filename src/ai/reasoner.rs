use super::curve;

pub fn attack_weight(enemy_health: i32) -> f32 {
    curve::above_zero(enemy_health as f32)
}

pub fn chase_weight(enemy_health: i32, distance: i32) -> f32 {
    curve::inverse(
        enemy_health as f32,
        1.0,
        curve::inverse(distance as f32, 1.0, 0.0),
    )
}

pub fn move_weight(distance: i32, offset: f32) -> f32 {
    1.0 / (distance as f32 + offset)
}
