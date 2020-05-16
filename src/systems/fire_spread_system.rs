use crate::components::{CombatStats, DungeonLevel, Flammable, OnFire, Position, SufferDamage};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use rltk::RandomNumberGenerator;
use specs::{
    storage::GenericWriteStorage, Entity, Join, ReadStorage, System, WriteExpect, WriteStorage,
};

pub struct FireSpreadSystem {}

impl<'a> System<'a> for FireSpreadSystem {
    type SystemData = (
        WriteStorage<'a, OnFire>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Flammable>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, DungeonLevel>,
        WriteExpect<'a, Dungeon>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, RandomNumberGenerator>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut on_fires,
            positions,
            flammables,
            combat_stats,
            dungeon_levels,
            mut dungeon,
            mut suffer_damage,
            mut rng,
        ) = data;

        let affected_entities: Vec<Entity> = (&mut on_fires, &positions, &dungeon_levels)
            .join()
            .map(|(_, position, dungeon_level)| {
                let x = position.x;
                let y = position.y;
                let coords = [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ];

                let ents: Vec<Entity> = coords
                    .iter()
                    .map(|(x, y)| {
                        let level = dungeon.get_level(dungeon_level.level).unwrap();
                        level_utils::entities_at_xy(&level, *x, *y)
                    })
                    .flatten()
                    .collect();
                ents
            })
            .flatten()
            .collect();
        affected_entities.iter().for_each(|e| {
            if let Some(_) = flammables.get(*e) {
                if rng.range(0, 2) == 1 {
                    on_fires
                        .insert(*e, OnFire {})
                        .expect("couldn't light entity on fire");
                }
            }
            if let Some(_) = combat_stats.get(*e) {
                if let Some(damage_to_suffer) = suffer_damage.get_mut_or_default(*e) {
                    damage_to_suffer.amount += 2;
                }
            }
        });
    }
}
