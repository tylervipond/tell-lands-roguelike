use crate::components::{DungeonLevel, Hidden, Name, Position, Viewshed, WantsToSearchHidden};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use crate::services::{GameLog, ParticleEffectSpawner};
use rltk::{DistanceAlg::Pythagoras, Point, RandomNumberGenerator};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct SearchForHiddenSystem {}

impl<'a> System<'a> for SearchForHiddenSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Hidden>,
        WriteStorage<'a, WantsToSearchHidden>,
        WriteExpect<'a, RandomNumberGenerator>,
        ReadStorage<'a, DungeonLevel>,
        ReadExpect<'a, Dungeon>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Viewshed>,
        WriteExpect<'a, ParticleEffectSpawner>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut hiddens,
            mut wants_to_search_hiddens,
            mut rng,
            dungeon_levels,
            dungeon,
            player_entity,
            mut log,
            names,
            positions,
            viewsheds,
            mut particle_effect_spawner,
        ) = data;
        for (entity, dungeon_level, position, viewshed, _wants_to_search) in (
            &entities,
            &dungeon_levels,
            &positions,
            &viewsheds,
            &mut wants_to_search_hiddens,
        )
            .join()
        {
            let level = dungeon.get_level(dungeon_level.level).unwrap();
            let is_player = *player_entity == entity;
            let player_level = dungeon_levels.get(*player_entity).unwrap();

            viewshed
                .visible_tiles
                .iter()
                .cloned()
                .filter(|tile_idx| {
                    Pythagoras.distance2d(Point::new(position.x, position.y), *tile_idx) <= 2.0
                })
                .for_each(|point| {
                    for hidden_entity in level_utils::entities_at_xy(level, point.x, point.y).iter()
                    {
                        if let Some(hidden) = hiddens.get_mut(*hidden_entity) {
                            match rng.range(1, 6) {
                                1 => {
                                    let inserted = hidden.found_by.insert(entity);
                                    if is_player && inserted {
                                        let hidden_name = names.get(*hidden_entity).unwrap();
                                        log.add(format!("You spotted a {}", hidden_name.name));
                                    }
                                }
                                _ => {
                                    if is_player {
                                        log.add("You found nothing.".to_string())
                                    }
                                }
                            }
                        }
                    }
                    if dungeon_level.level == player_level.level {
                        particle_effect_spawner.request_search_particle(
                            point.x,
                            point.y,
                            dungeon_level.level,
                        );
                    }
                });
        }
        wants_to_search_hiddens.clear();
    }
}
