use crate::components::{Flammable, Grabbable, Name, Position, Renderable, Saveable};
use crate::services::DebrisSpawner;
use specs::{
    saveload::{MarkerAllocator, SimpleMarker, SimpleMarkerAllocator},
    Entities, System, WriteExpect, WriteStorage,
};

pub struct DebrisSpawnSystem {}
impl<'a> System<'a> for DebrisSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Grabbable>,
        WriteStorage<'a, Flammable>,
        WriteExpect<'a, DebrisSpawner>,
        WriteExpect<'a, SimpleMarkerAllocator<Saveable>>,
        WriteStorage<'a, SimpleMarker<Saveable>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut renderables,
            mut names,
            mut grababbles,
            mut flammables,
            mut spawner,
            mut marker_allocator,
            mut markers,
        ) = data;
        for request in spawner.requests.iter() {
            let new_debris = entities.create();
            names
                .insert(
                    new_debris,
                    Name {
                        name: request.name.clone(),
                    },
                )
                .expect("failed inserting new Name for debris");
            positions
                .insert(
                    new_debris,
                    Position {
                        idx: request.idx,
                        level: request.level
                    },
                )
                .expect("failed inserting Position for debris");
            renderables
                .insert(
                    new_debris,
                    Renderable {
                        fg: request.fg,
                        bg: request.bg,
                        glyph: request.glyph,
                        layer: 3,
                    },
                )
                .expect("failed inserting renderable for debris");
            grababbles
                .insert(new_debris, Grabbable {})
                .expect("failed inserting grabbable for debris");
            if request.flammable {
                flammables
                    .insert(new_debris, Flammable { turns_remaining: 4 })
                    .expect("failed inserting flammable for debris");
            }
            marker_allocator.mark(new_debris, &mut markers);
        }
        spawner.requests.clear();
    }
}
