use crate::components::{
  combat_stats::CombatStats, name::Name, player::Player, suffer_damage::SufferDamage,
};
use crate::game_log::GameLog;
use specs::{Entity, Join, System, World, WorldExt, WriteStorage};

pub struct DamageSystem {}

impl DamageSystem {
  pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    {
      let combat_stats = ecs.read_storage::<CombatStats>();
      let players = ecs.read_storage::<Player>();
      let names = ecs.read_storage::<Name>();
      let entities = ecs.entities();
      let mut log = ecs.write_resource::<GameLog>();
      for (entity, stats, name) in (&entities, &combat_stats, &names).join() {
        if stats.hp < 1 {
          let player = players.get(entity);
          match player {
            Some(_) => log.entries.insert(0, "you are dead".to_string()),
            None => {
              log.entries.insert(0, format!("{} has died", name.name));
              dead.push(entity);
            }
          }
        }
      }
    }
    for victim in dead {
      ecs.delete_entity(victim).expect("Unable to delete");
    }
  }
}

impl<'a> System<'a> for DamageSystem {
  type SystemData = (
    WriteStorage<'a, CombatStats>,
    WriteStorage<'a, SufferDamage>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut stats, mut suffer_damage) = data;
    for (mut stats, suffer_damage) in (&mut stats, &suffer_damage).join() {
      stats.hp -= suffer_damage.amount;
    }
    suffer_damage.clear();
  }
}
