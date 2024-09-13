use deckbuilder::{tutorial::TutorialState, CoreGameState};
use std::io;

fn display_full_log(game: &CoreGameState) {
    let log = game.get_log();
    if log.is_empty() {
        println!("No game log entries.");
    } else {
        println!("Full game log:");
        for entry in log.iter() {
            println!("  {}", entry);
        }
    }
    println!();
}
fn main() {
    let mut game = CoreGameState::new();

    println!(
        "Enemy: Health = {}, Attack = {}",
        game.enemy.health, game.enemy.attack
    );

    // Main game loop
    loop {
        // Increment turn counter
        game.increment_turn();

        println!("Turn {}", game.get_turn_count());

        // Handle turn-based events
        game.handle_turn_events();

        // Refresh player's mana at the start of each turn
        game.player.restore_mana();

        // Check if hand is empty and draw cards if necessary
        if game.player.hand.is_empty() {
            println!("Your hand is empty. Drawing new cards...");
            for _ in 0..5 {
                if let Some(card) = game.draw_card() {
                    println!(
                        "Drew: {} (Attack: {}, Defense: {}, Mana Cost: {})",
                        card.name, card.attack, card.defense, card.mana_cost
                    );
                } else {
                    println!("No more cards to draw!");
                    break;
                }
            }
        }

        // Display player's hand and mana
        println!("Your hand:");
        for (i, card) in game.player.hand.iter().enumerate() {
            println!(
                "{}. {} (Attack: {}, Defense: {}, Mana Cost: {})",
                i + 1,
                card.name,
                card.attack,
                card.defense,
                card.mana_cost
            );
        }
        println!(
            "Current Mana: {}/{}",
            game.player.current_mana, game.player.max_mana
        );

        // Player's turn
        println!(
            "Enter the number of the card you want to play, 'c' to add a comment, 't' for tutorial, or 'q' to quit:"
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input {
            "q" => break,
            "c" => {
                println!("Enter your comment:");
                let mut comment = String::new();
                io::stdin()
                    .read_line(&mut comment)
                    .expect("Failed to read comment");
                game.add_user_comment(comment.trim().to_string());
                println!("Comment added to the log.");
                continue;
            }
            "t" => {
                let mut tutorial = TutorialState::new();
                loop {
                    let instruction = tutorial.get_current_instruction();
                    println!("{}", instruction);
                    let mut tutorial_input = String::new();
                    io::stdin()
                        .read_line(&mut tutorial_input)
                        .expect("Failed to read tutorial input");
                    let response = tutorial.handle_input(tutorial_input.trim());
                    if !response.is_empty() {
                        println!("{}", response);
                    }
                    if tutorial.is_complete() {
                        break;
                    }
                }
                // Update the game state with the tutorial's core game
                game = tutorial.core_game;
                println!("Tutorial completed. Returning to the main game.");
                println!(
                    "Enemy: Health = {}, Attack = {}",
                    game.enemy.health, game.enemy.attack
                );
                continue;
            }
            _ => {
                // Process player's move
                if let Ok(index) = input.parse::<usize>() {
                    if index > 0 && index <= game.get_hand().len() {
                        let result = game.play_card(index as i32 - 1);
                        println!("{}", result);
                    } else {
                        println!("Invalid card number. Please try again.");
                        continue;
                    }
                } else {
                    println!(
                        "Invalid input. Please enter a number, 'c' to comment, 't' for tutorial, or 'q' to quit."
                    );
                    continue;
                }
            }
        }

        // Display updated status
        println!("Player Health: {}", game.get_player_health());
        println!("Enemy Health: {}", game.get_enemy_health());

        // Enemy's turn
        game.enemy_turn();
        println!("Player Health: {}", game.get_player_health());

        // Display full game log
        display_full_log(&game);

        // Check for win/lose conditions
        if let Some(game_over_message) = game.check_game_over() {
            println!("{}", game_over_message);
            break;
        }
    }
}
