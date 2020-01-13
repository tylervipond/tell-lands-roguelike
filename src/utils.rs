use rltk::RandomNumberGenerator;

pub fn get_random_between_numbers(rng: &mut RandomNumberGenerator, n1: i32, n2: i32) -> i32 {
  n1 + rng.roll_dice(1, i32::abs(n2 - n1))
}
