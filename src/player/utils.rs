use crate::utils::change_board;
use crate::utils::is_move_possible;
use crate::Move;
use crate::Board;
use strum::IntoEnumIterator;

pub fn is_game_over(board: &mut Board) {
    let any_possible = Move::iter().any(|direction| {
        change_board(board, &direction, false);
        let possible = is_move_possible(board);
        change_board(board, &direction, true);
        possible
    });

    board.is_game_over = !any_possible;

}