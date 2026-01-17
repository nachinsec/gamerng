#![allow(dead_code)] // TODO: delete warns
mod action;
mod combat;
mod enemy;
mod player;
mod roles;
mod stats;

use crate::enemy::Enemy;
use combat::Combat;
use player::Player;
use roles::Role;
use std::io;
fn main() {
    let role = choose_role();
    let player = Player::new(role);
    let enemy = Enemy::new(15, 3);

    let combat = Combat::new(player, enemy);
    let (result, player) = combat.run();

    println!("Resultado {:?}", result);
    println!("Player: {:?}", player);
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
