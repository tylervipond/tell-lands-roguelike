// This file contains all code related to saving and loading JSON into the ECS.
// For the most part this code is taken from the tutorial at
// http://bfnightly.bracketproductions.com/rustbook/chapter_11.html
// It might be good in the future to look into making a custom impl for SerializeComponents
// to replace the custom macros
use crate::components::{
    AreaOfEffect, BlocksTile, Blood, CausesFire, CombatStats, Confusion, Consumable, Contained,
    Container, DungeonLevel, EntityMoved, EntryTrigger, Flammable, Furniture, Grabbable, Grabbing,
    Hidden, Hiding, HidingSpot, InBackpack, InflictsDamage, Item, Memory, Monster, Name, Objective,
    OnFire, ParticleLifetime, Player, Position, ProvidesHealing, Ranged, Renderable, Saveable,
    SerializationHelper, SingleActivation, SufferDamage, Trap, Triggered, Viewshed,
    WantsToDisarmTrap, WantsToDropItem, WantsToGrab, WantsToHide, WantsToMelee, WantsToMove,
    WantsToOpenDoor, WantsToPickUpItem, WantsToReleaseGrabbed, WantsToSearchHidden, WantsToTrap,
    WantsToUse,
};
use crate::dungeon::{constants::MAP_COUNT, dungeon::Dungeon};
use specs::{
    error::NoError,
    join::Join,
    saveload::{
        DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker,
        SimpleMarkerAllocator,
    },
    world::Builder,
    Entity, World, WorldExt,
};
#[cfg(not(target_arch = "wasm32"))]
use std::fs::{read_to_string, remove_file, File};
use std::io::Write;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::str;
#[cfg(target_arch = "wasm32")]
use web_sys::Storage;

#[cfg(not(target_arch = "wasm32"))]
const SAVE_FILE_PATH: &str = "./tell-lands-save.json";

#[cfg(target_arch = "wasm32")]
const SAVE_FILE_PATH: &str = "tell-lands-save";

macro_rules! serialize_individually {
  ($world:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
      $(
      SerializeComponents::<NoError, SimpleMarker<Saveable>>::serialize(
          &( $world.read_storage::<$type>(), ),
          &$data.0,
          &$data.1,
          &mut $ser,
      )
      .unwrap();
      )*
  };
}

macro_rules! deserialize_individually {
  ($world:expr, $de:expr, $data:expr, $( $type:ty),*) => {
      $(
      DeserializeComponents::<NoError, _>::deserialize(
          &mut ( &mut $world.write_storage::<$type>(), ),
          &mut $data.0,
          &mut $data.1,
          &mut $data.2,
          &mut $de,
      )
      .unwrap();
      )*
  };
}

fn create_save_game_helpers(world: &mut World) {
    let dungeon_copy = world.get_mut::<Dungeon>().unwrap().clone();
    world
        .create_entity()
        .with(SerializationHelper {
            dungeon: dungeon_copy,
        })
        .marked::<SimpleMarker<Saveable>>()
        .build();
}

fn delete_helpers(world: &mut World) {
    let helper_ents: Vec<Entity> = {
        let helpers = world.read_storage::<SerializationHelper>();
        let entities = world.entities();
        (&entities, &helpers).join().map(|(e, _h)| e).collect()
    };
    world
        .delete_entities(helper_ents.as_slice())
        .expect("Delete Helpers Failed");
}

fn save_game_with_writer<T: Write>(world: &mut World, writer: T) -> serde_json::Serializer<T> {
    create_save_game_helpers(world);

    let mut serializer = serde_json::Serializer::new(writer);
    {
        let ent_markers = (
            world.entities(),
            world.read_storage::<SimpleMarker<Saveable>>(),
        );
        serialize_individually!(
            world,
            serializer,
            ent_markers,
            Position,
            Renderable,
            Player,
            Viewshed,
            Monster,
            Name,
            BlocksTile,
            CombatStats,
            SufferDamage,
            WantsToMelee,
            Item,
            Consumable,
            Ranged,
            InflictsDamage,
            AreaOfEffect,
            Confusion,
            ProvidesHealing,
            InBackpack,
            WantsToPickUpItem,
            WantsToUse,
            WantsToDropItem,
            DungeonLevel,
            Blood,
            ParticleLifetime,
            Hidden,
            EntryTrigger,
            EntityMoved,
            SingleActivation,
            Triggered,
            Objective,
            Container,
            Contained,
            Flammable,
            OnFire,
            CausesFire,
            WantsToSearchHidden,
            Trap,
            WantsToTrap,
            WantsToDisarmTrap,
            WantsToGrab,
            Grabbable,
            Grabbing,
            WantsToMove,
            WantsToReleaseGrabbed,
            WantsToOpenDoor,
            Furniture,
            HidingSpot,
            Hiding,
            WantsToHide,
            Memory,
            SerializationHelper
        );
    }
    delete_helpers(world);
    serializer
}

fn deserialize_from_string(world: &mut World, game_string: String) {
    let mut deserializer = serde_json::Deserializer::from_str(&game_string);
    let mut ent_markers = (
        &mut world.entities(),
        &mut world.write_storage::<SimpleMarker<Saveable>>(),
        &mut world.write_resource::<SimpleMarkerAllocator<Saveable>>(),
    );

    deserialize_individually!(
        world,
        deserializer,
        ent_markers,
        Position,
        Renderable,
        Player,
        Viewshed,
        Monster,
        Name,
        BlocksTile,
        CombatStats,
        SufferDamage,
        WantsToMelee,
        Item,
        Consumable,
        Ranged,
        InflictsDamage,
        AreaOfEffect,
        Confusion,
        ProvidesHealing,
        InBackpack,
        WantsToPickUpItem,
        WantsToUse,
        WantsToDropItem,
        DungeonLevel,
        Blood,
        ParticleLifetime,
        Hidden,
        EntryTrigger,
        EntityMoved,
        SingleActivation,
        Triggered,
        Objective,
        Container,
        Contained,
        Flammable,
        OnFire,
        CausesFire,
        WantsToSearchHidden,
        Trap,
        WantsToTrap,
        WantsToDisarmTrap,
        WantsToGrab,
        Grabbable,
        Grabbing,
        WantsToMove,
        WantsToReleaseGrabbed,
        WantsToOpenDoor,
        Furniture,
        HidingSpot,
        Hiding,
        WantsToHide,
        Memory,
        SerializationHelper
    );
}

fn get_dungeon(world: &mut World) -> Dungeon {
    let serialization_helpers = world.read_storage::<SerializationHelper>();
    let mut dungeons: Vec<Dungeon> = (serialization_helpers)
        .join()
        .map(|h| {
            let mut cloned_dungeon = h.dungeon.clone();
            for (_i, mut level) in cloned_dungeon.levels.iter_mut() {
                level.tile_content = vec![Vec::new(); MAP_COUNT];
            }
            cloned_dungeon
        })
        .collect();
    dungeons.remove(0)
}

fn populate_map_from_helper(world: &mut World) {
    let dungeon = get_dungeon(world);
    world.insert(dungeon);
}

fn get_player_parts(world: &mut World) -> (i32, i32, Entity) {
    let entities = world.entities();
    let player = world.read_storage::<Player>();
    let position = world.read_storage::<Position>();
    let parts: Vec<(Entity, &Player, &Position)> = (&entities, &player, &position).join().collect();
    let player_part = parts.get(0).unwrap();
    (player_part.2.x, player_part.2.y, player_part.0)
}

fn populate_player(world: &mut World) {
    let player_parts = get_player_parts(world);
    world.insert(rltk::Point::new(player_parts.0, player_parts.1));
    world.insert(player_parts.2);
}

fn load_game_from_string(world: &mut World, game_string: String) {
    world.delete_all();
    deserialize_from_string(world, game_string);
    populate_map_from_helper(world);
    delete_helpers(world);
    populate_player(world);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_game(world: &mut World) {
    let game_string = read_to_string(SAVE_FILE_PATH).unwrap();
    load_game_from_string(world, game_string);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn has_save_game() -> bool {
    Path::new(SAVE_FILE_PATH).exists()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn delete_save() {
    if has_save_game() {
        remove_file(SAVE_FILE_PATH).expect("unable to delete save file")
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_game(world: &mut World) {
    let writer = File::create(SAVE_FILE_PATH).unwrap();
    save_game_with_writer(world, writer);
}

#[cfg(target_arch = "wasm32")]
fn get_local_storage() -> Storage {
    let window = web_sys::window().expect("no global `window` exists");
    window.local_storage().unwrap().expect("no local storage")
}

#[cfg(target_arch = "wasm32")]
pub fn save_game(world: &mut World) {
    let writer = Vec::<u8>::new();
    let serializer = save_game_with_writer(world, writer);
    let storage = get_local_storage();
    storage
        .set_item(
            SAVE_FILE_PATH,
            str::from_utf8(&serializer.into_inner()).unwrap(),
        )
        .expect("could not write to local storage");
}

#[cfg(target_arch = "wasm32")]
pub fn delete_save() {
    if has_save_game() {
        let storage = get_local_storage();
        storage
            .remove_item(SAVE_FILE_PATH)
            .expect("couldn't delete file");
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_game(world: &mut World) {
    let storage = get_local_storage();
    match storage.get_item(SAVE_FILE_PATH) {
        Ok(r) => match r {
            Some(game_string) => load_game_from_string(world, game_string),
            _ => (),
        },
        _ => (),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn has_save_game() -> bool {
    let storage = get_local_storage();
    match storage.get_item(SAVE_FILE_PATH) {
        Ok(r) => match r {
            Some(_) => true,
            _ => false,
        },
        _ => false,
    }
}
