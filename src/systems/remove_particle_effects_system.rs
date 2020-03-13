use crate::components::particle_lifetime::ParticleLifetime;

use specs::{Entities, Join, System, ReadStorage};

pub struct RemoveParticleEffectsSystem {}

impl<'a> System<'a> for RemoveParticleEffectsSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, ParticleLifetime>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, lifetimes) = data;
        for (ent, lifetime) in (&entities, &lifetimes).join() {
            if lifetime.duration < 0.0 {
                entities.delete(ent).expect("Failed deleting ParticleLifetime");
            }
        }
    }
}
