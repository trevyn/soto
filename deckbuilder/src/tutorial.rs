use crate::{Card, CoreGameState};

pub struct TutorialState {
    pub step: usize,
    pub core_game: CoreGameState,
    enemy_health_before: u32,
    player_health_before: u32,
    card_played: Option<Card>,
    enemy_damage_dealt: u32,
    player_damage_taken: u32,
    enemy_turn_damage: u32,
}

impl TutorialState {
    pub fn is_complete(&self) -> bool {
        self.step >= 7
    }

    pub fn new() -> Self {
        let core_game = CoreGameState::new();
        let enemy_health_before = core_game.enemy.health;
        let player_health_before = core_game.player.health;
        Self {
            step: 0,
            core_game,
            enemy_health_before,
            player_health_before,
            card_played: None,
            enemy_damage_dealt: 0,
            player_damage_taken: 0,
            enemy_turn_damage: 0,
        }
    }

    pub fn next_step(&mut self) -> String {
        self.step += 1;
        self.get_current_instruction()
    }
    pub fn get_current_instruction(&self) -> String {
        match self.step {
            0 => "Welcome to the Deckbuilder Tutorial! Let's start by looking at your hand. Press Enter to continue.".to_string(),
            1 => "You start with 5 cards in your hand. Each card has an Attack and Defense value. Press Enter to continue.".to_string(),
            2 => "Let's play your first card. Type '1' to play the first card in your hand.".to_string(),
            3 => {
                let card = self.card_played.as_ref().unwrap();
                format!(
                    r#"--------------------------------------------------
BATTLE ACTION SUMMARY:
--------------------------------------------------
1. Your Move:
   You played: {} (Attack: {}, Defense: {})

2. Your Attack:
   Enemy Health: {} -> {}  (*-{} damage*)

3. Enemy Counterattack:
   Your Health: {} -> {}   (*-{} damage*)
   {}

4. Final Status:
   YOU  - Health: {}, Cards in hand: {}
   ENEMY - Health: {}, Attack: {}
--------------------------------------------------
Great job! You've successfully attacked the enemy and survived their counterattack.
Press Enter to continue the tutorial."#,
                    card.name, card.attack, card.defense,
                    self.enemy_health_before, self.core_game.enemy.health, self.enemy_damage_dealt,
                    self.player_health_before, self.core_game.player.health, self.player_damage_taken,
                    if self.player_damage_taken < self.core_game.enemy.attack {
                        format!("(You blocked {} damage with your {}'s defense!)",
                            self.core_game.enemy.attack - self.player_damage_taken, card.name)
                    } else {
                        String::new()
                    },
                    self.core_game.player.health, self.core_game.player.hand.len(),
                    self.core_game.enemy.health, self.core_game.enemy.attack
                )
            },
            4 => "Now it's the enemy's turn. They will attack you. Press Enter to see what happens.".to_string(),
            5 => format!(
                "The enemy attacked you! You took {} damage. Your health decreased from {} to {}. The game continues until either you or the enemy runs out of health. Press Enter to continue.",
                self.enemy_turn_damage,
                self.player_health_before,
                self.core_game.player.health
            ),
            6 => "That's the basics of combat! Keep playing cards and defeating enemies. Press Enter to finish the tutorial.".to_string(),
            _ => "Tutorial complete! You can now start a real game.".to_string(),
        }
    }
    pub fn handle_input(&mut self, input: &str) -> String {
        match self.step {
            2 => {
                if input == "1" {
                    self.enemy_health_before = self.core_game.enemy.health;
                    self.player_health_before = self.core_game.player.health;
                    self.card_played = self.core_game.player.hand.get(0).cloned();
                    let result = self.core_game.play_card(0);
                    println!("{}", result); // Print the result of playing the card
                    if let Some(_card) = &self.card_played {
                        self.enemy_damage_dealt =
                            self.enemy_health_before - self.core_game.enemy.health;
                        self.player_damage_taken =
                            self.player_health_before - self.core_game.player.health;
                    }
                    self.next_step();
                    String::new() // Return empty string when advancing
                } else {
                    "Please type '1' to play the first card.".to_string()
                }
            }

            4 | 5 | 6 => {
                if input.is_empty() {
                    if self.step == 4 {
                        self.player_health_before = self.core_game.player.health;
                        self.core_game.enemy_turn();
                        self.enemy_turn_damage =
                            self.player_health_before - self.core_game.player.health;
                        self.next_step();
                        format!(
                            "The enemy attacks! You take {} damage.",
                            self.enemy_turn_damage
                        )
                    } else {
                        self.next_step();
                        String::new()
                    }
                } else {
                    "Press Enter to continue.".to_string()
                }
            }
            _ => {
                if input.is_empty() {
                    self.next_step();
                    String::new() // Return empty string when advancing
                } else {
                    "Press Enter to continue.".to_string()
                }
            }
        }
    }
}
