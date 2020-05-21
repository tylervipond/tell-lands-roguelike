use crate::components::{Grabbable, Grabbing, Name, Position, WantsToReleaseGrabbed};
use crate::services::GameLog;
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
        ) = data;
        let grabbings_to_release: Vec<Entity> = (&entities, (&mut release_intents).maybe())
            .join()
            .filter(|(entity, release_intent)| {
                let out_of_sync = match grabbings.get(*entity) {
                    Some(grabbing) => {
                        let ent_position = positions.get(*entity).unwrap();
                        match positions.get(grabbing.thing) {
                            Some(pos) => {
                                let same_x = pos.x == ent_position.x;
                                let same_y = pos.y == ent_position.y;
                                let neighbor_x =
                                    pos.x == ent_position.x + 1 || pos.x == ent_position.x - 1;
                                let neighbor_y =
                                    pos.y == ent_position.y + 1 || pos.y == ent_position.y - 1;
                                !(same_x && neighbor_y || same_y && neighbor_x)
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
                let no_longer_grabbable = match grabbings.get(*entity) {
                    Some(grabbing) => match grabbables.get(grabbing.thing) {
                        Some(_) => false,
                        None => true,
                    },
                    None => false,
                };
                wants_to_release || no_longer_grabbable || out_of_sync
            })
            .map(|(e, _)| e)
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
