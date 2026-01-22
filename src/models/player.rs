use crate::models::events::Trigger;
use crate::models::perk::Perk;
use crate::models::roles::Role;
use crate::models::stats::Stats;
#[derive(Debug)]
pub struct Player {
    role: Role,
    stats: Stats,
    perks: Vec<Perk>,
}

impl Player {
    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn perks(&self) -> &[Perk] {
        &self.perks
    }

    pub fn new(role: Role) -> Player {
        let stats = role.def_stats();
        Player {
            role,
            stats,
            perks: Vec::new(),
        }
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

    pub fn attack(&mut self) -> u32 {
        let dmg = self.role.attack();
        self.trigger_event(Trigger::OnAttack);
        dmg
    }

    pub fn add_perk(&mut self, perk: Perk) {
        self.perks.push(perk);
    }

    fn trigger_event(&mut self, trigger: Trigger) {
        for perk in &self.perks {
            if perk.trigger == trigger {
                print!("trigger!!")
            }
        }
    }
}
