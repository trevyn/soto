pub mod tutorial;

mod logger;

use crate::logger::GameLogger;
use rand::seq::SliceRandom;

pub struct Player {
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub max_mana: u32,
    pub current_mana: u32,
    pub health: u32,
}
impl Player {
    pub fn new() -> Self {
        let mut deck = vec![
            Card {
                name: "Mountain Strike".to_string(),
                attack: 3,
                defense: 1,
                lucky: false,
                special_ability: None,
                mana_cost: 1,
            },
            Card {
                name: "Stone Shield".to_string(),
                attack: 1,
                defense: 3,
                lucky: false,
                special_ability: None,
                mana_cost: 1,
            },
            Card {
                name: "Avalanche".to_string(),
                attack: 5,
                defense: 0,
                lucky: false,
                special_ability: Some(SpecialAbility::SummonAvalanche(2)),
                mana_cost: 3,
            },
        ];
        deck.extend(deck.clone());
        deck.extend(deck.clone());

        Player {
            deck,
            hand: Vec::new(),
            discard_pile: Vec::new(),
            max_mana: 10,
            current_mana: 10,
            health: 30,
        }
    }

    pub fn restore_mana(&mut self) {
        self.current_mana = self.max_mana;
    }

    pub fn play_card(&mut self, card_index: usize) -> Option<Card> {
        if card_index < self.hand.len() {
            let card = &self.hand[card_index];
            if self.current_mana >= card.mana_cost {
                let card = self.hand.remove(card_index);
                self.current_mana -= card.mana_cost;
                Some(card)
            } else {
                None
            }
        } else {
            None
        }
    }

    // ... existing methods ...
}

#[derive(Debug, Clone)]
pub enum SpecialAbility {
    Heal(u32),
    DrawCards(u32),
    ApplyPoison(u32),
    StunEnemy(u32),

    SummonAvalanche(u32),
}

pub struct CoreGameState {
    pub player: Player,
    pub enemy: Enemy,
    logger: GameLogger,
    turn_counter: u32,
}

impl CoreGameState {
    pub fn get_turn_count(&self) -> u32 {
        self.turn_counter
    }

    pub fn handle_turn_events(&mut self) {
        let event = rand::random::<f32>();

        if event < 0.1 {
            self.log("A sudden gust of wind sweeps across the battlefield!".to_string());
            self.player.health = self.player.health.saturating_add(1);
            self.enemy.health = self.enemy.health.saturating_add(1);
            self.log("Both you and the enemy recover 1 health.".to_string());
        } else if event < 0.2 {
            self.log("The ground trembles beneath your feet!".to_string());
            let damage = rand::random::<u32>() % 3 + 1;
            self.player.health = self.player.health.saturating_sub(damage);
            self.enemy.health = self.enemy.health.saturating_sub(damage);
            self.log(format!("Both you and the enemy take {} damage.", damage));
        } else if event < 0.3 {
            self.log("A mysterious energy fills the air...".to_string());
            self.player.max_mana = self.player.max_mana.saturating_add(1);
            self.player.current_mana = self.player.current_mana.saturating_add(1);
            self.log("Your maximum mana increases by 1!".to_string());
        } else if event < 0.4 {
            self.log("The mountain's ancient power surges through you!".to_string());
            if let Some(card) = self.draw_card() {
                self.log(format!("You draw an extra card: {}", card.name));
            }
        }
    }

    pub fn increment_turn(&mut self) {
        self.turn_counter += 1;
        self.log(format!("Turn {} begins", self.turn_counter));
    }

    pub fn log(&mut self, message: String) {
        self.logger.add_entry(message);
    }

    pub fn get_log(&self) -> &[String] {
        self.logger.get_log()
    }

    pub fn add_user_comment(&mut self, comment: String) {
        self.log(format!("User comment: {}", comment));
    }

    pub fn get_hand(&self) -> &[Card] {
        &self.player.hand
    }

    pub fn get_player_health(&self) -> u32 {
        self.player.health
    }

    pub fn get_enemy_health(&self) -> u32 {
        self.enemy.health
    }

    pub fn check_game_over(&self) -> Option<String> {
        if self.player.health == 0 {
            Some("Game Over: You have been defeated!".to_string())
        } else if self.enemy.health == 0 {
            Some("Congratulations! You have defeated the enemy!".to_string())
        } else {
            None
        }
    }
}

impl CoreGameState {
    pub fn draw_card(&mut self) -> Option<Card> {
        if self.player.deck.is_empty() {
            self.player.deck.append(&mut self.player.discard_pile);
            self.player.deck.shuffle(&mut rand::thread_rng());
        }
        self.player.deck.pop().map(|card| {
            self.player.hand.push(card.clone());
            card
        })
    }
    pub fn handle_combat(&mut self, card: &Card) -> String {
        let enemy_health_before = self.enemy.health;
        let player_health_before = self.player.health;

        // Apply card effects
        let mut damage_dealt = self.enemy.take_damage(card.attack);
        self.player.health = self.player.health.saturating_add(card.defense);

        // Handle special ability if present
        if let Some(ability) = &card.special_ability {
            damage_dealt += self.handle_special_ability(ability);
        }

        // Enemy counterattack
        if self.enemy.health > 0 {
            let damage_taken = self.enemy.attack.saturating_sub(card.defense);
            self.player.health = self.player.health.saturating_sub(damage_taken);
        }

        format!(
            "You dealt {} damage. Enemy health: {} -> {}. Your health: {} -> {}.",
            damage_dealt,
            enemy_health_before,
            self.enemy.health,
            player_health_before,
            self.player.health
        )
    }

    // ... existing methods ...

    pub fn handle_special_ability(&mut self, ability: &SpecialAbility) -> u32 {
        match ability {
            SpecialAbility::Heal(amount) => {
                let heal_amount = self.player.health.saturating_add(*amount) - self.player.health;
                self.player.health += heal_amount;
                self.log(format!("Player healed for {} health", heal_amount));
                self.log(format!(
                    "{} scoffs: \"Your pitiful healing won't save you!\"",
                    self.enemy.name
                ));
                0
            }
            SpecialAbility::DrawCards(amount) => {
                for _ in 0..*amount {
                    if let Some(card) = self.draw_card() {
                        self.log(format!("Player drew: {}", card.name));
                    }
                }
                self.log(format!(
                    "{} taunts: \"Draw all you want, it won't change your fate!\"",
                    self.enemy.name
                ));
                0
            }
            SpecialAbility::ApplyPoison(amount) => {
                self.enemy.apply_poison(*amount);
                self.log(format!("Applied {} poison to the enemy", amount));
                0
            }
            SpecialAbility::StunEnemy(duration) => {
                self.enemy.apply_stun(*duration);
                self.log(format!("Stunned the enemy for {} turns", duration));
                0
            }
            SpecialAbility::SummonAvalanche(damage) => {
                let actual_damage = self.enemy.take_damage(*damage);
                self.log(format!(
                    "You summon an avalanche, dealing {} damage to the enemy!",
                    actual_damage
                ));
                self.log(format!(
                    "{} roars: \"Your pathetic avalanche is nothing compared to my mountain-forged armor!\"",
                    self.enemy.name
                ));
                actual_damage
            }
        }
    }
    pub fn new() -> Self {
        let enemy = Enemy::new(
            "Mountain Sentinel".to_string(),
            20,
            2,
            vec![
                "Your primitive tactics are no match for my ancient strength!".to_string(),
                "Prepare to be crushed beneath the weight of these peaks, human!".to_string(),
                "Your defeat is as certain as the eternal snow on these mountaintops!".to_string(),
                "Initiating avalanche protocols...".to_string(),
                "Your extinction is inevitable. Surrender now and become one with the mountain!"
                    .to_string(),
            ],
        );
        let enemy_stats = format!(
            "Enemy stats: {} - Health = {}, Attack = {}",
            enemy.name, enemy.health, enemy.attack
        );

        let mut core_state = Self {
            player: Player::new(),
            enemy,
            logger: GameLogger::new(),
            turn_counter: 0,
        };

        core_state.log("The crisp mountain air suddenly turns electric, a surge of cosmic energy rippling through the ancient peaks!".to_string());
        core_state.log("The rugged landscape itself seems to warp and bend as an otherworldly presence begins to materialize...".to_string());
        core_state.log(format!("With a thunderous roar that echoes off the mountain walls, the {} emerges from a portal torn into the fabric of reality! Its form, a terrifying fusion of advanced alien technology and raw, destructive power, looms against the backdrop of snow-capped summits.", core_state.enemy.name));
        core_state.log("Rocks crumble from nearby cliffs, and you feel the weight of impending doom pressing down on you, as heavy as the mountains themselves.".to_string());
        core_state.log(format!("The {}'s eyes, glowing with an eerie red light that outshines even the setting sun, scan you coldly. You feel as if your very soul is being analyzed.", core_state.enemy.name));
        core_state.log("The thin mountain air grows thick with tension, humming with the weight of the coming battle.".to_string());
        core_state.log(format!("{}'s cybernetic eyes pulse with an unholy red glow, scanning you with cold, calculated malice that seems to freeze the very air around you.", core_state.enemy.name));
        core_state.log(enemy_stats);
        core_state.log(format!(
            "Standing tall against the mountainous backdrop, {}'s glowing red eyes fix upon you as it unleashes a chilling declaration:",
            core_state.enemy.name
        ));
        core_state.log(format!("\"{}\"", core_state.enemy.taunt()));
        core_state.log("The very rocks beneath your feet seem to tremble. Here, amidst the towering peaks, the battle for the fate of your world begins NOW!".to_string());

        // Add player introduction
        core_state.log("\nYou stand at the edge of a narrow mountain pass, the wind whipping around you, carrying the scent of snow and distant pine forests.".to_string());
        core_state.log("Your breath mists in the cold air as you face the Mountain Sentinel, your hand instinctively reaching for your deck of mystical cards.".to_string());
        core_state.log("The ancient spirits of the mountains seem to whisper encouragement as you prepare to defend your world from this otherworldly threat.".to_string());

        // Draw initial hand with enhanced descriptions
        core_state.log("\nYou draw your initial hand:".to_string());
        for _ in 0..5 {
            if let Some(card) = core_state.draw_card() {
                core_state.log(format!("- {}: A card infused with the power of {} (Attack: {}, Defense: {}, Mana Cost: {})",
                    card.name,
                    card.name.to_lowercase(),
                    card.attack,
                    card.defense,
                    card.mana_cost
                ));
            }
        }

        core_state
    }
    pub fn play_card(&mut self, card_index: i32) -> String {
        if card_index < 0 || card_index as usize >= self.player.hand.len() {
            return "Invalid card index".to_string();
        }
        if let Some(card) = self.player.play_card(card_index as usize) {
            let result = self.handle_combat(&card);
            let mut log_message = format!(
                "Played card: {} (Mana cost: {}). {}",
                card.name, card.mana_cost, result
            );

            if self.enemy.health > 0 {
                let enemy_reaction =
                    format!("\n{} reacts: \"{}\"", self.enemy.name, self.enemy.taunt());
                log_message.push_str(&enemy_reaction);
            } else {
                let enemy_defeat = format!(
                    "\n{} wails: \"Impossible! I cannot be defeated by a mere human!\"",
                    self.enemy.name
                );
                log_message.push_str(&enemy_defeat);
            }

            self.log(log_message.clone());
            log_message
        } else {
            "Not enough mana to play this card".to_string()
        }
    }

    pub fn enemy_turn(&mut self) {
        self.handle_turn_events();

        let poison_damage = self.enemy.take_poison_damage();
        if poison_damage > 0 {
            self.log(format!(
                "{} took {} poison damage",
                self.enemy.name, poison_damage
            ));
            self.log(format!(
                "{} hisses: \"Your toxins are mere annoyances to my superior systems!\"",
                self.enemy.name
            ));
        }
        if self.enemy.health > 0 {
            if self.enemy.is_stunned() {
                self.log(format!("{} is stunned and cannot attack!", self.enemy.name));
                self.log(format!(
                    "{} growls: \"This... delay... changes nothing!\"",
                    self.enemy.name
                ));
            } else {
                let mut damage = self.enemy.attack;

                // Turn-based event: Enemy power surge
                if self.turn_counter % 3 == 0 {
                    damage += 1;
                    self.log(format!(
                        "{} surges with power, increasing its attack!",
                        self.enemy.name
                    ));
                }

                self.player.health = self.player.health.saturating_sub(damage);
                self.log(format!(
                    "{} attacks! You take {} damage. Your current health: {}",
                    self.enemy.name, damage, self.player.health
                ));
                self.log(format!(
                    "{} taunts: \"{}\"",
                    self.enemy.name,
                    self.enemy.taunt()
                ));
            }
        }
    }
}

pub struct Enemy {
    pub name: String,
    pub health: u32,
    pub attack: u32,
    pub poison: u32,
    pub taunts: Vec<String>,
    pub stunned: u32,
    pub shield: u32,
    pub rage: u32,
}

impl Enemy {
    pub fn new(name: String, health: u32, attack: u32, taunts: Vec<String>) -> Self {
        Enemy {
            name,
            health,
            attack,
            poison: 0,
            taunts,
            stunned: 0,
            shield: 0,
            rage: 0,
        }
    }

    pub fn apply_shield(&mut self, amount: u32) {
        self.shield = self.shield.saturating_add(amount);
    }

    pub fn increase_rage(&mut self, amount: u32) {
        self.rage = self.rage.saturating_add(amount);
        if self.rage >= 100 {
            self.attack += 1;
            self.rage = 0;
        }
    }

    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let damage_after_shield = amount.saturating_sub(self.shield);
        self.shield = self.shield.saturating_sub(amount);
        self.health = self.health.saturating_sub(damage_after_shield);
        self.increase_rage(damage_after_shield);
        damage_after_shield
    }

    pub fn taunt(&self) -> &str {
        static DEFAULT_TAUNT: &str = "...";
        self.taunts
            .choose(&mut rand::thread_rng())
            .map(String::as_str)
            .unwrap_or(DEFAULT_TAUNT)
    }

    pub fn apply_stun(&mut self, duration: u32) {
        self.stunned = self.stunned.saturating_add(duration);
    }

    pub fn is_stunned(&mut self) -> bool {
        if self.stunned > 0 {
            self.stunned -= 1;
            true
        } else {
            false
        }
    }

    pub fn apply_poison(&mut self, amount: u32) {
        self.poison += amount;
    }

    pub fn take_poison_damage(&mut self) -> u32 {
        let damage = self.poison;
        self.health = self.health.saturating_sub(damage);
        self.poison = self.poison.saturating_sub(1);
        if damage > 0 {
            println!("{} sizzles: \"Your poison... it burns!\"", self.name);
        }
        damage
    }
}

#[derive(Clone, Debug)]
pub struct Card {
    pub name: String,
    pub attack: u32,
    pub defense: u32,
    pub lucky: bool,
    pub special_ability: Option<SpecialAbility>,
    pub mana_cost: u32,
}
