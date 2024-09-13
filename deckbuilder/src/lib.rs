use godot::prelude::*;
use rand::seq::SliceRandom;

mod logger;
mod tutorial;

pub use tutorial::TutorialState;

#[derive(Clone, Debug)]
pub struct Card {
    pub name: String,
    pub attack: u32,
    pub defense: u32,
}

pub struct CoreGameState {
    pub player: Player,
    pub enemy: Enemy,
    pub logger: logger::GameLogger,
}
impl CoreGameState {
    pub fn add_user_comment(&mut self, comment: String) {
        let formatted_comment = format!("User Comment: {}", comment);
        self.log(formatted_comment);
    }
    pub fn new() -> Self {
        let mut player = Player::new();
        player.deck = initialize_deck();

        // Draw initial hand
        for _ in 0..5 {
            player.draw_card();
        }

        let enemy = Enemy::new(20, 2);
        let enemy_stats = format!(
            "Enemy stats: Health = {}, Attack = {}",
            enemy.health, enemy.attack
        );

        let mut core_state = Self {
            player,
            enemy,
            logger: logger::GameLogger::new(),
        };

        core_state.log("Game started".to_string());
        core_state.log(enemy_stats);
        core_state
    }

    pub fn log(&mut self, message: String) {
        self.logger.add_entry(message.clone());
    }

    pub fn draw_card(&mut self) {
        self.player.draw_card();
    }

    pub fn play_card(&mut self, card_index: i32) -> String {
        if card_index < 0 || card_index as usize >= self.player.hand.len() {
            let message = "Invalid card index".to_string();
            self.log(message.clone());
            return message;
        }
        if self.player.hand.is_empty() {
            let message = "No cards in hand".to_string();
            self.log(message.clone());
            return message;
        }
        let card = self.player.hand.remove(card_index as usize);
        let result = handle_combat(&mut self.player, &mut self.enemy, &card);
        self.log(format!("Played card: {}. {}", card.name, result));
        result
    }

    pub fn enemy_turn(&mut self) {
        let damage = self.enemy.attack;
        self.player.health = self.player.health.saturating_sub(damage);
        self.log(format!("Enemy attacks! You take {} damage.", damage));
    }

    pub fn get_player_health(&self) -> u32 {
        self.player.health
    }

    pub fn get_enemy_health(&self) -> u32 {
        self.enemy.health
    }

    pub fn get_hand(&self) -> Vec<Card> {
        self.player.hand.clone()
    }

    pub fn get_enemy_attack(&self) -> u32 {
        self.enemy.attack
    }

    pub fn check_game_over(&self) -> Option<String> {
        is_game_over(&self.player, &self.enemy)
    }

    pub fn get_log(&self) -> &[String] {
        self.logger.get_log()
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameState {
    #[base]
    base: Base<Node>,
    core: CoreGameState,
    tutorial: Option<TutorialState>,
}

#[godot_api]
impl GameState {
    #[func]
    pub fn start_tutorial(&mut self) {
        self.tutorial = Some(TutorialState::new());
    }

    #[func]
    pub fn get_tutorial_instruction(&self) -> GString {
        if let Some(tutorial) = &self.tutorial {
            GString::from(tutorial.get_current_instruction())
        } else {
            GString::from("Tutorial not started")
        }
    }

    #[func]
    pub fn handle_tutorial_input(&mut self, input: GString) -> GString {
        if let Some(tutorial) = &mut self.tutorial {
            let response = tutorial.handle_input(&input.to_string());
            if tutorial.is_complete() {
                self.tutorial = None;
            }
            GString::from(response)
        } else {
            GString::from("Tutorial not started")
        }
    }

    #[func]
    pub fn add_user_comment(&mut self, comment: GString) {
        self.core.add_user_comment(comment.to_string());
    }

    #[func]
    pub fn get_log(&self) -> Vec<GString> {
        self.core
            .logger
            .get_log()
            .iter()
            .map(|s| GString::from(s))
            .collect()
    }

    pub fn new(base: Base<Node>) -> Self {
        Self {
            base,
            core: CoreGameState::new(),
            tutorial: None,
        }
    }

    #[func]
    pub fn draw_card(&mut self) {
        self.core.draw_card();
    }

    #[func]
    pub fn play_card(&mut self, card_index: i32) -> GString {
        GString::from(self.core.play_card(card_index))
    }
    #[func]
    pub fn enemy_turn(&mut self) {
        let damage = self.core.enemy.attack;
        self.core.player.health = self.core.player.health.saturating_sub(damage);
        godot_print!("Enemy attacks! You take {} damage.", damage);
    }

    #[func]
    pub fn get_player_health(&self) -> u32 {
        self.core.player.health
    }

    #[func]
    pub fn get_enemy_health(&self) -> u32 {
        self.core.enemy.health
    }

    #[func]
    pub fn get_hand(&self) -> Vec<Dictionary> {
        self.core
            .player
            .hand
            .iter()
            .map(|card| {
                let mut dict = Dictionary::new();
                dict.insert("name", card.name.clone()).unwrap();
                dict.insert("attack", card.attack).unwrap();
                dict.insert("defense", card.defense).unwrap();
                dict
            })
            .collect()
    }

    #[func]
    pub fn get_enemy_attack(&self) -> u32 {
        self.core.enemy.attack
    }

    #[func]
    pub fn check_game_over(&self) -> GString {
        match is_game_over(&self.core.player, &self.core.enemy) {
            Some(message) => GString::from(message),
            None => GString::new(),
        }
    }

    #[func]
    pub fn is_tutorial_active(&self) -> bool {
        self.tutorial.is_some()
    }
}

#[godot_api]
impl INode for GameState {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            core: CoreGameState::new(),
            tutorial: None,
        }
    }
}

#[derive(Default)]
pub struct Player {
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub health: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            deck: Vec::new(),
            hand: Vec::new(),
            health: 30,
        }
    }

    pub fn draw_card(&mut self) {
        if let Some(card) = self.deck.pop() {
            self.hand.push(card);
        }
    }
}

#[derive(Default)]
pub struct Enemy {
    pub health: u32,
    pub attack: u32,
}

impl Enemy {
    pub fn new(health: u32, attack: u32) -> Self {
        Enemy { health, attack }
    }
}
pub fn handle_combat(player: &mut Player, enemy: &mut Enemy, card: &Card) -> String {
    let mut combat_log = String::new();

    // Player attacks enemy
    let enemy_health_before = enemy.health;
    enemy.health = enemy.health.saturating_sub(card.attack);
    let damage_dealt = enemy_health_before - enemy.health;
    combat_log.push_str(&format!(
        "You attack with {}. Enemy takes {} damage.",
        card.name, damage_dealt
    ));

    // Enemy counterattacks if still alive
    if enemy.health > 0 {
        let enemy_attack = enemy.attack;
        let damage_blocked = enemy_attack.saturating_sub(card.defense);
        let damage_taken = damage_blocked;
        player.health = player.health.saturating_sub(damage_taken);
        combat_log.push_str(&format!(
            "\nEnemy counterattacks with {} damage. ",
            enemy_attack
        ));
        if damage_blocked < enemy_attack {
            combat_log.push_str(&format!(
                "You block {} damage with your {}'s defense. ",
                enemy_attack - damage_blocked,
                card.name
            ));
        }
        combat_log.push_str(&format!("You take {} damage.", damage_taken));
    }

    combat_log
}

pub fn initialize_deck() -> Vec<Card> {
    let initial_cards = vec![
        Card {
            name: "Warrior".to_string(),
            attack: 3,
            defense: 2,
        },
        Card {
            name: "Archer".to_string(),
            attack: 2,
            defense: 1,
        },
        Card {
            name: "Knight".to_string(),
            attack: 4,
            defense: 4,
        },
    ];

    let mut deck = Vec::new();
    for _ in 0..5 {
        deck.extend(initial_cards.clone());
    }

    deck.shuffle(&mut rand::thread_rng());
    deck
}

pub fn is_game_over(player: &Player, enemy: &Enemy) -> Option<String> {
    if player.health <= 0 {
        Some("Game Over! You lost.".to_string())
    } else if enemy.health <= 0 {
        Some("Congratulations! You defeated the enemy.".to_string())
    } else {
        None
    }
}
