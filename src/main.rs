mod board;
use crate::board::board::Board;
use crate::board::utils;
mod moves;
use std::io;
use crate::moves::moves::Move;
use std::io::Write;

fn main() {
    
    let mut my_board: Board = Board::new();

    while !my_board.is_game_over {
        
        my_board.display(); 
        println!("Choose your move:");
        println!("1 = Left");
        println!("2 = Right");
        println!("3 = Up");
        println!("4 = Down");

        print!("Enter a number (1-4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        let player_move = match trimmed {
            "1" => Move::Left,
            "2" => Move::Right,
            "3" => Move::Up,
            "4" => Move::Down,
            _ => {
                println!("Invalid input, please enter a number between 1 and 4.");
                continue;
            }
        };

        my_board.make_move(&player_move);
        
    }
    println!("You lost!");

}
