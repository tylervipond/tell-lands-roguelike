use crate::components::{
  combat_stats::CombatStats, name::Name, consumable::Consumable, wants_to_use::WantsToUse, provides_healing::ProvidesHealing
};
use crate::game_log::GameLog;
use specs::{Entities, Entity, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage, Join};

pub struct UseItemSystem {}

impl<'a> System<'a> for UseItemSystem {
  type SystemData = (
    ReadExpect<'a, Entity>,
    WriteExpect<'a, GameLog>,
    Entities<'a>,
    WriteStorage<'a, WantsToUse>,
    ReadStorage<'a, Name>,
    ReadStorage<'a, Consumable>,
    WriteStorage<'a, CombatStats>,
    ReadStorage<'a, ProvidesHealing>
  );
  fn run(&mut self, data: Self::SystemData) {
    let (
      player_entity,
      mut game_log,
      entities,
      mut wants_to_use,
      names,
      consumables,
      mut combat_stats,
      provides_healing
    ) = data;
    for (entity, to_use, stats) in (&entities, &wants_to_use, &mut combat_stats).join() {
      if let Some(heals) = provides_healing.get(to_use.item) {
        stats.hp = i32::min(stats.max_hp, stats.hp + heals.amount);
          if entity == *player_entity {
            game_log.entries.insert(
              0,
              format!(
                "You to_use the {}, healing {} hp.",
                names.get(to_use.item).unwrap().name,
                heals.amount
              ),
            );
          }
      }
      if let Some(_) = consumables.get(to_use.item) {
        entities.delete(to_use.item).expect("Delete Failed");
      };
    }
    wants_to_use.clear();
  }
}
