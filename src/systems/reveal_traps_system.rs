use crate::components::{DungeonLevel, Hidden, Name, Viewshed};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use crate::services::GameLog;
use rltk::RandomNumberGenerator;
use specs::{Entity, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct RevealTrapsSystem {}

impl<'a> System<'a> for RevealTrapsSystem {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, DungeonLevel>,
        WriteExpect<'a, Dungeon>,
        ReadExpect<'a, Entity>,
        WriteStorage<'a, Hidden>,
        WriteExpect<'a, RandomNumberGenerator>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, GameLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            viewsheds,
            dungeon_levels,
            mut dungeon,
            player_ent,
            mut hidden,
            mut rng,
            names,
            mut log,
        ) = data;
        let dungeon_level = dungeon_levels.get(*player_ent).unwrap();
        let level = dungeon.get_level(dungeon_level.level).unwrap();
        let player_viewshed = viewsheds.get(*player_ent).unwrap();
        player_viewshed
            .visible_tiles
            .iter()
            .map(|p| level_utils::entities_at_xy(&level, p.x, p.y))
            .flatten()
            .for_each(|e| {
                if let Some(this_hidden) = hidden.get_mut(e) {
                    if rng.roll_dice(1, 24) == 1 {
                        let inserted = this_hidden.found_by.insert(*player_ent);
                        if inserted {
                            if let Some(name) = names.get(e) {
                                log.add(format!("You spotted a {}.", name.name));
                            }
                        }
                    }
                }
            });
    }
}
