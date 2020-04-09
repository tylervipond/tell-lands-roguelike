// This file contains all code related to saving and loading JSON into the ECS.
// For the most part this code is taken from the tutorial at
// http://bfnightly.bracketproductions.com/rustbook/chapter_11.html
// It might be good in the future to look into making a custom impl for SerializeComponents
// to replace the custom macros
use crate::components::{
  area_of_effect::AreaOfEffect, blocks_tile::BlocksTile, blood::Blood, combat_stats::CombatStats,
  confusion::Confusion, consumable::Consumable, dungeon_level::DungeonLevel,
  entity_moved::EntityMoved, entry_trigger::EntryTrigger, hidden::Hidden, in_backpack::InBackpack,
  inflicts_damage::InflictsDamage, item::Item, monster::Monster, name::Name, objective::Objective,
  particle_lifetime::ParticleLifetime, player::Player, position::Position,
  provides_healing::ProvidesHealing, ranged::Ranged, renderable::Renderable, saveable::Saveable,
  serialization_helper::SerializationHelper, single_activation::SingleActivation,
  suffer_damage::SufferDamage, triggered::Triggered, viewshed::Viewshed,
  wants_to_drop_item::WantsToDropItem, wants_to_melee::WantsToMelee,
  wants_to_pick_up_item::WantsToPickUpItem, wants_to_use::WantsToUse,
};
use crate::dungeon::dungeon::Dungeon;
use crate::map::MAP_COUNT;
use specs::{
  error::NoError,
  join::Join,
  saveload::{
    DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker, SimpleMarkerAllocator,
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
  ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
      $(
      SerializeComponents::<NoError, SimpleMarker<Saveable>>::serialize(
          &( $ecs.read_storage::<$type>(), ),
          &$data.0,
          &$data.1,
          &mut $ser,
      )
      .unwrap();
      )*
  };
}

macro_rules! deserialize_individually {
  ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
      $(
      DeserializeComponents::<NoError, _>::deserialize(
          &mut ( &mut $ecs.write_storage::<$type>(), ),
          &mut $data.0,
          &mut $data.1,
          &mut $data.2,
          &mut $de,
      )
      .unwrap();
      )*
  };
}

fn create_save_game_helpers(ecs: &mut World) {
  let dungeon_copy = ecs.get_mut::<Dungeon>().unwrap().clone();
  ecs
    .create_entity()
    .with(SerializationHelper {
      dungeon: dungeon_copy,
    })
    .marked::<SimpleMarker<Saveable>>()
    .build();
}


fn delete_helpers(ecs: &mut World) {
  let helper_ents: Vec<Entity> = {
    let helpers = ecs.read_storage::<SerializationHelper>();
    let entities = ecs.entities();
    (&entities, &helpers).join().map(|(e, _h)| e).collect()
  };
  ecs
    .delete_entities(helper_ents.as_slice())
    .expect("Delete Helpers Failed");
}


fn save_game_with_writer<T: Write>(ecs: &mut World, writer: T) -> serde_json::Serializer<T> {
  create_save_game_helpers(ecs);

  let mut serializer = serde_json::Serializer::new(writer);
  {
    let ent_markers = (ecs.entities(), ecs.read_storage::<SimpleMarker<Saveable>>());
    serialize_individually!(
      ecs,
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
      SerializationHelper
    );
  }
  delete_helpers(ecs);
  serializer
}

fn deserialize_from_string(ecs: &mut World, game_string: String) {
  let mut deserializer = serde_json::Deserializer::from_str(&game_string);
  let mut ent_markers = (
    &mut ecs.entities(),
    &mut ecs.write_storage::<SimpleMarker<Saveable>>(),
    &mut ecs.write_resource::<SimpleMarkerAllocator<Saveable>>(),
  );

  deserialize_individually!(
    ecs,
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
    SerializationHelper
  );
}

fn get_dungeon(ecs: &mut World) -> Dungeon {
  let serialization_helpers = ecs.read_storage::<SerializationHelper>();
  let mut dungeons: Vec<Dungeon> = (serialization_helpers)
    .join()
    .map(|h| {
      let mut cloned_dungeon = h.dungeon.clone();
      for (_i, mut map) in cloned_dungeon.maps.iter_mut() {
        map.tile_content = vec![Vec::new(); MAP_COUNT];
      }
      cloned_dungeon
    })
    .collect();
  dungeons.remove(0)
}

fn populate_map_from_helper(ecs: &mut World) {
  let dungeon = get_dungeon(ecs);
  ecs.insert(dungeon);
}

fn get_player_parts(ecs: &mut World) -> (i32, i32, Entity) {
  let entities = ecs.entities();
  let player = ecs.read_storage::<Player>();
  let position = ecs.read_storage::<Position>();
  let parts: Vec<(Entity, &Player, &Position)> = (&entities, &player, &position).join().collect();
  let player_part = parts.get(0).unwrap();
  (player_part.2.x, player_part.2.y, player_part.0)
}

fn populate_player(ecs: &mut World) {
  let player_parts = get_player_parts(ecs);
  ecs.insert(rltk::Point::new(player_parts.0, player_parts.1));
  ecs.insert(player_parts.2);
}

fn load_game_from_string(ecs: &mut World, game_string: String) {
  ecs.delete_all();
  deserialize_from_string(ecs, game_string);
  populate_map_from_helper(ecs);
  delete_helpers(ecs);
  populate_player(ecs);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_game(ecs: &mut World) {
  let game_string = read_to_string(SAVE_FILE_PATH).unwrap();
  load_game_from_string(ecs, game_string);
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
pub fn save_game(ecs: &mut World) {
  let writer = File::create(SAVE_FILE_PATH).unwrap();
  save_game_with_writer(ecs, writer);
}

#[cfg(target_arch = "wasm32")]
fn get_local_storage() -> Storage {
  let window = web_sys::window().expect("no global `window` exists");
  window.local_storage().unwrap().expect("no local storage")
}

#[cfg(target_arch = "wasm32")]
pub fn save_game(ecs: &mut World) {
  let writer = Vec::<u8>::new();
  let serializer = save_game_with_writer(ecs, writer);
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
pub fn load_game(ecs: &mut World) {
  let storage = get_local_storage();
  match storage.get_item(SAVE_FILE_PATH) {
    Ok(r) => match r {
      Some(game_string) => load_game_from_string(ecs, game_string),
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
