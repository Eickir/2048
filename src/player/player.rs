use crate::player::utils::is_game_over;
use crate::Move;
use crate::Board;
use crate::utils::*;

#[derive(Debug)]
pub struct Player {
    username: String
}

// fonction associées 
impl Player {

    pub fn new(username: String) -> Self {

        Self {
            username
        }
    }
}

// methodes 
impl Player {

    pub fn make_move(&self, board: &mut Board, player_input: &Move) {

        change_board(board, player_input, false);
        if is_move_possible(board) {
            board.move_tiles()
                .sum_tiles();
            change_board(board, player_input, true);
            board.spawn_tiles();
        } else {
            change_board(board, player_input, true);
        }

        is_game_over(board);
        
    }

    
}