use crate::components::{
    equipable::EquipmentPositions, Item, Monster, Player, Position, Trap, Viewshed,
    WantsToDisarmTrap, WantsToDouse, WantsToEquip, WantsToGrab, WantsToHide, WantsToLight,
    WantsToMelee, WantsToMove, WantsToOpenDoor, WantsToPickUpItem, WantsToReleaseGrabbed,
    WantsToSearchHidden, WantsToTrap, WantsToUse,
};
use crate::dungeon::{dungeon::Dungeon, level::Level, level_utils, tile_type::TileType};
use crate::entity_option::EntityOption;
use crate::services::game_log::GameLog;
use crate::user_actions::MapAction;
use specs::{Entity, Join, World, WorldExt};

// It's not really "try move player" though, it's more like "player act"
// because the function does more than just move the player. Should probably
// break this up a bit.
fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let positions = world.read_storage::<Position>();
    let players = world.read_storage::<Player>();
    let monsters = world.read_storage::<Monster>();
    let entities = world.entities();
    let dungeon = world.fetch::<Dungeon>();
    let player_entity = world.fetch::<Entity>();
    let player_position = positions.get(*player_entity).unwrap();
    let level = dungeon.get_level(player_position.level).unwrap();
    let level_width = level.width as i32;
    for (entity, _player, pos) in (&entities, &players, &positions).join() {
        let destination_index = level_utils::add_xy_to_idx(
            level_width as i32,
            delta_x as i32,
            delta_y as i32,
            pos.idx as i32,
        );
        let target = level.tile_content[destination_index as usize]
            .iter()
            .filter(|e| monsters.get(**e).is_some())
            .next();
        match target {
            Some(target) => {
                world
                    .write_storage::<WantsToMelee>()
                    .insert(entity, WantsToMelee { target: *target })
                    .expect("Add target failed");
            }
            None => {
                world
                    .write_storage::<WantsToMove>()
                    .insert(
                        entity,
                        WantsToMove {
                            idx: destination_index as usize,
                        },
                    )
                    .expect("couldn't insert player move intent");
            }
        };
    }
}

fn try_pickup_item(world: &mut World) {
    let player_entity = world.fetch::<Entity>();
    let entities = world.entities();
    let items = world.read_storage::<Item>();
    let positions = world.read_storage::<Position>();
    let player_pos = positions.get(*player_entity).unwrap();
    let mut gamelog = world.fetch_mut::<GameLog>();
    let target_item: Option<Entity> =
        (&entities, &items, &positions)
            .join()
            .find_map(|(ent, _item, position)| {
                if position.idx == player_pos.idx && position.level == player_pos.level {
                    return Some(ent);
                }
                return None;
            });

    match target_item {
        None => gamelog
            .entries
            .insert(0, "there is nothing here to pick up".to_string()),
        Some(item) => {
            let mut pickup = world.write_storage::<WantsToPickUpItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickUpItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pick up");
        }
    }
}

fn try_open_door(world: &mut World) {
    let positions = world.write_storage::<Position>();
    let mut wants_to_open_door = world.write_storage::<WantsToOpenDoor>();
    let dungeon = world.fetch::<Dungeon>();
    let player_entity = world.fetch::<Entity>();
    let pos = positions.get(*player_entity).unwrap();
    let level = dungeon.get_level(pos.level).unwrap();
    let level_width = level.width as i32;
    let mut inserted = false;
    for x in -1..=1 {
        for y in -1..=1 {
            let open_door_index =
                level_utils::add_xy_to_idx(level_width as i32, x, y, pos.idx as i32);
            if level.tiles[open_door_index as usize] == TileType::Door {
                wants_to_open_door
                    .insert(
                        *player_entity,
                        WantsToOpenDoor {
                            idx: open_door_index as usize,
                            level: pos.level as usize,
                        },
                    )
                    .expect("could not insert wants to open door for player");
                inserted = true;
            }
        }
    }
    if !inserted {
        let mut gamelog = world.fetch_mut::<GameLog>();
        gamelog
            .entries
            .insert(0, "there is no door nearby".to_string());
    }
}

fn can_go_up(current_level: &Level, player_idx: usize) -> bool {
    match current_level.stairs_up {
        Some(idx) => idx == player_idx,
        None => false,
    }
}

fn can_go_down(current_level: &Level, player_idx: usize) -> bool {
    match current_level.stairs_down {
        Some(idx) => idx == player_idx,
        None => false,
    }
}

// most of this should be moved into a go down stairs system
fn try_go_down_stairs(world: &mut World) {
    let player_entity = world.fetch::<Entity>();
    let mut positions = world.write_storage::<Position>();
    let mut player_position = positions.get_mut(*player_entity).unwrap();
    let dungeon = world.fetch::<Dungeon>();
    let current_level = dungeon.get_level(player_position.level).unwrap();
    if !can_go_down(current_level, player_position.idx) {
        return;
    }
    let next_level_number = player_position.level - 1;
    let next_map = dungeon.get_level(next_level_number);
    if let Some(next_map) = next_map {
        let stairs_up_coords = next_map.stairs_up.unwrap();
        player_position.idx = stairs_up_coords;
        player_position.level = next_level_number;
        let mut viewsheds = world.write_storage::<Viewshed>();
        let mut player_viewshed = viewsheds.get_mut(*player_entity).unwrap();
        player_viewshed.dirty = true;
    }
}

// most of this should be moved into a go up stairs system
fn try_go_up_stairs(world: &mut World) {
    let player_entity = world.fetch::<Entity>();
    let mut positions = world.write_storage::<Position>();
    let mut player_position = positions.get_mut(*player_entity).unwrap();
    let dungeon = world.fetch::<Dungeon>();
    let current_level = dungeon.get_level(player_position.level).unwrap();
    if !can_go_up(current_level, player_position.idx) {
        return;
    }
    let next_level_number = player_position.level + 1;
    let next_map = dungeon.get_level(next_level_number);
    if let Some(next_map) = next_map {
        let stairs_down_coords = next_map.stairs_down.unwrap();
        player_position.idx = stairs_down_coords;
        player_position.level = next_level_number;
        let mut viewsheds = world.write_storage::<Viewshed>();
        let mut player_viewshed = viewsheds.get_mut(*player_entity).unwrap();
        player_viewshed.dirty = true;
    }
}

fn search_hidden(world: &mut World) {
    let player_entity = world.fetch::<Entity>();
    let mut wants_to_search_hidden = world.write_storage::<WantsToSearchHidden>();
    wants_to_search_hidden
        .insert(*player_entity, WantsToSearchHidden {})
        .expect("could not insert wants to search hidden for player");
}

pub fn use_item(world: &mut World, item: Entity, target: Option<usize>) {
    let player_entity = world.fetch::<Entity>();
    let traps = world.read_storage::<Trap>();
    match traps.get(item) {
        Some(_) => {
            let mut wants_to_trap = world.write_storage::<WantsToTrap>();
            wants_to_trap
                .insert(*player_entity, WantsToTrap { item, target })
                .expect("Unable To Insert Trap Intent");
        }
        None => {
            let mut wants_to_use = world.write_storage::<WantsToUse>();
            wants_to_use
                .insert(*player_entity, WantsToUse { item, target })
                .expect("Unable To Insert Use Item Intent");
        }
    }
}

pub fn disarm_trap(world: &mut World, item: Entity) {
    let player_entity = world.fetch::<Entity>();
    let mut disarm_traps_intents = world.write_storage::<WantsToDisarmTrap>();
    disarm_traps_intents
        .insert(*player_entity, WantsToDisarmTrap { trap: item })
        .expect("Unable to Insert Disarm Trap Intent");
}

pub fn grab_entity(world: &mut World, entity: Entity) {
    let player_entity = world.fetch::<Entity>();
    let mut grab_entity_intent = world.write_storage::<WantsToGrab>();
    grab_entity_intent
        .insert(
            *player_entity,
            WantsToGrab {
                thing: EntityOption::new(Some(entity)),
            },
        )
        .expect("Unable to Insert Grab Intent");
}

pub fn release_entity(world: &mut World) {
    let player_entity = world.fetch::<Entity>();
    world
        .write_storage::<WantsToReleaseGrabbed>()
        .insert(*player_entity, WantsToReleaseGrabbed {})
        .expect("Unable to Insert Release Intent");
}

pub fn attack_entity(world: &mut World, entity: Entity) {
    let player_entity = world.fetch::<Entity>();
    let mut melee_intents = world.write_storage::<WantsToMelee>();
    melee_intents
        .insert(*player_entity, WantsToMelee { target: entity })
        .expect("Unable to Insert Wants To Melee Intent");
}

pub fn hide_in_container(world: &mut World, entity: Entity) {
    let player_entity = world.fetch::<Entity>();
    let mut hide_intent = world.write_storage::<WantsToHide>();
    hide_intent
        .insert(
            *player_entity,
            WantsToHide {
                hiding_spot: Some(entity),
            },
        )
        .expect("Unable to Insert Hide in Container Intent");
}

pub fn equip_item(world: &mut World, entity: Option<Entity>, position: EquipmentPositions) {
    let player_entity = world.fetch::<Entity>();
    let mut equip_intent = world.write_storage::<WantsToEquip>();
    equip_intent
        .insert(
            *player_entity,
            WantsToEquip {
                equipment: entity,
                position,
            },
        )
        .expect("Unable to Insert Hide in Container Intent");
}

pub fn douse_item(world: &mut World, item: Entity) {
    let player_entity = world.fetch::<Entity>();
    let mut douse_intents = world.write_storage::<WantsToDouse>();
    douse_intents
        .insert(*player_entity, WantsToDouse { item })
        .expect("Unable to Insert Douse Intent");
}

pub fn light_item(world: &mut World, item: Entity) {
    let player_entity = world.fetch::<Entity>();
    let mut light_intents = world.write_storage::<WantsToLight>();
    light_intents
        .insert(*player_entity, WantsToLight { item })
        .expect("Unable to Insert Douse Intent");
}

pub fn player_action(world: &mut World, action: MapAction) {
    match action {
        MapAction::MoveLeft => try_move_player(-1, 0, world),
        MapAction::MoveRight => try_move_player(1, 0, world),
        MapAction::MoveUp => try_move_player(0, -1, world),
        MapAction::MoveDown => try_move_player(0, 1, world),
        MapAction::MoveUpLeft => try_move_player(-1, -1, world),
        MapAction::MoveUpRight => try_move_player(1, -1, world),
        MapAction::MoveDownLeft => try_move_player(-1, 1, world),
        MapAction::MoveDownRight => try_move_player(1, 1, world),
        MapAction::OpenDoor => try_open_door(world),
        MapAction::PickupItem => try_pickup_item(world),
        MapAction::GoDownStairs => try_go_down_stairs(world),
        MapAction::GoUpStairs => try_go_up_stairs(world),
        MapAction::SearchHidden => search_hidden(world),
        MapAction::ReleaseFurniture => release_entity(world),
        _ => {}
    }
}
