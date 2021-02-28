use crate::components::{CausesDamage, DamageHistory, EntityMoved, EntryTrigger, Hidden, Name, Position, SufferDamage, Triggered, Viewshed};
use crate::dungeon::{dungeon::Dungeon, level_utils};
use crate::services::{GameLog, ParticleEffectSpawner};
use rltk::RandomNumberGenerator;
use specs::{
    storage::GenericWriteStorage, Entities, Entity, Join, ReadExpect, ReadStorage, System,
    WriteExpect, WriteStorage,
};

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = (
        ReadExpect<'a, Dungeon>,
        ReadExpect<'a, Entity>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, EntryTrigger>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CausesDamage>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, Triggered>,
        WriteStorage<'a, DamageHistory>,
        ReadStorage<'a, Viewshed>,
        WriteExpect<'a, ParticleEffectSpawner>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteExpect<'a, RandomNumberGenerator>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            dungeon,
            player_ent,
            mut moved,
            entry_triggers,
            positions,
            mut hidden,
            names,
            damages,
            mut suffer_damage,
            mut triggered,
            mut damage_histories,
            viewsheds,
            mut particle_spawner,
            mut log,
            ents,
            mut rng,
        ) = data;
        let player_level = positions.get(*player_ent).unwrap().level;
        let player_viewshed = viewsheds.get(*player_ent).unwrap();
        let level = dungeon.get_level(player_level).unwrap();
        for (entity, mut _ent_moved, pos) in (&ents, &mut moved, &positions).join() {
            for maybe_triggered in level_utils::entities_at_idx(&level, pos.idx)
                .iter()
                .filter(|e| *e != &entity)
            {
                if let Some(_) = entry_triggers.get(*maybe_triggered) {
                    if player_viewshed.visible_tiles.contains(&pos.idx) {
                        if let Some(triggered_name) = names.get(*maybe_triggered) {
                            if let Some(ent_name) = names.get(entity) {
                                log.add(format!(
                                    "{} triggers {}",
                                    &ent_name.name, &triggered_name.name
                                ));
                            }
                        }
                    }
                    if let Some(damage) = damages.get(*maybe_triggered) {
                        if let Some(damage_to_suffer) = suffer_damage.get_mut_or_default(entity) {
                            damage_to_suffer.amount += rng.range(damage.min, damage.max) + damage.bonus;
                            particle_spawner.request_attack_particle(
                                pos.idx,
                                pos.level,
                            );
                        }
                        if let Some(damage_history) = damage_histories.get_mut(entity) {
                            let damage_type = rng.random_slice_entry(&damage.damage_type).unwrap();
                            damage_history.events.insert(*damage_type);
                        }
                    }
                    hidden.remove(*maybe_triggered);
                    triggered
                        .insert(*maybe_triggered, Triggered {})
                        .expect("could not insert triggered for trap");
                }
            }
        }
        moved.clear();
    }
}
