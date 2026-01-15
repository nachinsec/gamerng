use super::berserk::Berserk;
use super::person::Person;
use super::undead::Undead;
use crate::stats::Stats;
#[derive(Debug)]
pub enum Role {
    Person(Person),
    Undead(Undead),
    Berserk(Berserk),
}

impl Role {
    pub fn person() -> Self {
        Role::Person(Person)
    }

    pub fn undead() -> Self {
        Role::Undead(Undead)
    }

    pub fn berserk() -> Self {
        Role::Berserk(Berserk::new())
    }

    pub fn is_defeated(&self, stats: &Stats) -> bool {
        match self {
            Role::Person(_) => stats.hp() == 0,
            Role::Undead(_) => stats.energy() == 0,
            Role::Berserk(_) => stats.hp() == 0,
        }
    }

    pub fn can_act(&self, stats: &Stats) -> bool {
        match self {
            Role::Person(_) => stats.energy() > 0,
            Role::Undead(_) => stats.energy() > 0,
            Role::Berserk(_) => true,
        }
    }

    pub fn take_dmg(&mut self, stats: &mut Stats, dmg: u32) {
        match self {
            Role::Person(_) => {
                stats.reduce_hp(dmg);
            }
            Role::Undead(_) => {}
            Role::Berserk(berserk) => {
                stats.reduce_hp(dmg);
                berserk.gain_rage(dmg);
            }
        }
    }

    pub fn spend_energy(&mut self, stats: &mut Stats, energy: u32) {
        match self {
            Role::Person(_) | Role::Undead(_) => {
                stats.reduce_energy(energy);
            }
            Role::Berserk(_) => {}
        }
    }
}
