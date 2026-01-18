use crate::{
    combat::{Combat, CombatState},
    enemy::Enemy,
    player::Player,
};

pub struct Game {
    player: Player,
    floor: u32,
}

impl Game {
    pub fn new(player: Player, floor: u32) -> Game {
        Game { player, floor }
    }

    pub fn play(&mut self) {
        loop {
            if self.player.is_defeated() {
                println!("\n💀 GAME OVER 💀");
                println!("Caíste en el piso {}.", self.floor);
                break;
            }

            println!("\n=== 📍 PISO {} ===", self.floor);

            let enemy = Enemy::spawn(self.floor);
            println!(
                "⚔️  Aparece un enemigo con {} HP y {} de Fuerza extra.",
                enemy.hp(),
                enemy.dmg()
            );

            let combat = Combat::new(enemy);
            let result = combat.run(&mut self.player);

            if result == CombatState::Win {
                println!("🎉 ¡Enemigo derrotado! Avanzas al siguiente nivel...");
                self.floor += 1;
            }
        }
    }
}
