use crate::roles::Role;
use crate::stats::Stats;
#[derive(Debug)]
pub struct Player {
    role: Role,
    stats: Stats,
}

impl Player {
    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn new(role: Role, stats: Stats) -> Player {
        Player { role, stats }
    }

    pub fn is_defeated(&self) -> bool {
        self.role.is_defeated(&self.stats)
    }

    pub fn take_dmg(&mut self, dmg: u32) {
        self.role.take_dmg(&mut self.stats, dmg)
    }

    pub fn spend_energy(&mut self, energy: u32) {
        self.role.spend_energy(&mut self.stats, energy);
    }
}
