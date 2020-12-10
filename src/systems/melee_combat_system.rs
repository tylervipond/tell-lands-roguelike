use crate::components::{CombatStats, Name, Position, SufferDamage, WantsToMelee, Equipment, CausesDamage};
use crate::services::{GameLog, ParticleEffectSpawner};
use rltk::RandomNumberGenerator;
use specs::{
  storage::GenericWriteStorage, Entities, Join, ReadStorage, System, WriteExpect, WriteStorage,
};

fn format_damage_text(attacker: &str, target: &str, weapon: &str, damage: i32) -> String {
  format!("{} hits {} with {}, for {} hp", attacker, target, weapon, damage)
}

fn format_no_damage_text(attacker: &str, target: &str, weapon: &str) -> String {
  format!("{} is unable to hurt {} with {}", attacker, target, weapon)
}

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, WantsToMelee>,
    WriteStorage<'a, SufferDamage>,
    ReadStorage<'a, Name>,
    ReadStorage<'a, CombatStats>,
    ReadStorage<'a, Equipment>,
    ReadStorage<'a, CausesDamage>,
    WriteExpect<'a, GameLog>,
    WriteExpect<'a, ParticleEffectSpawner>,
    ReadStorage<'a, Position>,
    WriteExpect<'a, RandomNumberGenerator>
  );

  fn run(&mut self, data: Self::SystemData) {
    let (
      entities,
      mut wants_to_melee,
      mut suffer_damage,
      names,
      combat_stats,
      equipment,
      causes_damage,
      mut log,
      mut particle_effect_spawner,
      positions,
      mut rng
    ) = data;

    for (entity, wants_to_melee, name, stats, equipment) in
      (&entities, &wants_to_melee, &names, &combat_stats, &equipment).join()
    {
      if stats.hp > 0 {
        let target_stats = combat_stats.get(wants_to_melee.target).unwrap();
        if target_stats.hp > 0 {
          let mut total_damage = 0;
          let target_name = names.get(wants_to_melee.target).unwrap();
          let position = positions.get(entity).unwrap();
          particle_effect_spawner.request_attack_particle(position.idx, position.level);
          // dominant hand attack
          let (dominant_weapon_name, dominant_weapon_damage) = match equipment.dominant_hand {
            Some(e) => (names.get(e), causes_damage.get(e)),
            _ => (None, None),
          };
          let dominant_hand_damage_dealt = match dominant_weapon_damage {
            Some(damage) => i32::max(0, rng.range(damage.min, damage.max + 1) + damage.bonus + stats.power - target_stats.defense),
            None => i32::max(0, stats.power - target_stats.defense), // this could be wrong, what if the hand holds a shield or torch?
          };
          let dominant_weapon_name = match dominant_weapon_name {
            Some(name) => &name.name,
            None => "fist"
          };
          if dominant_hand_damage_dealt == 0 {
            log.add(format_no_damage_text(&name.name, &target_name.name, dominant_weapon_name));
          } else {
            total_damage += dominant_hand_damage_dealt;
            log.add(format_damage_text(&name.name, &target_name.name, dominant_weapon_name, dominant_hand_damage_dealt));
          }
          // off hand attack
          let off_weapon_damage = match equipment.off_hand {
            Some(e) => causes_damage.get(e),
            _ => None,
          };
          if let Some(damage) = off_weapon_damage {
            let off_hand_damage_dealt = i32::max(0, rng.range(damage.min, damage.max + 1) + damage.bonus + stats.power - target_stats.defense);
            let off_weapon_name = &names.get(equipment.off_hand.unwrap()).unwrap().name;
            if off_hand_damage_dealt == 0 {
              log.add(format_no_damage_text(&name.name, &target_name.name, off_weapon_name));
            } else {
              total_damage += off_hand_damage_dealt;
              log.add(format_damage_text(&name.name, &target_name.name, off_weapon_name, off_hand_damage_dealt));
            }
          }
          if total_damage > 0 {
            if let Some(damage_to_suffer) = suffer_damage.get_mut_or_default(wants_to_melee.target)
            {
              damage_to_suffer.amount += total_damage;
            }
          }
        }
      }
    }
    wants_to_melee.clear();
  }
}
