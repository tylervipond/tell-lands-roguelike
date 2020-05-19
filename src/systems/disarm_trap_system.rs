use crate::components::{DungeonLevel, Position, Trap, WantsToDisarmTrap};
use crate::services::{GameLog, ItemSpawner};
use crate::types::{item_type, ItemType, TrapType};
use rltk::RandomNumberGenerator;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct DisarmTrapSystem {}

impl<'a> System<'a> for DisarmTrapSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToDisarmTrap>,
        WriteExpect<'a, RandomNumberGenerator>,
        WriteExpect<'a, ItemSpawner>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, DungeonLevel>,
        ReadStorage<'a, Trap>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut wants_to_disarm_traps,
            mut rng,
            mut spawner,
            positions,
            levels,
            traps,
            player_entity,
            mut log,
        ) = data;
        for (entity, intent) in (&entities, &mut wants_to_disarm_traps).join() {
            let trap_type = traps.get(intent.trap).unwrap().trap_type;
            let item_type = match trap_type {
                TrapType::BearTrap => ItemType::BearTrap,
                _ => ItemType::Caltrops,
            };
            let item_name = item_type::get_name_for_item(&item_type);
            match rng.range(0, 6) {
                0 => {
                    log.add(format!("You failed to disarm the {}.", item_name));
                }
                _ => {
                    let position = positions.get(intent.trap).unwrap();
                    let level = levels.get(intent.trap).unwrap().level;
                    spawner.request(position.x, position.y, level, item_type);
                    if *player_entity == entity {
                        log.add(format!("You disarmed the {}.", item_name));
                    }
                    entities
                        .delete(intent.trap)
                        .expect("Couldn't delete disarmed trap entity.");
                }
            }
        }
        wants_to_disarm_traps.clear();
    }
}
