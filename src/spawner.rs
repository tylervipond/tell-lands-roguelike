use crate::components::{
    AreaOfEffect, BlocksTile, CausesFire, CombatStats, Confusion, Consumable, Contained, Container,
    DungeonLevel, EntryTrigger, Flammable, Grabbable, Hidden, InflictsDamage, Item, Monster, Name,
    Objective, Player, Position, ProvidesHealing, Ranged, Renderable, Saveable, SingleActivation,
    Trap, Viewshed,
};
use crate::dungeon::{
    level::Level,
    level_utils,
    rect::Rect,
    room::Room,
    room_decorators::{RoomPart, RoomType},
    tile_type::TileType,
};
use crate::entity_set::EntitySet;
use crate::types::{trap_type, TrapType};
use crate::utils;
use rltk::{to_cp437, RandomNumberGenerator, RGB};
use specs::{
    saveload::{MarkedBuilder, SimpleMarker},
    Builder, Entity, EntityBuilder, Join, World, WorldExt,
};
use stamp_rs::StampPart::Use;
use std::cmp;

pub const MAX_ITEMS_PER_ROOM: i32 = 4;
pub const MAX_TRAPS_SET_PER_LEVEL: i32 = 10;
pub const MIN_GOBLIN_GROUPS_PER_LEVEL: i32 = 2;
pub const MAX_GOBLIN_GROUPS_PER_LEVEL: i32 = 4;
pub const MIN_GOBLINS_PER_GROUP: i32 = 3;
pub const MAX_GOBLINS_PER_GROUP: i32 = 6;
pub const MAX_GOBLIN_SPACING: i32 = 4;

fn get_possible_spawn_points_in_level(level: &Level) -> Vec<usize> {
    level
        .tiles
        .iter()
        .enumerate()
        .filter(|(idx, tile)| {
            **tile == TileType::Floor && !level_utils::tile_is_blocked(*idx as i32, level)
        })
        .map(|(idx, _)| idx)
        .collect()
}

fn get_random_from_world(world: &mut World, min: i32, max: i32) -> i32 {
    let mut rng = world.write_resource::<RandomNumberGenerator>();
    rng.range(min, max)
}

fn get_random_spawn_points_for_level(
    world: &mut World,
    level: &Level,
    min: i32,
    max: i32,
) -> Vec<usize> {
    let amount = get_random_from_world(world, min, max);
    let mut possible_spawn_points = get_possible_spawn_points_in_level(level);
    (0..std::cmp::min(amount, possible_spawn_points.len() as i32))
        .map(|_| {
            let idx = get_random_from_world(world, 0, possible_spawn_points.len() as i32);
            possible_spawn_points.remove(idx as usize)
        })
        .collect()
}

fn create_marked_entity_with_position<'a>(
    world: &'a mut World,
    map_idx: i32,
    level: &'a Level,
) -> EntityBuilder<'a> {
    let (x, y) = level_utils::idx_xy(level, map_idx);
    world
        .create_entity()
        .with(Position { x, y })
        .with(DungeonLevel { level: level.depth })
        .marked::<SimpleMarker<Saveable>>()
}

fn create_marked_entity_in_container<'a>(
    world: &'a mut World,
    container_entity: Entity,
) -> EntityBuilder<'a> {
    world
        .create_entity()
        .with(Contained {
            container: container_entity,
        })
        .marked::<SimpleMarker<Saveable>>()
}

pub fn spawn_player(world: &mut World, x: i32, y: i32, level: u8) -> Entity {
    world
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            layer: 0,
        })
        .with(DungeonLevel { level })
        .with(Player {})
        .with(Viewshed {
            range: 8,
            visible_tiles: vec![],
            dirty: true,
        })
        .with(Name {
            name: "Player".to_owned(),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            power: 5,
            defense: 2,
        })
        .marked::<SimpleMarker<Saveable>>()
        .build()
}

pub fn spawn_monster<S: ToString>(
    world: &mut World,
    idx: i32,
    glyph: u16,
    name: S,
    level: &Level,
) -> Entity {
    create_marked_entity_with_position(world, idx, level)
        .with(Renderable {
            glyph,
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            layer: 0,
        })
        .with(Viewshed {
            visible_tiles: vec![],
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .build()
}

fn spawn_objective(world: &mut World, idx: i32, level: &Level) -> Entity {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "The Talisman".to_string(),
        })
        .with(Renderable {
            glyph: 241,
            fg: RGB::named(rltk::LIGHT_SALMON),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Item {})
        .with(Objective {})
        .build()
}

pub fn spawn_goblin(world: &mut World, idx: i32, level: &Level) -> Entity {
    spawn_monster(world, idx, to_cp437('g'), "Goblin", level)
}

fn make_entity_health_potion<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
    builder
        .with(Name {
            name: "Health Potion".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('i'),
            fg: RGB::named(rltk::LIGHT_BLUE),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Item {})
        .with(Consumable {})
        .with(ProvidesHealing { amount: 8 })
}

pub fn spawn_health_potion(world: &mut World, idx: i32, level: &Level) -> Entity {
    make_entity_health_potion(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_health_potion_in_container(world: &mut World, container_entity: Entity) -> Entity {
    make_entity_health_potion(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_magic_missile_scroll<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
    builder
        .with(Name {
            name: "Scroll of Magic Missile".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437(')'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { amount: 8 })
}

fn spawn_magic_missile_scroll(world: &mut World, idx: i32, level: &Level) -> Entity {
    make_entity_magic_missile_scroll(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_magic_missile_scroll_in_container(world: &mut World, container_entity: Entity) -> Entity {
    make_entity_magic_missile_scroll(create_marked_entity_in_container(world, container_entity))
        .build()
}

fn make_entity_fireball_scroll<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
    builder
        .with(Name {
            name: "Scroll of Fireball".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437(')'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { amount: 20 })
        .with(CausesFire {})
        .with(AreaOfEffect { radius: 3 })
}

fn spawn_fireball_scroll(world: &mut World, idx: i32, level: &Level) -> Entity {
    make_entity_fireball_scroll(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_fireball_scroll_in_container(world: &mut World, container_entity: Entity) -> Entity {
    make_entity_fireball_scroll(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_confusion_scroll<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
    builder
        .with(Name {
            name: "Scroll of Confusion".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437(')'),
            fg: RGB::named(rltk::PINK),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 6 })
        .with(Confusion { turns: 4 })
}

fn spawn_confusion_scroll(world: &mut World, idx: i32, level: &Level) -> Entity {
    make_entity_confusion_scroll(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_confusion_scroll_in_container(world: &mut World, container_entity: Entity) -> Entity {
    make_entity_confusion_scroll(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_bear_trap<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
    builder
        .with(Name {
            name: "Bear Trap".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('^'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 1 })
        .with(Trap {
            trap_type: TrapType::BearTrap,
            armed: false,
        })
}

fn spawn_bear_trap(world: &mut World, idx: i32, level: &Level) -> Entity {
    make_entity_bear_trap(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_bear_trap_in_container(world: &mut World, container_entity: Entity) -> Entity {
    make_entity_bear_trap(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_caltrops<'a>(builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
    builder
        .with(Name {
            name: "Caltrops".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('%'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Item {})
        .with(Consumable {})
        .with(Ranged { range: 3 })
        .with(Trap {
            trap_type: TrapType::Caltrops,
            armed: false,
        })
}

fn spawn_caltrops(world: &mut World, idx: i32, level: &Level) -> Entity {
    make_entity_caltrops(create_marked_entity_with_position(world, idx, level)).build()
}

fn spawn_caltrops_in_container(world: &mut World, container_entity: Entity) -> Entity {
    make_entity_caltrops(create_marked_entity_in_container(world, container_entity)).build()
}

fn make_entity_set_trap<'a>(
    builder: EntityBuilder<'a>,
    type_of_trap: &TrapType,
) -> EntityBuilder<'a> {
    builder
        .with(Name {
            name: trap_type::get_name_for_trap(type_of_trap),
        })
        .with(Renderable {
            glyph: trap_type::get_glyph_for_trap(type_of_trap),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            layer: 2,
        })
        .with(Hidden {
            found_by: EntitySet::new(),
        })
        .with(EntryTrigger {})
        .with(InflictsDamage {
            amount: trap_type::get_damage_for_trap(type_of_trap),
        })
        .with(Trap {
            trap_type: type_of_trap.to_owned(),
            armed: true,
        })
}

fn spawn_set_bear_trap(world: &mut World, idx: i32, level: &Level) -> Entity {
    make_entity_set_trap(
        create_marked_entity_with_position(world, idx, level),
        &TrapType::BearTrap,
    )
    .with(SingleActivation {})
    .build()
}

fn spawn_set_caltrops(world: &mut World, idx: i32, level: &Level) -> Entity {
    make_entity_set_trap(
        create_marked_entity_with_position(world, idx, level),
        &TrapType::Caltrops,
    )
    .build()
}

fn spawn_set_traps(world: &mut World, idx: i32, level: &Level) {
    let roll = get_random_from_world(world, 0, 2);
    match roll {
        1 => spawn_set_bear_trap(world, idx, level),
        _ => spawn_set_caltrops(world, idx, level),
    };
}

fn spawn_random_item(world: &mut World, idx: i32, level: &Level) {
    let roll = get_random_from_world(world, 0, 7);
    match roll {
        1 | 2 => spawn_health_potion(world, idx, level),
        3 => spawn_fireball_scroll(world, idx, level),
        4 => spawn_confusion_scroll(world, idx, level),
        5 => spawn_bear_trap(world, idx, level),
        6 => spawn_caltrops(world, idx, level),
        _ => spawn_magic_missile_scroll(world, idx, level),
    };
}

fn spawn_random_item_in_container(world: &mut World, container_entity: Entity) {
    let roll = get_random_from_world(world, 0, 7);
    match roll {
        1 | 2 => spawn_health_potion_in_container(world, container_entity),
        3 => spawn_fireball_scroll_in_container(world, container_entity),
        4 => spawn_confusion_scroll_in_container(world, container_entity),
        5 => spawn_bear_trap_in_container(world, container_entity),
        6 => spawn_caltrops_in_container(world, container_entity),
        _ => spawn_magic_missile_scroll_in_container(world, container_entity),
    };
}

fn get_containers_in_room(world: &World, room: &Room) -> Vec<Entity> {
    let containers = world.read_storage::<Container>();
    let positions = world.read_storage::<Position>();
    let entities = world.entities();
    (&containers, &positions, &entities)
        .join()
        .filter(|(_c, p, _e)| room.rect.contains(p.x, p.y))
        .map(|(_c, _p, e)| e)
        .collect()
}

pub fn spawn_item_entities_for_room(world: &mut World, room: &Room, level: &Level) {
    let containers_in_room = get_containers_in_room(world, room);
    let min_items = match room.room_type {
        Some(RoomType::TreasureRoom) => 2,
        _ => 0,
    };
    let num_items = get_random_from_world(world, min_items, MAX_ITEMS_PER_ROOM + 2) - 2;
    if num_items > 0 {
        // more than half of the items should be in containers if there are any.
        let min_items_in_containers = num_items as f32 * 0.6;
        let num_items_in_containers = cmp::min(
            get_random_from_world(world, min_items_in_containers.ceil() as i32, num_items + 1),
            containers_in_room.len() as i32,
        );
        let num_items_not_in_containers = num_items - num_items_in_containers;
        let spawn_points = {
            let mut rng = world.write_resource::<RandomNumberGenerator>();
            level_utils::get_spawn_points(&room.rect, level, &mut rng, num_items_not_in_containers)
        };
        for idx in spawn_points.iter() {
            spawn_random_item(world, (*idx) as i32, level);
        }
        for _ in 0..num_items_in_containers {
            let container_idx = get_random_from_world(world, 0, containers_in_room.len() as i32);
            spawn_random_item_in_container(world, containers_in_room[container_idx as usize]);
        }
    }
}

pub fn spawn_bed(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Bed".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('b'),
            fg: RGB::named(rltk::LIGHT_BLUE),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_bedside_table(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Bedside Table".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('t'),
            fg: RGB::named(rltk::BROWN4),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_chair(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Chair".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('c'),
            fg: RGB::named(rltk::BROWN4),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_desk(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Desk".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('d'),
            fg: RGB::named(rltk::BROWN4),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_armoire(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Armoire".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('a'),
            fg: RGB::named(rltk::BROWN4),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_towel_rack(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Towel Rack".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('t'),
            fg: RGB::named(rltk::LIGHT_YELLOW),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 3 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_dresser(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Dresser".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('d'),
            fg: RGB::named(rltk::BROWN3),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_shelf(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Shelf".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('s'),
            fg: RGB::named(rltk::BROWN3),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_table(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Table".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('t'),
            fg: RGB::named(rltk::BROWN3),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}
pub fn spawn_counter(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Counter".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('C'),
            fg: RGB::named(rltk::BROWN3),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}
pub fn spawn_stove(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Stove".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('S'),
            fg: RGB::named(rltk::BROWN3),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}
pub fn spawn_cupboard(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Cupboard".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('c'),
            fg: RGB::named(rltk::BROWN3),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}
pub fn spawn_weapon_rack(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Weapon Rack".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('W'),
            fg: RGB::named(rltk::LIGHT_GREY),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_barrel(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Barrel".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('B'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_treasure_chest(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Trasure Chest".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('T'),
            fg: RGB::named(rltk::BROWN3),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(Container {})
        .with(Flammable { turns_remaining: 8 })
        .with(BlocksTile {})
        .with(Grabbable {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_debris(world: &mut World, idx: i32, level: &mut Level) {
    create_marked_entity_with_position(world, idx, level)
        .with(Name {
            name: "Debris".to_string(),
        })
        .with(Renderable {
            glyph: to_cp437('x'),
            fg: RGB::named(rltk::GREY),
            bg: RGB::named(rltk::BLACK),
            layer: 1,
        })
        .with(BlocksTile {})
        .build();
    level.blocked[idx as usize] = true;
}

pub fn spawn_entities_for_room(world: &mut World, room: &Room, level: &mut Level) {
    spawn_item_entities_for_room(world, room, level);
}

pub fn spawn_entites_from_room_stamp(world: &mut World, room: &Room, level: &mut Level) {
    let room_x = room.rect.x1;
    let room_y = room.rect.y1;
    for (y, row) in room.stamp.pattern.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let this_x = room_x + x as i32;
            let this_y = room_y + y as i32;
            let idx = level_utils::xy_idx(level, this_x, this_y);
            match tile {
                Use(RoomPart::Bed) => spawn_bed(world, idx, level),
                Use(RoomPart::BedsideTable) => spawn_bedside_table(world, idx, level),
                Use(RoomPart::Chair) => spawn_chair(world, idx, level),
                Use(RoomPart::Desk) => spawn_desk(world, idx, level),
                Use(RoomPart::Dresser) => spawn_dresser(world, idx, level),
                Use(RoomPart::Armoire) => spawn_armoire(world, idx, level),
                Use(RoomPart::Shelf) => spawn_shelf(world, idx, level),
                Use(RoomPart::Table) => spawn_table(world, idx, level),
                Use(RoomPart::Chest) => spawn_treasure_chest(world, idx, level),
                Use(RoomPart::Barrel) => spawn_barrel(world, idx, level),
                Use(RoomPart::Stove) => spawn_stove(world, idx, level),
                Use(RoomPart::Counter) => spawn_counter(world, idx, level),
                Use(RoomPart::Cupboard) => spawn_cupboard(world, idx, level),
                Use(RoomPart::WeaponRack) => spawn_weapon_rack(world, idx, level),
                Use(RoomPart::Debris) => spawn_debris(world, idx, level),
                Use(RoomPart::TowelRack) => spawn_towel_rack(world, idx, level),
                _ => (),
            };
        }
    }
}

fn spawn_set_traps_for_level(world: &mut World, level: &mut Level) {
    get_random_spawn_points_for_level(world, level, 4, MAX_TRAPS_SET_PER_LEVEL)
        .iter()
        .for_each(|idx| spawn_set_traps(world, *idx as i32, level));
}

fn spawn_goblins_for_level(world: &mut World, level: &mut Level) {
    get_random_spawn_points_for_level(
        world,
        level,
        MIN_GOBLIN_GROUPS_PER_LEVEL,
        MAX_GOBLIN_GROUPS_PER_LEVEL,
    )
    .iter()
    .for_each(|idx| {
        let mut possible_spawn_points_for_group =
            level_utils::get_all_spawnable_tiles_in_radius(level, *idx as i32, MAX_GOBLIN_SPACING);
        let goblin_count =
            get_random_from_world(world, MIN_GOBLINS_PER_GROUP, MAX_GOBLINS_PER_GROUP);
        let spawn_points = {
            let mut rng = world.write_resource::<RandomNumberGenerator>();
            utils::get_x_random_elements(
                &mut rng,
                goblin_count as u32,
                &mut possible_spawn_points_for_group,
            )
        };
        spawn_points.iter().for_each(|idx| {
            spawn_goblin(world, *idx, level);
        });
    });
}

pub fn spawn_entities_for_level(world: &mut World, level: &mut Level) {
    let count = level.rooms.len();
    for i in (0..count).skip(1) {
        let room = level.rooms[i].clone();
        spawn_entites_from_room_stamp(world, &room, level);
        spawn_entities_for_room(world, &room, level);
    }
    spawn_goblins_for_level(world, level);
    spawn_set_traps_for_level(world, level);
}

pub fn spawn_objective_for_room(ecs: &mut World, rect: &Rect, level: &Level) {
    let idx = {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        level_utils::get_random_spawn_point(rect, level, &mut rng)
    };
    spawn_objective(ecs, idx as i32, level);
}
