pub fn inverse(value: f32, factor: f32, offset: f32) -> f32 {
    1.0 / (value * factor + offset)
}

pub fn above_zero(value: f32) -> f32 {
    if value > 0.0 {
        return 1.0;
    }
    0.0
}
