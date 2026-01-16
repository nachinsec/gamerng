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
use stats::Stats;
use std::io;
fn main() {
    let (role, stats) = choose_role();
    let player = Player::new(role, stats);
    let enemy = Enemy::new(15, 3);

    let combat = Combat::new(player, enemy);
    let (result, player) = combat.run();

    println!("Resultado {:?}", result);
    println!("Player: {:?}", player);
}

fn choose_role() -> (Role, Stats) {
    println!("Choose role");
    println!("1. Person");
    println!("2. Berserk");
    println!("3. Undead");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error input");

    match input.trim() {
        "1" => {
            println!("Person role selected!");
            (Role::person(), Stats::new(20, 10))
        }
        "2" => {
            println!("Berserk role selected!");
            (Role::berserk(), Stats::new(30, 0))
        }
        "3" => {
            println!("Undead role selected!");
            (Role::undead(), Stats::new(0, 40))
        }
        _ => {
            println!("Error input. Person role selected!");
            (Role::person(), Stats::new(20, 10))
        }
    }
}
