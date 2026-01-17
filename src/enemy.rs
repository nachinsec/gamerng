use crate::dice::Dice;

pub struct Enemy {
    hp: u32,
    dmg: u32,
}

impl Enemy {
    pub fn new(hp: u32, dmg: u32) -> Enemy {
        Enemy { hp, dmg }
    }

    pub fn is_defeated(&self) -> bool {
        self.hp == 0
    }

    pub fn take_dmg(&mut self, dmg: u32) {
        self.hp = self.hp.saturating_sub(dmg);
    }

    pub fn attack(&self) -> u32 {
        Dice::d4() + self.dmg
    }

    pub fn hp(&self) -> u32 {
        self.hp
    }

    pub fn dmg(&self) -> u32 {
        self.dmg
    }
}
