#![allow(dead_code)] // TODO: delete warns

mod combat;
mod dice;
mod game;
mod models;

use crate::game::Game;
use models::events::{Effect, Rarity, Trigger};
use models::perk::Perk;
use models::player::Player;
use models::roles::Role;
use std::io;

fn main() {
    let role = choose_role();
    let mut player = Player::new(role);

    // Test perk: Vampirism - heals 3 HP on attack
    let vampirism = Perk::new(
        "Vampirism",
        "Heal 3 HP when attacking",
        Rarity::Rare,
        Trigger::OnAttack,
        Effect::Heal(3),
    );
    player.add_perk(vampirism);

    println!("Player starts with perk: Vampirism (Heal 3 on attack)");
    println!("Initial HP: {}", player.stats().vital_value());

    let mut game = Game::new(player, 1);

    game.play();
}

fn choose_role() -> Role {
    println!("Choose role");
    println!("1. Person");
    println!("2. Berserk");
    println!("3. Undead");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error input");

    match input.trim() {
        "1" => {
            println!("Person role selected!");
            Role::person()
        }
        "2" => {
            println!("Berserk role selected!");
            Role::berserk()
        }
        "3" => {
            println!("Undead role selected!");
            Role::undead()
        }
        _ => {
            println!("Error input. Person role selected!");
            Role::person()
        }
    }
}
