// notes for AI assistant:
// this is running under Godot 4.4
// some stuff that used to be under godot::engine is now under godot::classes, so assume all that stuff is imported
// when using upcast(), you must turbofish the node type, e.g. upcast::<Node>()
// in general, we want to do everything from rust and not use any gdscript at all
// instead of e.g. Node::new(), it's now Node::new_gd()
// anything that might change a node probably takes &mut, so define the variable as mut

use godot::builtin::*;
use godot::classes::*;
use godot::prelude::*;
use rand::Rng;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    #[base]
    base: Base<Node>,
}
#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        godot_print!("GameManager initialized!");
        Self { base }
    }
    fn ready(&mut self) {
        godot_print!("GameManager ready() called");
        let my_player = MyPlayer::new_alloc();
        godot_print!("MyPlayer allocated: {:?}", my_player);
        self.base_mut()
            .add_child(my_player.clone().upcast::<Node>());
        godot_print!("MyPlayer added as child to GameManager");

        // Create a one-shot timer for delayed execution
        let mut timer = godot::classes::Timer::new_alloc();
        timer.set_one_shot(true);
        timer.set_autostart(true);
        timer.set_wait_time(0.1); // 100ms delay
        godot_print!("Timer created: {:?}", timer);
        timer.connect(
            "timeout".into(),
            self.base().callable("search_for_my_player"),
        );
        godot_print!("Timer connected to search_for_my_player");
        let timer_clone = timer.clone();
        self.base_mut()
            .add_child(timer_clone.upcast::<godot::classes::Timer>());
        godot_print!("Timer added as child to GameManager");
        timer.start();
        godot_print!("Timer started");
    }
}

#[godot_api]
impl GameManager {
    #[func]
    fn search_for_my_player(&mut self) {
        godot_print!("search_for_my_player called");
        godot_print!("Current node (GameManager): {:?}", self.base().get_name());

        for child in self.base().get_children().iter_shared() {
            let child_name = child.get_name().to_string();
            godot_print!("Child of GameManager: {}", child_name);

            if child_name.starts_with("@MyPlayer@") {
                if let Ok(my_player) = child.try_cast::<MyPlayer>() {
                    godot_print!("MyPlayer found: {:?}", my_player);
                    return;
                }
            }
        }

        godot_print!("MyPlayer not found in children of GameManager");
    }

    #[func]
    fn create_player(&mut self) {
        godot_print!("Creating new player");
        let my_player = MyPlayer::new_alloc();
        self.base_mut().add_child(my_player.upcast::<Node>());
        godot_print!("New player created and added as child");
    }
}
#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MyPlayer {
    #[base]
    base: Base<Node2D>,
    speed: f32,
    direction: Vector2,
    velocity: Vector2,
    shape: Gd<ColorRect>,
    boop_player: Option<Gd<AudioStreamPlayer2D>>,
    hue: f32,
    glow_shader: Gd<ShaderMaterial>,
}
#[godot_api]
impl INode2D for MyPlayer {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("MyPlayer initialized!");
        let mut rect = ColorRect::new_alloc();
        rect.set_size(Vector2::new(200.0, 200.0));
        rect.set_position(Vector2::new(-100.0, -100.0));
                rect.set_color(Color::from_rgba(1.0, 1.0, 1.0, 0.0)); // Fully transparent

        rect.set_clip_contents(false);

        let mut shader_material = ShaderMaterial::new_gd();
        let mut shader = Shader::new_gd();
        let shader_code = include_str!("glow_shader.gdshader");
        godot_print!("Shader code length: {}", shader_code.len());
        shader.set_code(shader_code.into());
        shader_material.set_shader(shader);
        godot_print!("Shader set on material: {:?}", shader_material.get_shader());

        // Apply the shader to the ColorRect
        rect.set_material(shader_material.clone().upcast::<Material>());
        godot_print!("Material set on ColorRect: {:?}", rect.get_material());
        rect.set_clip_contents(false);

        Self {
            base,
            speed: 1000.0,
            direction: Vector2::new(1.0, 1.0).normalized(),
            velocity: Vector2::ZERO,
            shape: rect,
            boop_player: None,
            hue: 0.0,
            glow_shader: shader_material,
        }
    }
    fn ready(&mut self) {
        godot_print!("MyPlayer ready called");
        let shape_clone = self.shape.clone();
        godot_print!("Shape cloned: {:?}", shape_clone);
        self.base_mut()
            .add_child(shape_clone.clone().upcast::<Node>());
        godot_print!("ColorRect added as child to MyPlayer");

        // Create a one-shot timer for delayed execution
        let mut timer = godot::classes::Timer::new_alloc();
        timer.set_one_shot(true);
        timer.set_autostart(true);
        timer.set_wait_time(0.1); // 100ms delay
        godot_print!("Timer created: {:?}", timer);
        timer.connect(
            "timeout".into(),
            self.base().callable("search_for_color_rect"),
        );
        godot_print!("Timer connected to search_for_color_rect");
        let timer_clone = timer.clone();
        self.base_mut()
            .add_child(timer_clone.upcast::<godot::classes::Timer>());
        godot_print!("Timer added as child to MyPlayer");
        timer.start();
        godot_print!("Timer started");

        self.base_mut().set_process(true);
        godot_print!("Processing enabled for MyPlayer");

        // Create AudioStreamPlayer2D
        let mut audio_player = AudioStreamPlayer2D::new_alloc();
        audio_player.set_autoplay(false);
        self.base_mut()
            .add_child(audio_player.clone().upcast::<Node>());
        self.boop_player = Some(audio_player);
    }
    fn process(&mut self, delta: f64) {
        // Update hue
        self.hue = (self.hue + 0.5 * delta as f32) % 1.0;

        // Convert HSV to RGB
        let mut color = Color::from_hsv(self.hue as f64, 1.0, 1.0);
        color.a = 0.25; // Increased alpha for a more visible glow

        // Update shader color parameter
        self.glow_shader.set_shader_parameter("glow_color".into(), Variant::from(color));

        // Update shader brightness parameter based on a value, here we can use a simple manual control or a constant value
        let brightness_value = (0.5 + 0.5 * (self.hue * std::f32::consts::PI).sin()).clamp(0.0, 2.0); // Example modulation
        self.glow_shader.set_shader_parameter("brightness".into(), Variant::from(brightness_value));

        // Print current shader parameters
        godot_print!(
            "Current glow_color parameter: {:?}, Current brightness: {:?}",
            self.glow_shader.get_shader_parameter("glow_color".into()),
            self.glow_shader.get_shader_parameter("brightness".into())
        );

        godot_print!("ColorRect visible: {}", self.shape.is_visible());
        godot_print!(
            "ColorRect global position: {:?}, size: {:?}",
            self.shape.get_global_position(),
            self.shape.get_size()
        );

        // Existing movement code
        self.move_shape(delta);
    }
}

#[godot_api]
impl MyPlayer {
    #[func]
    fn search_for_color_rect(&mut self) {
        godot_print!("search_for_color_rect called");
        godot_print!("Current node: {:?}", self.base().get_name());

        for child in self.base().get_children().iter_shared() {
            let child_name = child.get_name().to_string();
            godot_print!("Child: {}", child_name);

            if child_name.starts_with("@ColorRect@") {
                if let Ok(color_rect) = child.try_cast::<ColorRect>() {
                    godot_print!("ColorRect found: {:?}", color_rect);
                    return;
                }
            }
        }

        godot_print!("ColorRect not found in children");
    }

    fn move_shape(&mut self, delta: f64) {
        // Update velocity based on direction and speed
        self.velocity = self.direction * self.speed;

        // Update position
        let mut new_position = self.base().get_global_position() + (self.velocity * delta as f32);

        // Bounce off screen edges
        if let Some(viewport) = self.base().get_viewport() {
            let size = viewport.get_visible_rect().size;
            let shape_size = self.shape.get_size();

            if new_position.x < 0.0 || new_position.x + shape_size.x > size.x {
                self.direction.x *= -1.0;
                new_position.x = new_position.x.clamp(0.0, size.x - shape_size.x);
                godot_print!(
                    "MyPlayer bounced horizontally. New direction: {:?}",
                    self.direction
                );
                self.play_boop();
            }
            if new_position.y < 0.0 || new_position.y + shape_size.y > size.y {
                self.direction.y *= -1.0;
                new_position.y = new_position.y.clamp(0.0, size.y - shape_size.y);
                godot_print!(
                    "MyPlayer bounced vertically. New direction: {:?}",
                    self.direction
                );
                self.play_boop();
            }
        }

        // Set the new position
        self.base_mut().set_global_position(new_position);
    }

    fn play_boop(&mut self) {
        if let Some(audio_player) = &mut self.boop_player {
            let sample_rate = 44100.0;
            let duration = 0.1; // 100ms

            // Generate a random frequency between 220Hz (A3) and 880Hz (A5)
            let frequency = rand::thread_rng().gen_range(220.0..=880.0);

            let mut buffer = PackedVector2Array::new();
            for i in 0..(sample_rate * duration) as i32 {
                let t = i as f32 / sample_rate as f32;
                let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin();
                let envelope = 1.0 - (t / duration as f32);
                buffer.push(Vector2::new(sample * envelope, sample * envelope));
            }

            let mut audio_stream = AudioStreamGenerator::new_gd();
            audio_stream.set_mix_rate(sample_rate);
            audio_stream.set_buffer_length(duration);

            audio_player.set_stream(audio_stream.upcast::<AudioStream>());

            // Play the audio player first
            audio_player.play();

            // Now we can get the stream playback
            if let Some(playback) = audio_player.get_stream_playback() {
                if let Ok(mut generator_playback) =
                    playback.try_cast::<AudioStreamGeneratorPlayback>()
                {
                    // Convert PackedVector2Array to Vec<Vector2>
                    let buffer_vec = buffer.to_vec();
                    for frame in buffer_vec.iter() {
                        generator_playback.push_frame(*frame);
                    }
                }
            }

            godot_print!("Boop played with frequency: {:.2} Hz", frequency);
        }
    }
}

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
