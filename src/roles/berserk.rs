pub struct Berserk {
    rage: u32
}
impl Berserk {
    pub fn new() -> Self {
        Self { rage: 0 }
    }

    pub fn rage(&self) -> u32 {
        self.rage
    }

    pub fn gain_rage(&mut self, amount: u32) {
        self.rage = self.rage.saturating_add(amount);
    }

    pub fn consume_rage(&mut self, amount: u32) -> bool {
        if self.rage >= amount {
            self.rage -= amount;
            true
        } else {
            false
        }
    }
}
