use crate::{
    combat::{Combat, CombatState},
    models::{enemy::Enemy, perks_lib::get_all_perks, player::Player},
};
use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};

pub struct Game {
    player: Player,
    floor: u32,
    next_reward: u32,
}

impl Game {
    pub fn new(player: Player, floor: u32) -> Game {
        let mut rng = rand::rng();
        let next_reward = floor + rng.random_range(2..=4);
        Game {
            player,
            floor,
            next_reward,
        }
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
                if self.floor >= self.next_reward {
                    self.process_reward();

                    let mut rng = rand::rng();
                    self.next_reward = self.floor + rng.random_range(2..=4);
                }
                println!("🎉 ¡Enemigo derrotado! Avanzas al siguiente nivel...");
                println!("Vida: {:?}", self.player.stats());
                self.floor += 1;
            }
        }
    }

    pub fn process_reward(&mut self) {
        let mut rng = rand::rng();
        let perks = get_all_perks();
        let choices: Vec<_> = perks.choose_multiple(&mut rng, 3).collect();
        println!("\n🎁 ¡Recompensa! Elige una mejora:");
        for (index, choice) in choices.iter().enumerate() {
            println!("{}. {} - {}", index + 1, choice.name, choice.description);
        }

        loop {
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_err() {
                continue;
            }

            match input.trim().parse::<usize>() {
                Ok(n) if n > 0 && n <= choices.len() => {
                    let selected = choices[n - 1].clone();
                    println!("¡Has elegido: {}!", selected.name);
                    self.player.add_perk(selected);
                    break;
                }
                _ => println!("Opción inválida. Elige entre 1 y {}.", choices.len()),
            }
        }
    }
}
