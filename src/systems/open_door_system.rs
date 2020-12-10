use crate::components::{ WantsToOpenDoor, Position, Viewshed};
use crate::dungeon::{level_utils, dungeon::Dungeon, tile_type::TileType};
use specs::{ Join, ReadStorage, System, WriteExpect, WriteStorage};
use std::collections::HashSet;

pub struct OpenDoorSystem {}

impl<'a> System<'a> for OpenDoorSystem {
    type SystemData = (
        WriteExpect<'a, Dungeon>,
        WriteStorage<'a, WantsToOpenDoor>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Viewshed>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut dungeon, mut wants_to_open_door, position, mut viewsheds) = data;
        let mut levels_with_door_open = HashSet::new();
        for desire in (&wants_to_open_door).join() {
            let mut level = dungeon.get_level_mut(desire.level as u8).unwrap();
            if level.tiles[desire.idx] == TileType::Door {
                level_utils::set_tile_to_floor(&mut level, desire.idx);
                level.blocked[desire.idx] = false;
                levels_with_door_open.insert(desire.level);
            }
        }
        if levels_with_door_open.len() > 0 {
            (&position, &mut viewsheds)
                .join()
                .filter(|(p, _)| levels_with_door_open.contains(&(p.level as usize)))
                .for_each(|(_, v)|{ v.dirty = true});
        }
        wants_to_open_door.clear();
    }
}
