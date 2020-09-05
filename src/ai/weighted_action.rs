use super::Action;
use std::cmp::Ordering;
#[derive(Copy, Clone)]
pub struct WeightedAction {
    pub action: Action,
    pub weight: f32,
}

impl WeightedAction {
    pub fn new(action: Action, weight: f32) -> Self {
        Self { action, weight }
    }
}

impl Eq for WeightedAction {

}

impl Ord for WeightedAction {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.weight > other.weight {
            return Ordering::Greater;
        }
        if self.weight < other.weight {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl PartialOrd for WeightedAction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl PartialEq for WeightedAction {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
