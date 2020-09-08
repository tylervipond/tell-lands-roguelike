use crate::components::{
    CombatStats, DungeonLevel, Monster, Name, Player, Position, Renderable, SufferDamage,
};
use crate::services::{BloodSpawner, DebrisSpawner, GameLog};
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
            let monsters = ecs.read_storage::<Monster>();
            let renderables = ecs.read_storage::<Renderable>();
            let positions = ecs.read_storage::<Position>();
            let dungeon_levels = ecs.read_storage::<DungeonLevel>();
            let players = ecs.read_storage::<Player>();
            let names = ecs.read_storage::<Name>();
            let entities = ecs.entities();
            let mut log = ecs.write_resource::<GameLog>();
            let mut debris_spawner = ecs.write_resource::<DebrisSpawner>();

            for (entity, stats, name) in (&entities, &combat_stats, &names).join() {
                if stats.hp < 1 {
                    if players.get(entity).is_some() {
                        log.add("you are dead".to_string());
                    } else if monsters.get(entity).is_some() {
                        log.add(format!("{} has died", name.name));
                        dead.push(entity);
                    } else {
                        let dungeon_level = dungeon_levels.get(entity).unwrap();
                        let renderable = renderables.get(entity).unwrap();
                        let position = positions.get(entity).unwrap();
                        let name = names.get(entity).unwrap();
                        debris_spawner.request(
                            position.x,
                            position.y,
                            renderable.fg,
                            renderable.bg,
                            35,
                            dungeon_level.level,
                            format!("{} debris", name.name),
                        );
                        log.add(format!("{} has been destroyed", name.name));
                        dead.push(entity);
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
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut stats,
            mut suffer_damage,
            positions,
            levels,
            mut blood_spawner,
            monsters,
            players,
        ) = data;
        for (mut stats, suffer_damage, ent) in (&mut stats, &suffer_damage, &entities).join() {
            stats.hp -= suffer_damage.amount;
            // create blood
            if monsters.get(ent).is_some() || players.get(ent).is_some() {
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
        }
        suffer_damage.clear();
    }
}
