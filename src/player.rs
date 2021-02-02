use std::collections::HashSet;

use crate::components::{
    equipable::EquipmentPositions, Item, Monster, Position, Trap, Viewshed, WantsToCloseDoor,
    WantsToDisarmTrap, WantsToDouse, WantsToEquip, WantsToGrab, WantsToHide, WantsToLight,
    WantsToMelee, WantsToMove, WantsToOpenDoor, WantsToPickUpItem, WantsToReleaseGrabbed,
    WantsToSearchHidden, WantsToTrap, WantsToUse,
};
use crate::dungeon::{dungeon::Dungeon, level::Level, level_utils};
use crate::entity_option::EntityOption;
use crate::services::game_log::GameLog;
use crate::user_actions::MapAction;
use specs::{Component, Entity, Join, World, WorldExt};

pub fn move_to_position(world: &mut World, idx: usize) {
    let player_entity = world.fetch::<Entity>();
    world
        .write_storage::<WantsToMove>()
        .insert(*player_entity, WantsToMove { idx })
        .expect("couldn't insert player move intent");
}

// It's not really "try move player" though, it's more like "player act"
// because the function does more than just move the player. Should probably
// break this up a bit.
fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let positions = world.read_storage::<Position>();
    let monsters = world.read_storage::<Monster>();
    let dungeon = world.fetch::<Dungeon>();
    let player_entity = world.fetch::<Entity>();
    let player_position = positions.get(*player_entity).unwrap();
    let level = dungeon.get_level(player_position.level).unwrap();
    let level_width = level.width as i32;
    let destination_index = level_utils::add_xy_to_idx(
        level_width as i32,
        delta_x as i32,
        delta_y as i32,
        player_position.idx as i32,
    );
    let target = level.tile_content[destination_index as usize]
        .iter()
        .filter(|e| monsters.get(**e).is_some())
        .next();
    match target {
        Some(target) => {
            world
                .write_storage::<WantsToMelee>()
                .insert(*player_entity, WantsToMelee { target: *target })
                .expect("Add target failed");
        }
        None => {
            world
                .write_storage::<WantsToMove>()
                .insert(
                    *player_entity,
                    WantsToMove {
                        idx: destination_index as usize,
                    },
                )
                .expect("couldn't insert player move intent");
        }
    };
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
            let mut items = HashSet::new();
            items.insert(item);
            let mut pickup = world.write_storage::<WantsToPickUpItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickUpItem {
                        container: None,
                        items,
                    },
                )
                .expect("Unable to insert want to pick up");
        }
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

fn insert_intent<T: Component>(
    world: &mut World,
    intent: T,
) -> Result<Option<T>, specs::error::Error> {
    let player_entity = world.fetch::<Entity>();
    let mut intents = world.write_storage::<T>();
    intents.insert(*player_entity, intent)
}

pub fn use_item(world: &mut World, item: Entity, target: Option<usize>) {
    let is_trap = {
        let traps = world.read_storage::<Trap>();
        traps.get(item).is_some()
    };
    match is_trap {
        true => {
            insert_intent(world, WantsToTrap { item, target })
                .expect("Unable To Insert Trap Intent");
        }
        false => {
            insert_intent(world, WantsToUse { item, target })
                .expect("Unable To Insert Use Item Intent");
        }
    }
}

pub fn open_door(world: &mut World, door: Entity) {
    insert_intent(world, WantsToOpenDoor { door })
        .expect("could not insert wants to open door for player");
}

pub fn close_door(world: &mut World, door: Entity) {
    insert_intent(world, WantsToCloseDoor { door })
        .expect("could not insert wants to close door for player");
}

pub fn search_hidden(world: &mut World) {
    insert_intent(world, WantsToSearchHidden {})
        .expect("could not insert wants to search hidden for player");
}

pub fn disarm_trap(world: &mut World, item: Entity) {
    insert_intent(world, WantsToDisarmTrap { trap: item })
        .expect("Unable to Insert Disarm Trap Intent");
}

pub fn arm_trap(world: &mut World, item: Entity) {
    let target = {
        let positions = world.read_storage::<Position>();
        match positions.get(item) {
            Some(pos) => Some(pos.idx),
            None => None,
        }
    };
    insert_intent(world, WantsToTrap { item, target }).expect("Unable to Insert Trap Intent");
}

pub fn grab_entity(world: &mut World, entity: Entity) {
    insert_intent(
        world,
        WantsToGrab {
            thing: EntityOption::new(Some(entity)),
        },
    )
    .expect("Unable to Insert Grab Intent");
}

pub fn release_entity(world: &mut World) {
    insert_intent(world, WantsToReleaseGrabbed {}).expect("Unable to Insert Release Intent");
}

pub fn attack_entity(world: &mut World, entity: Entity) {
    insert_intent(world, WantsToMelee { target: entity })
        .expect("Unable to Insert Wants To Melee Intent");
}

pub fn hide_in_container(world: &mut World, entity: Entity) {
    insert_intent(
        world,
        WantsToHide {
            hiding_spot: Some(entity),
        },
    )
    .expect("Unable to Insert Hide in Container Intent");
}

pub fn equip_item(world: &mut World, entity: Option<Entity>, position: EquipmentPositions) {
    insert_intent(
        world,
        WantsToEquip {
            equipment: entity,
            position,
        },
    )
    .expect("Unable to Insert Hide in Container Intent");
}

pub fn douse_item(world: &mut World, item: Entity) {
    insert_intent(world, WantsToDouse { item }).expect("Unable to Insert Douse Intent");
}

pub fn light_item(world: &mut World, item: Entity) {
    insert_intent(world, WantsToLight { item }).expect("Unable to Insert Douse Intent");
}

pub fn pickup_items(world: &mut World, items: HashSet<Entity>, container: Option<Entity>) {
    insert_intent(world, WantsToPickUpItem { container, items })
        .expect("Unable to insert want to pick up");
}

pub fn pickup_item(world: &mut World, item: Entity, container: Option<Entity>) {
    let mut items = HashSet::new();
    items.insert(item);
    pickup_items(world, items, container);
}

#[derive(Copy, Clone, PartialEq)]
pub enum InteractionType {
    Douse,
    Light,
    HideIn,
    Attack,
    Grab,
    // Release,
    Disarm,
    Arm,
    // Use,
    // GoUp,
    // GoDown,
    // Exit,
    Pickup,
    OpenDoor,
    CloseDoor,
    OpenContainer,
}

pub fn interact(world: &mut World, object: Entity, interaction_type: InteractionType) {
    match interaction_type {
        InteractionType::Douse => douse_item(world, object),
        InteractionType::Light => light_item(world, object),
        InteractionType::HideIn => hide_in_container(world, object),
        InteractionType::Grab => grab_entity(world, object),
        InteractionType::Disarm => disarm_trap(world, object),
        InteractionType::Arm => arm_trap(world, object),
        InteractionType::Attack => attack_entity(world, object),
        InteractionType::Pickup => pickup_item(world, object, None),
        InteractionType::OpenDoor => open_door(world, object),
        InteractionType::CloseDoor => close_door(world, object),
        _ => {}
    }
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
        MapAction::PickupItem => try_pickup_item(world),
        MapAction::GoDownStairs => try_go_down_stairs(world),
        MapAction::GoUpStairs => try_go_up_stairs(world),
        MapAction::SearchHidden => search_hidden(world),
        MapAction::ReleaseFurniture => release_entity(world),
        _ => {}
    }
}
