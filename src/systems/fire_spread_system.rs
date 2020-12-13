use crate::components::{CombatStats, Flammable, OnFire, Position, SufferDamage};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use rltk::RandomNumberGenerator;
use specs::{
    storage::GenericWriteStorage, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect,
    WriteStorage,
};

pub struct FireSpreadSystem {}

impl<'a> System<'a> for FireSpreadSystem {
    type SystemData = (
        WriteStorage<'a, OnFire>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Flammable>,
        ReadStorage<'a, CombatStats>,
        ReadExpect<'a, Dungeon>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, RandomNumberGenerator>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut on_fires,
            positions,
            flammables,
            combat_stats,
            dungeon,
            mut suffer_damage,
            mut rng,
        ) = data;

        let affected_entities: Vec<Entity> = (&mut on_fires, &positions)
            .join()
            .map(|(_, position)| {
                let level = dungeon.get_level(position.level).unwrap();
                level_utils::get_neighbors_for_idx(level.width as i32, position.idx as i32)
                    .iter()
                    .map(|idx| level_utils::entities_at_idx(level, *idx as usize))
                    .flatten()
                    .collect::<Vec<Entity>>()
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
