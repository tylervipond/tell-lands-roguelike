use crate::components::{CausesLight, Dousable, Lightable, Name, WantsToLight};
use crate::services::GameLog;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct LightItemSystem {}

impl<'a> System<'a> for LightItemSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToLight>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, CausesLight>,
        WriteStorage<'a, Dousable>,
        WriteStorage<'a, Lightable>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player_entity,
            mut game_log,
            mut wants_to_light,
            names,
            mut causes_light,
            mut dousables,
            mut lightables,
        ) = data;
        for (intent, entity) in (&wants_to_light, &entities).join() {
            if let Some(light) = causes_light.get_mut(intent.item) {
                if lightables.get(intent.item).is_some() {
                    light.lit = true;
                    lightables.remove(intent.item);
                    dousables
                        .insert(intent.item, Dousable {})
                        .expect("could not insert dousable");
                    let item_name = match names.get(intent.item) {
                        Some(name) => name.name.clone(),
                        _ => "unknown".to_string(),
                    };
                    if entity == *player_entity {
                        game_log.add(format!("you light the {}", item_name));
                    }
                }
                
            }
        }

        wants_to_light.clear();
    }
}
