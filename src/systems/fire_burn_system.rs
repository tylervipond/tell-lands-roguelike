use crate::components::{flammable::Flammable, on_fire::OnFire};
use specs::{ReadStorage, System, WriteStorage, Join};

pub struct FireBurnSystem {}

impl<'a> System<'a> for FireBurnSystem {
    type SystemData = (ReadStorage<'a, OnFire>, WriteStorage<'a, Flammable>);

    fn run(&mut self, data: Self::SystemData) {
        let (on_fires, mut flammables) = data;
        for (_on_fire, flammable) in (&on_fires, &mut flammables).join() {
            flammable.turns_remaining -= 1;
        }
    }
}
