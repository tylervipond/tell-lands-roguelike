use crate::components::{ WantsToOpenDoor, DungeonLevel, Viewshed};
use crate::dungeon::{level_utils, dungeon::Dungeon, tile_type::TileType};
use specs::{ Join, ReadStorage, System, WriteExpect, WriteStorage};
use std::collections::HashSet;

pub struct OpenDoorSystem {}

impl<'a> System<'a> for OpenDoorSystem {
    type SystemData = (
        WriteExpect<'a, Dungeon>,
        WriteStorage<'a, WantsToOpenDoor>,
        ReadStorage<'a, DungeonLevel>,
        WriteStorage<'a, Viewshed>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut dungeon, mut wants_to_open_door, dungeon_levels, mut viewsheds) = data;
        let mut levels_with_door_open = HashSet::new();
        for desire in (&wants_to_open_door).join() {
            let mut level = dungeon.get_level_mut(desire.level as u8).unwrap();
            let open_door_index = level_utils::xy_idx(level.width as i32, desire.position.0, desire.position.1);

            if level.tiles[open_door_index as usize] == TileType::Door {
                level_utils::set_tile_to_floor(&mut level, open_door_index as usize);
                level.blocked[open_door_index as usize] = false;
                levels_with_door_open.insert(desire.level);
            }
        }
        if levels_with_door_open.len() > 0 {
            (&dungeon_levels, &mut viewsheds)
                .join()
                .filter(|(l, _)| levels_with_door_open.contains(&(l.level as usize)))
                .for_each(|(_, v)|{ v.dirty = true});
        }
        wants_to_open_door.clear();
    }
}
