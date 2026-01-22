use super::berserk::Berserk;
use super::person::Person;
use super::undead::Undead;
use crate::{
    dice::Dice,
    models::stats::{Stats, VitalResource},
};
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

    pub fn def_stats(&self) -> Stats {
        match self {
            Role::Person(_) => Stats::new(VitalResource::Health(20), 10),
            Role::Berserk(_) => Stats::new(VitalResource::Health(30), 0),
            Role::Undead(_) => Stats::new(VitalResource::Health(0), 30),
            //Role::Robot(_) => Stats::new(VitalResource::Battery(20), 10),
        }
    }

    pub fn is_defeated(&self, stats: &Stats) -> bool {
        match self {
            Role::Undead(_) => stats.energy() == 0,
            _ => stats.vital_value() == 0,
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
                stats.reduce_vital(dmg);
            }
            Role::Undead(_) => {}
            Role::Berserk(berserk) => {
                stats.reduce_vital(dmg);
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

    pub fn attack(&mut self) -> u32 {
        match self {
            Role::Person(_) => 5 + Dice::d6(),
            Role::Berserk(b) => {
                if b.rage() > 5 {
                    b.rage() + Dice::d6()
                } else {
                    5 + Dice::d6()
                }
            }
            Role::Undead(_) => 5 + Dice::d6(),
        }
    }
}
