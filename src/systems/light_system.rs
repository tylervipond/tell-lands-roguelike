use crate::components::{DungeonLevel, Light, Position};
use crate::dungeon::{constants::MAP_COUNT, dungeon::Dungeon, level_utils};
use rltk::{DistanceAlg::Pythagoras, Point};
use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};
pub struct LightSystem {}

impl<'a> System<'a> for LightSystem {
    type SystemData = (
        WriteExpect<'a, Dungeon>,
        ReadStorage<'a, Light>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, DungeonLevel>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut dungeon, light, position, dungeon_levels) = data;
        dungeon
            .levels
            .iter_mut()
            .for_each(|(_number, level)| level.lit_tiles = vec![false; MAP_COUNT]);

        for (light, position, dungeon_level) in (&light, &position, &dungeon_levels).join() {
            let level = dungeon.get_level_mut(dungeon_level.level).unwrap();
            let start_point = Point::new(position.x, position.y);
            let lit_points = ((position.y - light.range as i32)
                ..=(position.y + light.range as i32))
                .map(|y| {
                    ((position.x - light.range as i32)..=(position.x + light.range as i32))
                        .map(move |x| Point::new(x, y))
                })
                .flatten()
                .filter(|p| Pythagoras.distance2d(start_point, *p) < light.range as f32)
                .map(|p| level_utils::xy_idx(level.width as i32, p.x, p.y))
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
