use crate::components::{Grabbable, Grabbing, Name, Position, WantsToReleaseGrabbed};
use crate::services::GameLog;
use crate::dungeon::dungeon::Dungeon;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct ReleaseSystem {}

impl<'a> System<'a> for ReleaseSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToReleaseGrabbed>,
        WriteStorage<'a, Grabbing>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Grabbable>,
        ReadStorage<'a, Name>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadExpect<'a, Dungeon>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut release_intents,
            mut grabbings,
            positions,
            grabbables,
            names,
            player_entity,
            mut log,
            dungeon,
        ) = data;
        let grabbings_to_release: Vec<Entity> = (&entities, (&mut release_intents).maybe(), (&grabbings).maybe())
            .join()
            .filter(|(entity, release_intent, grabbing)| {
                let out_of_sync = match grabbing {
                    Some(grabbing) => {
                        let ent_position = positions.get(*entity).unwrap();
                        let level = dungeon.get_level(ent_position.level).unwrap();
                        match positions.get(grabbing.thing) {
                            Some(pos) => {
                                let neighbor_x =
                                    pos.idx == ent_position.idx + 1 || pos.idx == ent_position.idx - 1;
                                let neighbor_y =
                                    pos.idx == ent_position.idx + level.width as usize || pos.idx == ent_position.idx - level.width as usize;
                                !(neighbor_y || neighbor_x)
                            }
                            None => false,
                        }
                    }
                    None => false,
                };
                let wants_to_release = match release_intent {
                    Some(_) => true,
                    None => false,
                };
                let no_longer_grabbable = match grabbing {
                    Some(grabbing) => match grabbables.get(grabbing.thing) {
                        Some(_) => false,
                        None => true,
                    },
                    None => false,
                };
                wants_to_release || no_longer_grabbable || out_of_sync
            })
            .map(|(e, _, _)| e)
            .collect();
        grabbings_to_release.iter().for_each(|entity| {
            if *entity == *player_entity {
                let name = match grabbings.get(*entity) {
                    Some(grabbing) => match names.get(grabbing.thing) {
                        Some(name) => name.name.to_owned(),
                        None => "nothing".to_string(),
                    },
                    None => "nothing".to_string(),
                };
                log.add(format!("You release {}", name));
            }
            grabbings.remove(*entity);
        });
        release_intents.clear();
    }
}
