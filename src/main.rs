mod tic_utils;
mod ttt_board;
mod ttt_game_state;
mod mcts;

use crate::tic_utils::TicState;
use crate::ttt_game_state::{TTTGameState, BoardLoc};
use crate::mcts::MCTS;
use std::usize;

fn get_player_choice(prompt: &str) -> bool {
    loop {
        println!("{} (y/n):", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}

fn get_number_input(prompt: &str, min: usize, max: usize) -> usize {
    loop {
        println!("{} ({}-{}):", prompt, min, max);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(n) if n >= min && n <= max => return n,
            _ => println!("Please enter a number between {} and {}", min, max),
        }
    }
}

fn get_game_mode() -> u8 {
    println!("Select game mode:");
    println!("1. Human vs Human");
    println!("2. Human vs AI");
    println!("3. AI vs AI");
    get_number_input("Enter mode", 1, 3) as u8
}

fn get_human_move(game: &TTTGameState) -> BoardLoc {
    loop {
        let row = get_number_input("Enter row", 0, 8);
        let col = get_number_input("Enter column", 0, 8);
        let loc = BoardLoc::new(row / 3, col / 3, row % 3, col % 3);
        
        if game.is_legal_move(loc) {
            return loc;
        }
        println!("Illegal move! Try again.");
    }
}

fn get_ai_move(game: &TTTGameState, simulations: usize, player: &str) -> BoardLoc {
    println!("\n{} is thinking...", player);
    let mut mcts = MCTS::new(game.clone());
    let loc = mcts.find_move_sim(simulations).unwrap();
    println!("{} made a move: ({}, {})", player, 
             loc.outer_row*3 + loc.inner_row, 
             loc.outer_col*3 + loc.inner_col);
    loc
}

fn intro_art() {
    println!(r#"-------------------------------------------------------------
 _   _ _____ _____ _____   ____            _        _    ___
| | | |_   _|_   _|_   _| |  _ \ _   _ ___| |_     / \  |_ _|
| | | | | |   | |   | |   | |_) | | | / __| __|   / _ \  | |
| |_| | | |   | |   | |   |  _ <| |_| \__ \ |_   / ___ \ | |
 \___/  |_|   |_|   |_|   |_| \_\\__,_|___/\__| /_/   \_\___|
 -------------------------------------------------------------
"#);
}

fn main() {
    intro_art();

    loop {
        let game_mode = get_game_mode();
        println!("");
        
        let (simulations_x, simulations_o) = match game_mode {
            1 => (0, 0),
            2 => {
                let sims = get_number_input("Enter AI thinking power in thousands of simulations", 1, 100) * 1000;
                println!("");
                (sims, sims)
            },
            3 => {
                let sims_x = get_number_input("Enter AI for X thinking power in thousands of simulations", 1, 100) * 1000;
                println!("");
                let sims_o = get_number_input("Enter AI for O thinking power in thousands of simulations", 1, 100) * 1000;
                println!("");
                (sims_x, sims_o)
            },
            _ => unreachable!()
        };
        
        let mut game = TTTGameState::new();
        let mut player_starts = false;

        match game_mode {
            1 => {
                println!("{}", game);
                while game.board_state == TicState::N {
                    let loc = get_human_move(&game);
                    game = game.make_move(loc);
                    println!("{}", game);
                    if game.board_state != TicState::N { break; }

                    let loc = get_human_move(&game);
                    game = game.make_move(loc);
                    println!("{}", game);
                }
            },
            2 => {
                player_starts = get_player_choice("Do you want to play as X (start first)?");
                println!("\nGame starting!");

                if player_starts {
                    println!("{}", game);
                    let loc = get_human_move(&game);
                    game = game.make_move(loc);
                    println!("{}", game);

                    let loc = get_ai_move(&game, simulations_x, "AI");
                    game = game.make_move(loc);
                    println!("{}", game);
                } else {
                    let loc = get_ai_move(&game, simulations_x, "AI");
                    game = game.make_move(loc);
                    println!("{}", game);
                }

                while game.board_state == TicState::N {
                    let loc = get_human_move(&game);
                    game = game.make_move(loc);
                    println!("{}", game);
                    if game.board_state != TicState::N { break; }

                    let loc = get_ai_move(&game, simulations_o, "AI");
                    game = game.make_move(loc);
                    println!("{}", game);
                }
            },
            3 => {
                while game.board_state == TicState::N {
                    let loc = get_ai_move(&game, simulations_x, "AI X");
                    game = game.make_move(loc);
                    println!("{}", game);
                    if game.board_state != TicState::N { break; }

                    let loc = get_ai_move(&game, simulations_o, "AI O");
                    game = game.make_move(loc);
                    println!("{}", game);
                }
            },
            _ => unreachable!(),
        }
        
        println!("\nGame Over!");
        match (game.board_state, game_mode) {
            (TicState::X, 1) => println!("{} win!", if player_starts { "You" } else { "AI" }),
            (TicState::O, 1) => println!("{} win!", if player_starts { "AI" } else { "You" }),
            (TicState::X, 2) => println!("Player X wins!"),
            (TicState::O, 2) => println!("Player O wins!"),
            (TicState::X, 3) => println!("AI X wins!"),
            (TicState::O, 3) => println!("AI O wins!"),
            (TicState::T, _) => println!("It's a tie!"),
            _ => unreachable!(),
        }

        if !get_player_choice("\nWould you like to play again?") {
            break;
        }
        println!("\n--------------------------\n");
    }
}

