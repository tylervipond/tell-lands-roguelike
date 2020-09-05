use super::{Action, WeightedAction};

pub fn choose_action(weighted_actions: Vec<WeightedAction>) -> Option<Action> {
    let max = weighted_actions
        .iter()
        .max();
    match max {
        Some(weighted_action) => Some(weighted_action.action),
        None => None,
    }
}
