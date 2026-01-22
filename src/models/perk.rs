use crate::models::events::{Effect, Rarity, Trigger};

#[derive(Debug, Clone)]
pub struct Perk {
    pub name: String,
    pub description: String,
    pub rarity: Rarity,
    pub trigger: Trigger,
    pub effect: Effect,
}

impl Perk {
    pub fn new(name: &str, desc: &str, rarity: Rarity, trigger: Trigger, effect: Effect) -> Self {
        Perk {
            name: name.to_string(),
            description: desc.to_string(),
            rarity,
            trigger,
            effect,
        }
    }
}
