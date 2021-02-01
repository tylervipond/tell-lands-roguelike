use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use specs::{
    error::NoError,
    saveload::{ConvertSaveload, Marker},
    Component, DenseVecStorage, Entity,
};

use super::causes_damage::DamageType;

#[derive(Component, ConvertSaveload, Clone, Debug, Default)]
pub struct DamageHistory {
    pub events: HashSet<DamageType>,
}

impl DamageHistory {
    pub fn describe_in_past_tense(&self) -> String {
        let mut terms: Vec<&str> = self
            .events
            .iter()
            .map(|event| match event {
                DamageType::Blunt => "bludgeoned",
                DamageType::Burn => "burnt",
                DamageType::Hack => "hacked",
                DamageType::Slash => "slashed",
                DamageType::Stab => "stabbed",
                DamageType::Pierce => "pierced",
                DamageType::Crush => "crushed",
            })
            .collect();
        if terms.len() > 1 {
            terms.insert(terms.len() - 1, "and");
        }
        if terms.len() < 4 {
            return terms.join(" ");
        }
        let terms_end = terms.split_off(terms.len() - 3).join(" ");
        [terms.join(","), terms_end].join(" ")
    }
}
