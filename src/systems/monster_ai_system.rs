use crate::ai::{choose_action, reasoner, Action, WeightedAction};
use crate::components::{
    CombatStats, Confused, DungeonLevel, Furniture, Hiding, Memory, Monster, Position, Viewshed,
    WantsToMelee, WantsToMove, WantsToOpenDoor,
};
use crate::dungeon::{dungeon::Dungeon, level::Level, level_utils, tile_type::TileType};
use rltk::{a_star_search, DistanceAlg::Pythagoras, Point, RandomNumberGenerator};
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

fn get_move_action(
    level: &Level,
    move_idx: i32,
    step_count: i32,
    furniture_storage: &ReadStorage<Furniture>,
) -> WeightedAction {
    let (x, y) = level_utils::idx_xy(&level, move_idx);
    if level.tiles[move_idx as usize] == TileType::Door {
        return WeightedAction::new(
            Action::OpenDoor((x, y)),
            reasoner::move_weight(step_count, 2.0),
        );
    }
    if level.blocked[move_idx as usize] {
        for entity in level_utils::entities_at_xy(level, x, y) {
            if furniture_storage.get(entity).is_some() {
                return WeightedAction::new(
                    Action::Attack(entity),
                    reasoner::move_weight(step_count, 3.0),
                );
            }
        }
    }

    WeightedAction::new(
        Action::MoveTo((x, y)),
        reasoner::move_weight(step_count, 2.0),
    )
}

fn get_next_step(level: &Level, start_idx: i32, end_idx: i32) -> Option<(usize, usize)> {
    let path = a_star_search(start_idx, end_idx, level);
    let step_count = path.steps.len();
    match path.success && step_count > 1 {
        true => Some((path.steps[1], step_count)),
        _ => None,
    }
}

fn get_move_action_from_path(
    level: &Level,
    start_idx: i32,
    end_idx: i32,
    furniture_storage: &ReadStorage<Furniture>,
) -> Option<WeightedAction> {
    match get_next_step(level, start_idx, end_idx) {
        Some((next_step, step_count)) => Some(get_move_action(
            level,
            next_step as i32,
            step_count as i32,
            furniture_storage,
        )),
        _ => None,
    }
}

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Dungeon>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, Confused>,
        ReadStorage<'a, DungeonLevel>,
        WriteStorage<'a, WantsToMove>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, Memory>,
        WriteExpect<'a, RandomNumberGenerator>,
        WriteStorage<'a, WantsToOpenDoor>,
        ReadStorage<'a, Furniture>,
        ReadStorage<'a, Hiding>,
    );
    // This is currently very limited. Monsters will only act if they can see a player, which means that they must
    // also be on the same level to act.
    fn run(&mut self, data: Self::SystemData) {
        let (
            mut dungeon,
            player_position,
            player_entity,
            entities,
            viewsheds,
            positions,
            monsters,
            mut wants_to_melee,
            mut confused,
            levels,
            mut wants_to_move,
            combat_stats,
            mut memory,
            mut rng,
            mut wants_to_open_door,
            furniture,
            hiding,
        ) = data;
        let player_level = levels.get(*player_entity).unwrap();
        let level = dungeon.get_level_mut(player_level.level).unwrap();
        let player_hp = combat_stats.get(*player_entity).unwrap().hp;
        let player_idx = level_utils::xy_idx(&level, player_position.x, player_position.y);

        for (_monsters, entity, viewshed, position, dungeon_level, memory) in (
            &monsters,
            &entities,
            &viewsheds,
            &positions,
            &levels,
            &mut memory,
        )
            .join()
        {
            if let Some(is_confused) = confused.get_mut(entity) {
                is_confused.turns -= 1;
                if is_confused.turns < 1 {
                    confused.remove(entity);
                }
                continue;
            }
            if dungeon_level.level != player_level.level {
                continue;
            }
            let mut weighted_actions = vec![];
            let current_idx = level_utils::xy_idx(&level, position.x, position.y);
            if hiding.get(*player_entity).is_none() {
                let distance =
                    Pythagoras.distance2d(Point::new(position.x, position.y), *player_position);
                if distance < 1.5 {
                    weighted_actions.push(WeightedAction::new(
                        Action::Attack(*player_entity),
                        reasoner::attack_weight(player_hp),
                    ));
                } else if viewshed.visible_tiles.contains(&*player_position) {
                    if let Some((next_step, step_count)) =
                        get_next_step(&level, current_idx, player_idx)
                    {
                        let (x, y) = level_utils::idx_xy(&level, next_step as i32);
                        weighted_actions.push(WeightedAction::new(
                            Action::Chase((x, y)),
                            reasoner::chase_weight(player_hp, step_count as i32),
                        ));
                    }
                }
            }
            for enemy_position in memory.last_known_enemy_positions.iter() {
                if enemy_position.level != dungeon_level.level as i32 {
                    continue;
                }
                let idx2 = level_utils::xy_idx(&level, enemy_position.x, enemy_position.y);
                if let Some(action) =
                    get_move_action_from_path(&level, current_idx, idx2, &furniture)
                {
                    weighted_actions.push(action);
                }
            }
            for memory_enemy_hiding_place in memory.known_enemy_hiding_spots.iter() {
                let hiding_place = memory_enemy_hiding_place.hiding_spot;
                if let Some(hiding_position) = positions.get(hiding_place) {
                    let hiding_point = Point::new(hiding_position.x, hiding_position.y);
                    let distance =
                        Pythagoras.distance2d(Point::new(position.x, position.y), hiding_point);
                    if distance < 1.5 {
                        let hiding_place_hp = combat_stats.get(hiding_place).unwrap().hp;
                        weighted_actions.push(WeightedAction::new(
                            Action::Attack(hiding_place),
                            reasoner::attack_weight(hiding_place_hp),
                        ));
                    } else if viewshed.visible_tiles.contains(&hiding_point) {
                        if let Some((next_step, step_count)) =
                            get_next_step(&level, current_idx, player_idx)
                        {
                            let (x, y) = level_utils::idx_xy(&level, next_step as i32);
                            let hiding_place_hp = combat_stats.get(hiding_place).unwrap().hp;
                            weighted_actions.push(WeightedAction::new(
                                Action::Chase((x, y)),
                                reasoner::chase_weight(hiding_place_hp, step_count as i32),
                            ));
                        }
                    }
                }
            }
            if let Some(destination) = memory.wander_destination {
                let idx2 = level_utils::xy_idx(&level, destination.x, destination.y);
                if let Some(action) =
                    get_move_action_from_path(&level, current_idx, idx2, &furniture)
                {
                    weighted_actions.push(action);
                }
            } else if let Some(destination) =
                level_utils::get_random_unblocked_floor_point(&level, &mut rng)
            {
                if let Some(action) =
                    get_move_action_from_path(&level, current_idx, destination as i32, &furniture)
                {
                    weighted_actions.push(action);
                }
            }

            // in the future this should be updated to include item use possibly
            match choose_action(weighted_actions) {
                Some(Action::Chase((x, y))) => {
                    wants_to_move
                        .insert(entity, WantsToMove { x, y })
                        .expect("couldn't insert move intent");
                }
                Some(Action::Attack(target)) => {
                    wants_to_melee
                        .insert(entity, WantsToMelee { target })
                        .expect("Unable to insert attack intent");
                }
                Some(Action::MoveTo((x, y))) => {
                    wants_to_move
                        .insert(entity, WantsToMove { x, y })
                        .expect("couldn't insert move intent");
                }
                Some(Action::OpenDoor((x, y))) => {
                    wants_to_open_door
                        .insert(
                            entity,
                            WantsToOpenDoor {
                                position: (x, y),
                                level: dungeon_level.level as usize,
                            },
                        )
                        .expect("couldn't insert move intent");
                }
                _ => {}
            };
        }
    }
}
