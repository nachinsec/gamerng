use crate::models::{perk::Perk, stats::Stats};

#[derive(Debug, Clone, PartialEq)]
pub enum Trigger {
    OnAttack,
    OnTakeDamage,
}

#[derive(Debug, Clone)]
pub enum Effect {
    Heal(u32),
    Buff(u32),
    Reflect(f32), // percentage
}

#[derive(Debug, Clone)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

fn process_trigger(perks: &[Perk], stats: &mut Stats, trigger: Trigger) {
    for perk in perks {
        if perk.trigger == trigger {
            apply_effect(&perk.effect, stats);
        }
    }
}

fn apply_effect(effect: &Effect, stats: &mut Stats) {
    println!("test")
}
