use crate::components::{
    CombatStats, DungeonLevel, Item, Monster, Player, Position, Trap, Viewshed, WantsToDisarmTrap,
    WantsToGrab, WantsToMelee, WantsToMove, WantsToOpenDoor, WantsToPickUpItem,
    WantsToReleaseGrabbed, WantsToSearchHidden, WantsToTrap, WantsToUse,
};
use crate::dungeon::{dungeon::Dungeon, level::Level, level_utils, tile_type::TileType};
use crate::entity_option::EntityOption;
use crate::services::game_log::GameLog;
use crate::user_actions::MapAction;
use crate::utils;
use rltk::Point;
use specs::{Entity, Join, World, WorldExt};

// It's not really "try move player" though, it's more like "player act"
// because the function does more than just move the player. Should probably
// break this up a bit.
fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut players = world.write_storage::<Player>();
    let mut wants_to_melee = world.write_storage::<WantsToMelee>();
    let monsters = world.read_storage::<Monster>();
    let entities = world.entities();
    let dungeon = world.fetch::<Dungeon>();
    let player_entity = world.fetch::<Entity>();
    let dungeon_levels = world.read_storage::<DungeonLevel>();
    let player_level = dungeon_levels.get(*player_entity).unwrap();
    let map = dungeon.get_level(player_level.level).unwrap();
    for (entity, _player, pos) in (&entities, &mut players, &mut positions).join() {
        let x = pos.x + delta_x;
        let y = pos.y + delta_y;
        let destination_index = level_utils::xy_idx(&map, x, y);
        let target = map.tile_content[destination_index as usize]
            .iter()
            .filter(|e| monsters.get(**e).is_some())
            .next();
        match target {
            Some(target) => {
                wants_to_melee
                    .insert(entity, WantsToMelee { target: *target })
                    .expect("Add target failed");
            }
            None => {
                world
                    .write_storage::<WantsToMove>()
                    .insert(entity, WantsToMove { x, y })
                    .expect("couldn't insert player move intent");
            }
        };
    }
}

fn try_pickup_item(world: &mut World) {
    let player_pos = world.fetch::<Point>();
    let player_entity = world.fetch::<Entity>();
    let entities = world.entities();
    let items = world.read_storage::<Item>();
    let positions = world.read_storage::<Position>();
    let levels = world.read_storage::<DungeonLevel>();
    let mut gamelog = world.fetch_mut::<GameLog>();
    let player_level = levels.get(*player_entity).unwrap();

    let target_item: Option<Entity> = (&entities, &items, &positions, &levels).join().find_map(
        |(ent, _item, position, level)| {
            if position.x == player_pos.x
                && position.y == player_pos.y
                && level.level == player_level.level
            {
                return Some(ent);
            }
            return None;
        },
    );

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
    let player_level = utils::get_current_level_from_world(world);
    let map = dungeon.get_level(player_level).unwrap();
    let mut inserted = false;
    let pos = positions.get(*player_entity).unwrap();
    for x in (pos.x - 1)..=(pos.x + 1) {
        for y in (pos.y - 1)..=(pos.y + 1) {
            let open_door_index = level_utils::xy_idx(&map, x, y);

            if map.tiles[open_door_index as usize] == TileType::Door {
                wants_to_open_door
                    .insert(
                        *player_entity,
                        WantsToOpenDoor {
                            position: (x, y),
                            level: player_level as usize,
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

fn points_match(point_a: &Point, point_b: &Point) -> bool {
    point_a.x == point_b.x && point_a.y == point_b.y
}

fn can_go_up(current_level: &Level, player_point: &Point) -> bool {
    if let Some(stairs_up_coords) = current_level.stairs_up {
        return points_match(&stairs_up_coords, player_point);
    }
    return false;
}

fn can_go_down(current_level: &Level, player_point: &Point) -> bool {
    if let Some(stairs_down_coords) = current_level.stairs_down {
        return points_match(&stairs_down_coords, player_point);
    }
    return false;
}

fn try_go_down_stairs(world: &mut World) {
    let mut player_point = world.write_resource::<Point>();
    let mut levels = world.write_storage::<DungeonLevel>();
    let player_entity = world.fetch::<Entity>();
    let mut player_level = levels.get_mut(*player_entity).unwrap();
    let dungeon = world.fetch::<Dungeon>();
    let current_level = dungeon.get_level(player_level.level).unwrap();
    if !can_go_down(current_level, &player_point) {
        return;
    }
    let next_level_number = player_level.level - 1;
    let next_map = dungeon.get_level(next_level_number);
    if let Some(next_map) = next_map {
        let stairs_up_coords = next_map.stairs_up.unwrap();
        let mut positions = world.write_storage::<Position>();
        let mut player_position = positions.get_mut(*player_entity).unwrap();
        player_position.x = stairs_up_coords.x;
        player_position.y = stairs_up_coords.y;
        player_level.level = next_level_number;
        player_point.x = stairs_up_coords.x;
        player_point.y = stairs_up_coords.y;
        let mut viewsheds = world.write_storage::<Viewshed>();
        let mut player_viewshed = viewsheds.get_mut(*player_entity).unwrap();
        player_viewshed.dirty = true;
    }
}

fn try_go_up_stairs(world: &mut World) {
    let mut player_point = world.write_resource::<Point>();
    let mut levels = world.write_storage::<DungeonLevel>();
    let player_entity = world.fetch::<Entity>();
    let mut player_level = levels.get_mut(*player_entity).unwrap();
    let dungeon = world.fetch::<Dungeon>();
    let current_level = dungeon.get_level(player_level.level).unwrap();
    if !can_go_up(current_level, &player_point) {
        return;
    }
    let next_level_number = player_level.level + 1;
    let next_map = dungeon.get_level(next_level_number);
    if let Some(next_map) = next_map {
        let stairs_down_coords = next_map.stairs_down.unwrap();
        let mut positions = world.write_storage::<Position>();
        let mut player_position = positions.get_mut(*player_entity).unwrap();
        player_position.x = stairs_down_coords.x;
        player_position.y = stairs_down_coords.y;
        player_level.level = next_level_number;
        player_point.x = stairs_down_coords.x;
        player_point.y = stairs_down_coords.y;
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

pub fn use_item(world: &mut World, item: Entity, target: Option<Point>) {
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
