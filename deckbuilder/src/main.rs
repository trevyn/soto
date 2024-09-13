use deckbuilder::{CoreGameState, TutorialState};
use std::io;

fn display_recent_logs(game: &CoreGameState, num_entries: usize) {
    let log = game.get_log();
    if log.is_empty() {
        println!("No recent game log entries.");
    } else {
        println!("Recent game log:");
        let entries: Vec<_> = log.iter().take(num_entries).collect();
        for entry in entries.iter() {
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
        // Display player's hand
        println!("Your hand:");
        for (i, card) in game.player.hand.iter().enumerate() {
            println!(
                "{}. {} (Attack: {}, Defense: {})",
                i + 1,
                card.name,
                card.attack,
                card.defense
            );
        }

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

        // Display recent log entries
        display_recent_logs(&game, 5);

        // Check for win/lose conditions
        if let Some(game_over_message) = game.check_game_over() {
            println!("{}", game_over_message);
            break;
        }
    }
}
