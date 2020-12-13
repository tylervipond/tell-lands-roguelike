use crate::components::{Hidden, Name, Position, Viewshed, WantsToSearchHidden};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use crate::services::{GameLog, ParticleEffectSpawner};
use rltk::RandomNumberGenerator;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct SearchForHiddenSystem {}

impl<'a> System<'a> for SearchForHiddenSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Hidden>,
        WriteStorage<'a, WantsToSearchHidden>,
        WriteExpect<'a, RandomNumberGenerator>,
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
            dungeon,
            player_entity,
            mut log,
            names,
            positions,
            viewsheds,
            mut particle_effect_spawner,
        ) = data;
        let player_level = positions.get(*player_entity).unwrap().level;

        for (entity, position, viewshed, _wants_to_search) in (
            &entities,
            &positions,
            &viewsheds,
            &mut wants_to_search_hiddens,
        )
            .join()
        {
            let level = dungeon.get_level(position.level).unwrap();
            let is_player = *player_entity == entity;
            viewshed
                .visible_tiles
                .iter()
                .cloned()
                .filter(|tile_idx| {
                    level_utils::get_distance_between_idxs(level, position.idx, *tile_idx as usize) <= 2.0
                })
                .for_each(|tile_idx| {
                    for hidden_entity in level_utils::entities_at_idx(level, tile_idx as usize).iter()
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
                    if position.level == player_level {
                        particle_effect_spawner.request_search_particle(
                            tile_idx as usize,
                            position.level,
                        );
                    }
                });
        }
        wants_to_search_hiddens.clear();
    }
}
