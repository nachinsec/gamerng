use std::io;

use crate::{action::Action, enemy::Enemy, player::Player};
#[derive(PartialEq, Debug)]
pub enum CombatState {
    Win,
    Lose,
    Fighting,
}
pub struct Combat {
    player: Player,
    enemy: Enemy,
    combat_state: CombatState,
}

impl Combat {
    pub fn new(player: Player, enemy: Enemy) -> Combat {
        Combat {
            player,
            enemy,
            combat_state: CombatState::Fighting,
        }
    }

    pub fn run(mut self) -> (CombatState, Player) {
        while self.combat_state == CombatState::Fighting {
            let action = get_player_action();

            match action {
                Action::Attack => {
                    print!("Player choose Attack!");
                    self.enemy.take_dmg(5);
                    println!(
                        "\n Player HP: {} | Enemy HP: {}",
                        self.player.stats().hp(),
                        self.enemy.hp()
                    );
                    if self.enemy.is_defeated() {
                        self.combat_state = CombatState::Win;
                        break;
                    }
                    self.player.take_dmg(self.enemy.attack());
                    if self.player.is_defeated() {
                        self.combat_state = CombatState::Lose;
                        break;
                    }
                }
                Action::Defend => {
                    self.player.take_dmg(self.enemy.attack() / 2);
                    if self.player.is_defeated() {
                        self.combat_state = CombatState::Lose;
                        break;
                    }
                }
                Action::Rest => {
                    // TODO heal
                    self.player.take_dmg(self.enemy.attack());
                    if self.player.is_defeated() {
                        self.combat_state = CombatState::Lose;
                        break;
                    }
                }
            }
        }
        (self.combat_state, self.player)
    }
}

fn get_player_action() -> Action {
    println!("Choose action");
    println!("1. Attack");
    println!("2. Defend");
    println!("3. Rest");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error stdin");

    match input.trim() {
        "1" => Action::Attack,
        "2" => Action::Defend,
        "3" => Action::Rest,
        _ => {
            println!("Invalid option. Attack by default.");
            Action::Attack
        }
    }
}
