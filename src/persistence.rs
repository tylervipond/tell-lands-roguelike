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
use std::fs::{read_to_string, remove_file, File};
use std::path::Path;

const SAVE_FILE_PATH: &str = "./tell-lands-save.json";

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

#[cfg(not(target_arch = "wasm32"))]
pub fn save_game(ecs: &mut World) {
  let dungeon_copy = ecs.get_mut::<Dungeon>().unwrap().clone();
  let savehelper = ecs
    .create_entity()
    .with(SerializationHelper {
      dungeon: dungeon_copy,
    })
    .marked::<SimpleMarker<Saveable>>()
    .build();

  {
    let ent_markers = (ecs.entities(), ecs.read_storage::<SimpleMarker<Saveable>>());

    let writer = File::create(SAVE_FILE_PATH).unwrap();
    let mut serializer = serde_json::Serializer::new(writer);
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
  ecs.delete_entity(savehelper).expect("Crash on cleanup");
}

#[cfg(target_arch = "wasm32")]
pub fn save_game(_ecs: &mut World) {
  
}

fn deserialize_from_save_file(ecs: &mut World) {
  let data = read_to_string(SAVE_FILE_PATH).unwrap();
  let mut deserializer = serde_json::Deserializer::from_str(&data);
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

fn populate_map_from_helper(ecs: &mut World) {
  (ecs.read_storage::<SerializationHelper>())
    .join()
    .for_each(|h| {
      let mut dungeon = ecs.write_resource::<Dungeon>();
      let mut cloned_dungeon = h.dungeon.clone();
      for (_i, mut map) in cloned_dungeon.maps.iter_mut() {
        map.tile_content = vec![Vec::new(); MAP_COUNT];
      }
      *dungeon = cloned_dungeon;
    });
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

fn populate_player(ecs: &mut World) {
  let entities = ecs.entities();
  let player = ecs.read_storage::<Player>();
  let position = ecs.read_storage::<Position>();
  for (e, _p, pos) in (&entities, &player, &position).join() {
    let mut ppos = ecs.write_resource::<rltk::Point>();
    *ppos = rltk::Point::new(pos.x, pos.y);
    let mut player_resource = ecs.write_resource::<Entity>();
    *player_resource = e;
  }
}

pub fn load_game(ecs: &mut World) {
  ecs.delete_all();
  deserialize_from_save_file(ecs);
  populate_map_from_helper(ecs);
  delete_helpers(ecs);
  populate_player(ecs);
}

pub fn has_save_game() -> bool {
  Path::new(SAVE_FILE_PATH).exists()
}

pub fn delete_save() {
  if has_save_game() {
    remove_file(SAVE_FILE_PATH).expect("unable to delete save file")
  }
}
