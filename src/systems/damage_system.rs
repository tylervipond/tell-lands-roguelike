use crate::{components::{
    CombatStats, Hiding, Monster, Name, Player, Position, Renderable, SufferDamage,
}, player::InteractionType};
use crate::services::{BloodSpawner, DebrisSpawner, GameLog};
use rltk::RGB;
use specs::{
    Entities, Entity, Join, ReadStorage, System, World, WorldExt, WriteExpect, WriteStorage, ReadExpect
};

pub struct DamageSystem<'a>{
    pub queued_action: &'a mut Option<(Entity, InteractionType)>
}

impl<'a> DamageSystem<'a> {
    pub fn delete_the_dead(ecs: &mut World) {
        let mut dead: Vec<Entity> = Vec::new();
        {
            let combat_stats = ecs.read_storage::<CombatStats>();
            let monsters = ecs.read_storage::<Monster>();
            let renderables = ecs.read_storage::<Renderable>();
            let positions = ecs.read_storage::<Position>();
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
                        let renderable = renderables.get(entity).unwrap();
                        let position = positions.get(entity).unwrap();
                        let name = names.get(entity).unwrap();
                        debris_spawner.request(
                            position.idx,
                            renderable.fg,
                            renderable.bg,
                            35,
                            position.level,
                            format!("{} debris", name.name),
                        );
                        log.add(format!("{} has been destroyed", name.name));
                        dead.push(entity);
                    }
                }
            }
        }

        for victim in dead {
            {
                let entities = ecs.entities();
                let mut hiding = ecs.write_storage::<Hiding>();
                let hiding_to_expose: Vec<Entity> = (&entities, &mut hiding)
                    .join()
                    .filter(|(_, h)| {
                        if let Some(hiding_spot) = h.hiding_spot {
                            return hiding_spot == victim;
                        }
                        false
                    })
                    .map(|(e, _h)| e)
                    .collect();
                hiding_to_expose.iter().for_each(|e| {
                    hiding.remove(*e);
                });
            };
            ecs.delete_entity(victim).expect("Unable to delete");
        }
    }
}

impl<'a> System<'a> for DamageSystem<'a> {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, Position>,
        WriteExpect<'a, BloodSpawner>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Player>,
        ReadExpect<'a, Entity>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut stats,
            mut suffer_damage,
            positions,
            mut blood_spawner,
            monsters,
            players,
            player_ent,
        ) = data;
        for (mut stats, suffer_damage, ent) in (&mut stats, &suffer_damage, &entities).join() {
            stats.hp -= suffer_damage.amount;
            // create blood
            if monsters.get(ent).is_some() || players.get(ent).is_some() {
                let position = positions.get(ent).unwrap();
                blood_spawner.request(
                    position.idx,
                    RGB::from_f32(0.85, 0., 0.),
                    RGB::from_f32(0.50, 0., 0.),
                    177,
                    position.level,
                );
            }
            if ent == *player_ent {
                self.queued_action.take();
            }
        }
        suffer_damage.clear();
    }
}
