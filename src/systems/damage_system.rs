use crate::components::{
  combat_stats::CombatStats, dungeon_level::DungeonLevel, name::Name, player::Player,
  position::Position, suffer_damage::SufferDamage,
};
use crate::game_log::GameLog;
use crate::services::blood_spawner::BloodSpawner;
use rltk::RGB;
use specs::{
  Entities, Entity, Join, ReadStorage, System, World, WorldExt, WriteExpect, WriteStorage,
};

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
    Entities<'a>,
    WriteStorage<'a, CombatStats>,
    WriteStorage<'a, SufferDamage>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, DungeonLevel>,
    WriteExpect<'a, BloodSpawner>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut stats, mut suffer_damage, positions, levels, mut blood_spawner) = data;
    for (mut stats, suffer_damage, ent) in (&mut stats, &suffer_damage, &entities).join() {
      stats.hp -= suffer_damage.amount;
      // create blood
      let position = positions.get(ent).unwrap().clone();
      let level = levels.get(ent).unwrap().clone();
      blood_spawner.request(
        position.x,
        position.y,
        RGB::from_f32(0.85, 0., 0.),
        RGB::from_f32(0.50, 0., 0.),
        177,
        level.level,
      );
    }
    suffer_damage.clear();
  }
}
