mod player;
mod role;
mod stats;

use player::Player;
use role::Role;
use stats::Stats;

fn main() {
    // Player normal
    let mut person = Player::new(
        Role::Person,
        Stats::new(10, 5),
    );

    // Player Berserk
    let mut berserk = Player::new(
        Role::Berserk,
        Stats::new(15, 0),
    );

    // Player Undead
    let mut undead = Player::new(
        Role::Undead,
        Stats::new(0, 8),
    );

    println!("== Estado inicial ==");
    print_player("Person", &person);
    print_player("Berserk", &berserk);
    print_player("Undead", &undead);

    println!("\n== Turno: reciben 4 de daño ==");
    person.take_dmg(4);
    berserk.take_dmg(4);
    undead.take_dmg(4);

    print_player("Person", &person);
    print_player("Berserk", &berserk);
    print_player("Undead", &undead);

    println!("\n== Turno: intentan gastar 3 de energía ==");
    person.spend_energy(3);
    berserk.spend_energy(3);
    undead.spend_energy(3);

    print_player("Person", &person);
    print_player("Berserk", &berserk);
    print_player("Undead", &undead);

    println!("\n== Checks finales ==");
    check_status("Person", &person);
    check_status("Berserk", &berserk);
    check_status("Undead", &undead);
}

fn print_player(name: &str, player: &Player) {
    let stats = player.stats();
    println!(
        "{} -> HP: {}, Energy: {}",
        name,
        stats.hp(),
        stats.energy()
    );
}

fn check_status(name: &str, player: &Player) {
    println!(
        "{} | can_act: {} | defeated: {}",
        name,
        player.role().can_act(player.stats()),
        player.is_defeated()
    );
}
