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

fn main() {
    let player = Player::new(Role::person(), Stats::new(20, 10));
    let enemy = Enemy::new(15, 3);

    let combat = Combat::new(player, enemy);
    let (result, player) = combat.run();

    println!("Resultado {:?}", result);
    println!("Player: {:?}", player);
}
