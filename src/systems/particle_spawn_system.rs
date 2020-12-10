use crate::components::{ParticleLifetime, Position, Renderable};
use crate::services::particle_effect_spawner::ParticleEffectSpawner;
use specs::{Entities, System, WriteExpect, WriteStorage};

pub struct ParticleSpawnSystem {}
impl<'a> System<'a> for ParticleSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, ParticleLifetime>,
        WriteStorage<'a, Renderable>,
        WriteExpect<'a, ParticleEffectSpawner>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, mut lifetimes, mut renderables, mut spawner) =
            data;
        for request in spawner.requests.iter() {
            let p = entities.create();
            positions
                .insert(
                    p,
                    Position {
                        idx: request.idx,
                        level: request.level
                    },
                )
                .expect("failed inserting position for particle");
            renderables
                .insert(
                    p,
                    Renderable {
                        fg: request.fg,
                        bg: request.bg,
                        glyph: request.glyph,
                        layer: 0,
                    },
                )
                .expect("failed inserting renderable for particle");
            lifetimes
                .insert(
                    p,
                    ParticleLifetime {
                        duration: request.lifetime,
                    },
                )
                .expect("failed inserting lifetime for particle");
        }
        spawner.requests.clear();
    }
}
