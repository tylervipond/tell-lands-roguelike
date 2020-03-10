use crate::components::{single_activation::SingleActivation, triggered::Triggered};
use specs::{Entities, Join, ReadStorage, System};

pub struct RemoveTriggeredTrapsSystem {}

impl<'a> System<'a> for RemoveTriggeredTrapsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, SingleActivation>,
        ReadStorage<'a, Triggered>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, single_activations, triggered) = data;
        for (e, _s, _t) in (&entities, &single_activations, &triggered).join() {
            entities
                .delete(e)
                .expect("couldn't delete triggered single activation entity");
        }
    }
}
