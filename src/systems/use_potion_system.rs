use crate::components::{
  combat_stats::CombatStats, name::Name, potion::Potion, wants_to_drink_potion::WantsToDrinkPotion,
};
use crate::game_log::GameLog;
use specs::{Entities, Entity, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage, Join};

pub struct UsePotionSystem {}

impl<'a> System<'a> for UsePotionSystem {
  type SystemData = (
    ReadExpect<'a, Entity>,
    WriteExpect<'a, GameLog>,
    Entities<'a>,
    WriteStorage<'a, WantsToDrinkPotion>,
    ReadStorage<'a, Name>,
    ReadStorage<'a, Potion>,
    WriteStorage<'a, CombatStats>,
  );
  fn run(&mut self, data: Self::SystemData) {
    let (
      player_entity,
      mut game_log,
      entities,
      mut wants_to_drink,
      names,
      potions,
      mut combat_stats,
    ) = data;
    for (entity, drink, stats) in (&entities, &wants_to_drink, &mut combat_stats).join() {
      let potion = potions.get(drink.potion);
      match potion {
        None => {}
        Some(potion) => {
          stats.hp = i32::min(stats.max_hp, stats.hp + potion.heal_amount);
          if entity == *player_entity {
            game_log.entries.insert(
              0,
              format!(
                "You drink the {}, healing {} hp.",
                names.get(drink.potion).unwrap().name,
                potion.heal_amount
              ),
            );
          }
          entities.delete(drink.potion).expect("Delete Failed");
        }
      }
    }
    wants_to_drink.clear();
  }
}
