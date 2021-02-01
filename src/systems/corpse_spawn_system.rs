use crate::components::{Name, Position, Renderable, Saveable};
use crate::services::CorpseSpawner;
use specs::{
    saveload::{MarkerAllocator, SimpleMarker, SimpleMarkerAllocator},
    Entities, System, WriteExpect, WriteStorage,
};

pub struct CorpseSpawnSystem {}
impl<'a> System<'a> for CorpseSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Name>,
        WriteExpect<'a, CorpseSpawner>,
        WriteExpect<'a, SimpleMarkerAllocator<Saveable>>,
        WriteStorage<'a, SimpleMarker<Saveable>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut renderables,
            mut names,
            mut spawner,
            mut marker_allocator,
            mut markers,
        ) = data;
        for request in spawner.requests.iter() {
            let new_corpse = entities.create();
            names
                .insert(new_corpse, Name { name: request.name.clone() })
                .expect("failed inserting new corpse");
            positions
                .insert(
                    new_corpse,
                    Position {
                        idx: request.idx,
                        level: request.level,
                    },
                )
                .expect("failed inserting position for corpse");
            renderables
                .insert(
                    new_corpse,
                    Renderable {
                        fg: request.fg,
                        bg: request.bg,
                        glyph: request.glyph,
                        layer: 2,
                    },
                )
                .expect("failed inserting renderable for corpse");
            marker_allocator.mark(new_corpse, &mut markers);
        }
        spawner.requests.clear();
    }
}
