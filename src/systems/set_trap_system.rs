use crate::components::{Consumable, Position, Trap, WantsToTrap};
use crate::services::{GameLog, TrapSpawner};
use crate::types::trap_type;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct SetTrapSystem {}

impl<'a> System<'a> for SetTrapSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToTrap>,
        ReadStorage<'a, Trap>,
        WriteExpect<'a, TrapSpawner>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Consumable>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            player_entity,
            mut log,
            entities,
            mut wants_to_traps,
            traps,
            mut spawner,
            positions,
            consumables,
        ) = data;
        for (trapping_entity, trap_intent, position) in
            (&entities, &mut wants_to_traps, &positions).join()
        {
            let trap_type = traps.get(trap_intent.item).unwrap().trap_type;
            let idx = match trap_intent.target {
                Some(idx) => idx,
                None => position.idx
            };
            spawner.request(idx, position.level, trapping_entity, trap_type);
            if trapping_entity == *player_entity {
                let trap_name = trap_type::get_name_for_trap(&trap_type);
                log.add(format!("{} set.", trap_name));
            }

            if let Some(_) = consumables.get(trap_intent.item) {
                entities.delete(trap_intent.item).expect("Delete Failed");
            };
        }
        wants_to_traps.clear();
    }
}
