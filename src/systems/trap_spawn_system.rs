use crate::components::{
    DungeonLevel, EntryTrigger, Hidden, InflictsDamage, Name, Position, Renderable, Saveable,
    SingleActivation, Trap,
};
use crate::entity_set::EntitySet;
use crate::services::TrapSpawner;
use crate::types::trap_type;
use rltk::{BLACK, RED, RGB};
use specs::{
    saveload::{MarkerAllocator, SimpleMarker, SimpleMarkerAllocator},
    Entities, System, WriteExpect, WriteStorage,
};

pub struct TrapSpawnSystem {}
impl<'a> System<'a> for TrapSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, DungeonLevel>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Hidden>,
        WriteStorage<'a, EntryTrigger>,
        WriteStorage<'a, InflictsDamage>,
        WriteStorage<'a, SingleActivation>,
        WriteStorage<'a, Trap>,
        WriteExpect<'a, TrapSpawner>,
        WriteExpect<'a, SimpleMarkerAllocator<Saveable>>,
        WriteStorage<'a, SimpleMarker<Saveable>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut levels,
            mut renderables,
            mut names,
            mut hiddens,
            mut entry_triggers,
            mut inflicts_damage,
            mut single_activations,
            mut trap,
            mut spawner,
            mut marker_allocator,
            mut markers,
        ) = data;
        for request in spawner.requests.iter() {
            let new_trap = entities.create();
            positions
                .insert(
                    new_trap,
                    Position {
                        x: request.x,
                        y: request.y,
                    },
                )
                .expect("failed inserting position for new trap");
            renderables
                .insert(
                    new_trap,
                    Renderable {
                        fg: RGB::from(RED),
                        bg: RGB::from(BLACK),
                        glyph: trap_type::get_glyph_for_trap(&request.trap_type),
                        layer: 2,
                    },
                )
                .expect("failed inserting renderable for new trap");
            levels
                .insert(
                    new_trap,
                    DungeonLevel {
                        level: request.level,
                    },
                )
                .expect("failed inserting level for new trap");
            entry_triggers
                .insert(new_trap, EntryTrigger {})
                .expect("failed inserting entry trigger for new trap");
            let mut hidden = Hidden {
                found_by: EntitySet::new(),
            };
            hidden.found_by.insert(request.set_by);
            hiddens
                .insert(new_trap, hidden)
                .expect("failed inserting hidden for new trap");

            inflicts_damage
                .insert(
                    new_trap,
                    InflictsDamage {
                        amount: trap_type::get_damage_for_trap(&request.trap_type),
                    },
                )
                .expect("failed inserting inflicts damage for new trap");
            names
                .insert(
                    new_trap,
                    Name {
                        name: trap_type::get_name_for_trap(&request.trap_type),
                    },
                )
                .expect("failed inserting name for new trap");
            trap.insert(
                new_trap,
                Trap {
                    trap_type: request.trap_type,
                    armed: true,
                },
            )
            .expect("failed inserting trap for new trap");
            if trap_type::is_trap_single_activation(&request.trap_type) {
                single_activations
                    .insert(new_trap, SingleActivation {})
                    .expect("failed inserting single activation for new trap");
            }
            marker_allocator.mark(new_trap, &mut markers);
            // need to mark entity
        }
        spawner.requests.clear();
    }
}
