use crate::components::{CausesLight, Dousable, Lightable, Name, WantsToDouse};
use crate::services::GameLog;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct DouseItemSystem {}

impl<'a> System<'a> for DouseItemSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToDouse>,
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
            mut wants_to_douse,
            names,
            mut causes_light,
            mut dousables,
            mut lightables,
        ) = data;
        for (intent, entity) in (&wants_to_douse, &entities).join() {
            if let Some(light) = causes_light.get_mut(intent.item) {
                if dousables.get(intent.item).is_some() {
                    light.lit = false;
                    dousables.remove(intent.item);
                    lightables.insert(intent.item, Lightable {}).expect("could not insert lightable");
                    let item_name = match names.get(intent.item) {
                        Some(name) => name.name.clone(),
                        _ => "unknown".to_string(),
                    };
                    if entity == *player_entity {
                        game_log.add(format!("you douse the {}", item_name));
                    }
                }
            }
        }

        wants_to_douse.clear();
    }
}
