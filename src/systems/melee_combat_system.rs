use crate::components::{
  combat_stats::CombatStats, name::Name, suffer_damage::SufferDamage, wants_to_melee::WantsToMelee,
};
use crate::game_log::GameLog;
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, WantsToMelee>,
    WriteStorage<'a, SufferDamage>,
    ReadStorage<'a, Name>,
    ReadStorage<'a, CombatStats>,
    WriteExpect<'a, GameLog>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut wants_to_melee, mut suffer_damage, names, combat_stats, mut log) = data;

    for (_entity, wants_to_melee, name, stats) in
      (&entities, &wants_to_melee, &names, &combat_stats).join()
    {
      if stats.hp > 0 {
        let target_stats = combat_stats.get(wants_to_melee.target).unwrap();
        if target_stats.hp > 0 {
          let target_name = names.get(wants_to_melee.target).unwrap();
          let damage = i32::max(0, stats.power - target_stats.defense);
          if damage == 0 {
            log.entries.insert(
              0,
              format!("{} is unable to hurt {}", &name.name, &target_name.name),
            );
          } else {
            log.entries.insert(
              0,
              format!(
                "{} hits {}, for {} hp",
                &name.name, &target_name.name, damage
              ),
            );
            suffer_damage
              .insert(wants_to_melee.target, SufferDamage { amount: damage })
              .expect("Unable to do damage");
          }
        }
      }
    }
    wants_to_melee.clear();
  }
}
