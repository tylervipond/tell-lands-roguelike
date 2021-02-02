use crate::{components::Monster, dungeon::dungeon::Dungeon, utils};
use specs::{Entity, Join, World, WorldExt};

#[cfg(debug_assertions)]
pub fn kill_all_monsters(world: &mut World) {
    let monster_ents: Vec<Entity> = {
        let entities = world.entities();
        let monsters = world.read_storage::<Monster>();
        (&entities, &monsters).join().map(|(e, _)| e).collect()
    };
    world
        .delete_entities(&monster_ents)
        .expect("couldn't delete ents");
}

#[cfg(debug_assertions)]
pub fn reveal_map(world: &mut World) {
    use super::dungeon::constants::MAP_COUNT;
    let player_level = utils::get_current_level_from_world(world);
    let mut dungeon = world.fetch_mut::<Dungeon>();
    let mut level = dungeon.get_level_mut(player_level).unwrap();
    level.revealed_tiles = Box::new([true; MAP_COUNT])
}
