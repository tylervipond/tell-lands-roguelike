use crate::components::{
    dungeon_level::DungeonLevel, hidden::Hidden, name::Name, viewshed::Viewshed,
};
use crate::dungeon::dungeon::Dungeon;
use crate::game_log::GameLog;
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
        let map = dungeon.get_map(dungeon_level.level).unwrap();
        let player_viewshed = viewsheds.get(*player_ent).unwrap();
        player_viewshed
            .visible_tiles
            .iter()
            .map(|p| map.entities_at_xy(p.x, p.y))
            .flatten()
            .for_each(|e| {
                if let Some(_hidden) = hidden.get(e) {
                    // roll dice
                    // if roll
                    if rng.roll_dice(1, 24) == 1 {
                        if let Some(name) = names.get(e) {
                            log.entries.push(format!("You spotted a {}.", name.name))
                        }
                        hidden.remove(e);
                    }
                }
            })
    }
}
