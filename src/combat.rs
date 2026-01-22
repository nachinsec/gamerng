use std::io;

use crate::models::action::Action;
use crate::models::enemy::Enemy;
use crate::models::player::Player;

#[derive(PartialEq, Debug)]
pub enum CombatState {
    Win,
    Lose,
    Fighting,
}
pub struct Combat {
    enemy: Enemy,
    combat_state: CombatState,
}

impl Combat {
    pub fn new(enemy: Enemy) -> Combat {
        Combat {
            enemy,
            combat_state: CombatState::Fighting,
        }
    }

    pub fn run(mut self, player: &mut Player) -> CombatState {
        while self.combat_state == CombatState::Fighting {
            let action = get_player_action();

            match action {
                Action::Attack => {
                    print!("Player choose Attack!");
                    self.enemy.take_dmg(player.attack());
                    println!(
                        "\n Player Status: {:?} | Enemy HP: {}",
                        player.role(),
                        self.enemy.hp()
                    );
                    if self.enemy.is_defeated() {
                        self.combat_state = CombatState::Win;
                        break;
                    }
                    player.take_dmg(self.enemy.attack());
                    if player.is_defeated() {
                        self.combat_state = CombatState::Lose;
                        break;
                    }
                }
                Action::Defend => {
                    player.take_dmg(self.enemy.attack() / 2);
                    if player.is_defeated() {
                        self.combat_state = CombatState::Lose;
                        break;
                    }
                }
                Action::Rest => {
                    // TODO heal
                    player.take_dmg(self.enemy.attack());
                    if player.is_defeated() {
                        self.combat_state = CombatState::Lose;
                        break;
                    }
                }
            }
        }
        self.combat_state
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
