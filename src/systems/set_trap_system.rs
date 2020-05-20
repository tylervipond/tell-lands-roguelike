use crate::components::{Consumable, DungeonLevel, Position, Trap, WantsToTrap};
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
        ReadStorage<'a, DungeonLevel>,
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
            dungeon_levels,
            positions,
            consumables,
        ) = data;
        for (trapping_entity, trap_intent, dungeon_level) in
            (&entities, &mut wants_to_traps, &dungeon_levels).join()
        {
            let trap_type = traps.get(trap_intent.item).unwrap().trap_type;
            let (x, y) = match trap_intent.target {
                Some(point) => (point.x, point.y),
                None => {
                    let pos = positions.get(trapping_entity).unwrap();
                    (pos.x, pos.y)
                }
            };
            spawner.request(x, y, dungeon_level.level, trapping_entity, trap_type);
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
