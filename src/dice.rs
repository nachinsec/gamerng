use rand::Rng;

pub struct Dice;

impl Dice {
    pub fn roll(sides: u32) -> u32 {
        let mut rng = rand::rng();
        rng.random_range(1..=sides)
    }

    pub fn d4() -> u32 {
        Self::roll(4)
    }
    pub fn d6() -> u32 {
        Self::roll(6)
    }

    pub fn d20() -> u32 {
        Self::roll(20)
    }
}
