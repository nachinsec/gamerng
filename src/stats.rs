#[derive(Debug)]
pub struct Stats {
    hp: u32,
    energy: u32,
}

impl Stats {
    pub fn new(hp: u32, energy: u32) -> Stats {
        Stats { hp, energy }
    }
    pub fn hp(&self) -> u32 {
        self.hp
    }
    pub fn energy(&self) -> u32 {
        self.energy
    }

    pub fn reduce_hp(&mut self, hp: u32) {
        self.hp = self.hp.saturating_sub(hp) // saturating_sub 10-20 = 0
    }

    pub fn reduce_energy(&mut self, energy: u32) {
        self.energy = self.energy.saturating_sub(energy)
    }
}
