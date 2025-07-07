use crate::Board;
use crate::Move;

pub fn change_board<'a>(board: &'a mut Board, player_input: &Move, board_has_been_checked: bool) -> &'a mut Board {

    if !board_has_been_checked {
        match player_input {
            Move::Left => board, 
            Move::Right => reverse_rows(board), 
            Move::Up => transpose(board), 
            Move::Down => {
                transpose(board);
                reverse_rows(board);
                board
            }
        }
    } else {
        match player_input {
            Move::Left => board, 
            Move::Right => reverse_rows(board), 
            Move::Up => transpose(board), 
            Move::Down => {
                reverse_rows(board);
                transpose(board);
                board
            }
        }        
    }

}

fn board_has_only_trailing_zeros(board: &Board) -> bool {

    let mut rows: Vec<u8> = Vec::new();
    for row in &board.board_game {
        let zero_position = zero_position(row);
        let non_zero_occurences = count_non_zero_value(row);
        if zero_position < non_zero_occurences {
            rows.push(0);
        } else {
            rows.push(1);
        }
    }

    trailing_zero_rows(&rows)

}

fn is_fusion_possible(board: &Board) -> bool {
        
    for row in board.board_game.iter() {
        let mut i = 0;
        while i + 1 < row.len() {
            if row[i] != 0 && row[i] == row[i + 1] {
                return true;
            } else {
                i += 1;
            }
        }
    }

    false
}

pub fn is_move_possible(board: &Board) -> bool {
    
    let trailing_zero = board_has_only_trailing_zeros(&board);
    let fusion_possible = is_fusion_possible(&board);

    if !trailing_zero || fusion_possible {
        true
    } else {
        false
    }
}

fn count_non_zero_value(array: &Vec<u32>) -> u32 {

    let mut occurences: u32 = 0;
    for element in array {
        if *element > 0 {
            occurences += 1;
        }
    }

    occurences

} 

fn zero_position(array: &Vec<u32>) -> u32 {

    array.iter().position(|&x|x==0).unwrap_or(4) as u32

} 

fn trailing_zero_rows(array: &Vec<u8>) -> bool {

    let result: u8 = array.into_iter().sum();
    if result < 4 {
        return false;
    }
    true
}

fn reverse_rows(board: &mut Board) -> &mut Board {
        for row in board.board_game.iter_mut() {
            row.reverse();
        }

        board
    }

fn transpose(board: &mut Board) -> &mut Board {
    let n = board.board_game.len();
    for i in 0..n {
        for j in i + 1..n {
            let tmp = board.board_game[i][j];
            board.board_game[i][j] = board.board_game[j][i];
            board.board_game[j][i] = tmp;
        }
    }

    board
}