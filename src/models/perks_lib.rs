use crate::models::events::{Effect, Rarity, Trigger};
use crate::models::perk::Perk;

pub fn get_all_perks() -> Vec<Perk> {
    vec![
        Perk::new(
            "Vampirism",
            "Heal 1 HP when attacking",
            Rarity::Common,
            Trigger::OnAttack,
            Effect::Heal(1),
        ),
        Perk::new(
            "Greater Vampirism",
            "Heal 3 HP when attacking",
            Rarity::Rare,
            Trigger::OnAttack,
            Effect::Heal(3),
        ),
        Perk::new(
            "Survivor",
            "Heal 1 HP when taking damage",
            Rarity::Common,
            Trigger::OnTakeDamage,
            Effect::Heal(1),
        ),
    ]
}
