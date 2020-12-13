use crate::components::{Position, Equipment, CausesLight};
use crate::dungeon::{constants::MAP_COUNT, dungeon::Dungeon, level_utils};
use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};
pub struct LightSystem {}

impl<'a> System<'a> for LightSystem {
    type SystemData = (
        WriteExpect<'a, Dungeon>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Equipment>,
        ReadStorage<'a, CausesLight>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut dungeon, position, equipment, causes_light) = data;
        dungeon
            .levels
            .iter_mut()
            .for_each(|(_number, level)| level.lit_tiles = vec![false; MAP_COUNT]);

        for (equipment, position) in (&equipment, &position).join() {
            let level = dungeon.get_level_mut(position.level).unwrap();

            let light_radius = {
                let dominant_hand_light_range = match equipment.dominant_hand {
                    Some(item) => match causes_light.get(item) {
                        Some(light) => light.radius,
                        None => 0,
                    },
                    None => 0
                };
                let off_hand_light_range = match equipment.off_hand {
                    Some(item) => match causes_light.get(item) {
                        Some(light) => light.radius,
                        None => 0,
                    },
                    None => 0
                };
                i32::max(dominant_hand_light_range as i32, off_hand_light_range as i32)
            };
            let level_width = level.width as i32;
            let position_idx = position.idx as i32;
            let lit_points = (-light_radius..=light_radius)
                .map(|y| {
                    (-light_radius..=light_radius)
                        .map(move |x| level_utils::add_xy_to_idx(level_width, x, y, position_idx))
                })
                .flatten()
                .filter(|idx| level_utils::get_distance_between_idxs(level, position.idx, *idx as usize) < light_radius as f32)
                .collect::<Vec<i32>>();
            for index in lit_points {
                match level.lit_tiles.get_mut(index as usize) {
                    Some(i) => {
                        *i = true;
                    }
                    _ => {}
                }
            }
        }
    }
}
