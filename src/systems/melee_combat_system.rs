use crate::components::{
  combat_stats::CombatStats, dungeon_level::DungeonLevel, name::Name, position::Position,
  suffer_damage::SufferDamage, wants_to_melee::WantsToMelee,
};
use crate::game_log::GameLog;
use crate::services::particle_effect_spawner::ParticleEffectSpawner;
use specs::{
  storage::GenericWriteStorage, Entities, Join, ReadStorage, System, WriteExpect, WriteStorage,
};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, WantsToMelee>,
    WriteStorage<'a, SufferDamage>,
    ReadStorage<'a, Name>,
    ReadStorage<'a, CombatStats>,
    WriteExpect<'a, GameLog>,
    WriteExpect<'a, ParticleEffectSpawner>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, DungeonLevel>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (
      entities,
      mut wants_to_melee,
      mut suffer_damage,
      names,
      combat_stats,
      mut log,
      mut particle_effect_spawner,
      positions,
      levels,
    ) = data;

    for (entity, wants_to_melee, name, stats) in
      (&entities, &wants_to_melee, &names, &combat_stats).join()
    {
      if stats.hp > 0 {
        let target_stats = combat_stats.get(wants_to_melee.target).unwrap();
        if target_stats.hp > 0 {
          let target_name = names.get(wants_to_melee.target).unwrap();
          let damage = i32::max(0, stats.power - target_stats.defense);
          let position = positions.get(entity).unwrap();
          let level = levels.get(entity).unwrap();
          particle_effect_spawner.request_attack_particle(position.x, position.y, level.level);
          if damage == 0 {
            log.add(format!(
              "{} is unable to hurt {}",
              &name.name, &target_name.name
            ));
          } else {
            log.add(format!(
              "{} hits {}, for {} hp",
              &name.name, &target_name.name, damage
            ));
            if let Some(damage_to_suffer) = suffer_damage.get_mut_or_default(wants_to_melee.target)
            {
              damage_to_suffer.amount += damage;
            }
          }
        }
      }
    }
    wants_to_melee.clear();
  }
}
