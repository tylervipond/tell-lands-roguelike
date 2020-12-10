use crate::components::{Blood, Position, Renderable, Saveable};
use crate::services::BloodSpawner;
use specs::{
    saveload::{MarkerAllocator, SimpleMarker, SimpleMarkerAllocator},
    Entities, System, WriteExpect, WriteStorage,
};

pub struct BloodSpawnSystem {}
impl<'a> System<'a> for BloodSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Blood>,
        WriteExpect<'a, BloodSpawner>,
        WriteExpect<'a, SimpleMarkerAllocator<Saveable>>,
        WriteStorage<'a, SimpleMarker<Saveable>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut renderables,
            mut bloods,
            mut spawner,
            mut marker_allocator,
            mut markers,
        ) = data;
        for request in spawner.requests.iter() {
            let new_blood = entities.create();
            bloods
                .insert(new_blood, Blood {})
                .expect("failed inserting new Blood");
            positions
                .insert(
                    new_blood,
                    Position {
                        idx: request.idx,
                        level: request.level
                    },
                )
                .expect("failed inserting position for blood");
            renderables
                .insert(
                    new_blood,
                    Renderable {
                        fg: request.fg,
                        bg: request.bg,
                        glyph: request.glyph,
                        layer: 3,
                    },
                )
                .expect("failed inserting renderable for blood");
            marker_allocator.mark(new_blood, &mut markers);
        }
        spawner.requests.clear();
    }
}
