use crate::role::{self, Role};

pub struct Player {
    role: Role,
    hp: u32,
    energy: u32,
}

impl Player {

    pub fn role (&self) -> &Role {
        &self.role
    }
    pub fn hp (&self) -> u32 {
        self.hp
    }
    pub fn energy (&self) -> u32 {
        self.energy
    }

    pub fn new (role: Role, hp: u32, energy: u32) -> Player{
        Player {
            role,
            hp,
            energy
        }
    }

    pub (crate) fn reduce_hp(&mut self, hp: u32) {
        self.hp = self.hp.saturating_sub(hp) // saturating_sub 10-20 = 0
    }

    pub (crate) fn reduce_energy(&mut self, energy: u32) {
        self.energy = self.energy.saturating_sub(energy)
    }

    pub fn is_defeated (&self) -> bool {
        self.role.is_defeated(self)
    }

    pub fn take_dmg (&mut self, dmg: u32) {
        let role = self.role;
        role.take_dmg(self, dmg)
    } 

    pub fn spend_energy(&mut self, energy: u32) {
        let role = self.role;
        role.spend_energy(self, energy);
    }
}