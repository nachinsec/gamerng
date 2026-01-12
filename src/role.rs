use crate::player::Player;
#[derive(Clone, Copy)]
pub enum Role {
    Person,
    Undead,
    Berserk
}

impl Role {

    pub fn is_defeated (&self, player: &Player) -> bool {
        match self {
            Role::Person => player.hp() == 0,
            Role::Undead => player.energy() == 0,
            Role::Berserk => player.hp() == 0
        }
    }

    pub fn can_act (&self, player: &Player) -> bool {
        match self {
            Role::Person => player.energy() > 0,
            Role::Undead => player.energy() > 0,
            Role::Berserk => true
        }
    }

    pub fn take_dmg (&self, player: &mut Player, dmg: u32) {
        match self {
            Role::Person | Role::Berserk => player.reduce_hp(dmg),
            Role::Undead => {}
        }
    }

    pub fn spend_energy (&self, player: &mut Player, energy: u32) {
        match self {
            Role::Person | Role::Undead => player.reduce_energy(energy),
            Role::Berserk => {}
        }
    }
}