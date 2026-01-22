#![allow(dead_code)] // TODO: delete warns

mod combat;
mod dice;
mod game;
mod models;

use crate::game::Game;
use models::player::Player;
use models::roles::Role;
use std::io;
fn main() {
    let role = choose_role();
    let player = Player::new(role);

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
