use crate::{enemy::Enemy, player::Player};
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
            self.enemy.take_dmg(5);
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
        (self.combat_state, self.player)
    }
}
