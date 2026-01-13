use crate::stats::Stats;

#[derive(Clone, Copy)]
pub enum Role {
    Person,
    Undead,
    Berserk
}

impl Role {

    pub fn is_defeated (&self, stats: &Stats) -> bool {
        match self {
            Role::Person => stats.hp() == 0,
            Role::Undead => stats.energy() == 0,
            Role::Berserk => stats.hp() == 0
        }
    }

    pub fn can_act (&self, stats: &Stats) -> bool {
        match self {
            Role::Person => stats.energy() > 0,
            Role::Undead => stats.energy() > 0,
            Role::Berserk => true
        }
    }

    pub fn take_dmg (&self, stats: &mut Stats, dmg: u32) {
        match self {
            Role::Person | Role::Berserk => stats.reduce_hp(dmg),
            Role::Undead => {}
        }
    }

    pub fn spend_energy (&self, stats: &mut Stats, energy: u32) {
        match self {
            Role::Person | Role::Undead => stats.reduce_energy(energy),
            Role::Berserk => {}
        }
    }
}