use rand::Rng;
use strum::IntoEnumIterator;
use rand::prelude::SliceRandom;
use crate::utils::{zero_position, count_non_zero_value, trailing_zero_rows};
use crate::board::constants::{ROWS, COLS};
use crate::Move;


#[derive(Debug)]
pub struct Board {
    pub board_game: Vec<Vec<u32>>,
    pub is_game_over: bool
}

// Fonctions associées
impl Board {
    
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let mut flat: Vec<u32> = vec![0; ROWS * COLS];

        flat[0] = if rng.random_bool(0.9) { 2 } else { 4 };
        flat[1] = if rng.random_bool(0.9) { 2 } else { 4 };

        flat.shuffle(&mut rng);

        let board: Vec<Vec<u32>> = flat.chunks(ROWS).map(|chunk| chunk.to_vec()).collect();

        Self {
            board_game: board,
            is_game_over: false
        }
    }

}

// Méthodes
impl Board {

    pub fn display(&self) {
        println!("+----+----+----+----+");
        for row in &self.board_game {
            for val in row {
                if *val == 0 {
                    // case vide
                    print!("|    ");
                } else {
                    // convertit en string
                    let s = val.to_string();
                    let width = 4;
                    let padding = width - s.len();
                    let left = padding / 2;
                    let right = padding - left;
                    print!("|{}{}{}", " ".repeat(left), s, " ".repeat(right));
                }
            }
            println!("|");
            println!("+----+----+----+----+");
        }
    }

    fn reverse_rows(&mut self) -> &mut Self {
            for row in self.board_game.iter_mut() {
                row.reverse();
            }

            self
        }

    fn transpose(&mut self) -> &mut Self {
        let n = self.board_game.len();
        for i in 0..n {
            for j in i + 1..n {
                let tmp = self.board_game[i][j];
                self.board_game[i][j] = self.board_game[j][i];
                self.board_game[j][i] = tmp;
            }
        }

        self
    }

    fn change_board<'a>(&'a mut self, player_input: &Move, board_has_been_checked: bool) -> &'a mut Board {

        if !board_has_been_checked {
            match player_input {
                Move::Left => self, 
                Move::Right => self.reverse_rows(), 
                Move::Up => self.transpose(), 
                Move::Down => {
                    self.transpose();
                    self.reverse_rows();
                    self
                }
            }
        } else {
            match player_input {
                Move::Left => self, 
                Move::Right => self.reverse_rows(), 
                Move::Up => self.transpose(), 
                Move::Down => {
                    self.reverse_rows();
                    self.transpose();
                    self
                }
            }        
        }

    }

    fn board_has_only_trailing_zeros(&self) -> bool {

        let mut rows: Vec<u8> = Vec::new();
        for row in &self.board_game {
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

    fn is_fusion_possible(&self) -> bool {
            
        for row in self.board_game.iter() {
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

    fn is_move_possible(&self) -> bool {
        
        let trailing_zero = self.board_has_only_trailing_zeros();
        let fusion_possible = self.is_fusion_possible();

        if !trailing_zero || fusion_possible {
            true
        } else {
            false
        }
    }

    fn is_game_over(&mut self) {
        let any_possible = Move::iter().any(|direction| {
            self.change_board(&direction, false);
            let possible = self.is_move_possible();
            self.change_board(&direction, true);
            possible
        });

        self.is_game_over = !any_possible;

    }

    fn zero_coordinates(&self) -> Vec<(usize, usize)> {
        let mut zero_coordinates: Vec<(usize, usize)> = Vec::new();
        for row in self.board_game.iter().enumerate() {
            for element in row.1.iter().enumerate() {
                if *element.1 == 0 {
                    zero_coordinates.push((row.0, element.0));
                }
            }
        }
        zero_coordinates
    }

    fn move_tiles(&mut self) -> &mut Self {
            for row in self.board_game.iter_mut() {
                row.retain(|&x| x != 0);
            }
            self
        }

    fn sum_tiles(&mut self) -> &mut Self {
        for row in self.board_game.iter_mut() {
            let mut i = 0;
            while i + 1 < row.len() {
                if row[i] != 0 && row[i] == row[i + 1] {
                    row[i] *= 2;
                    row[i + 1] = 0;
                    i += 2; // skip next to avoid double merge
                } else {
                    i += 1;
                }
            }
            row.retain(|&x| x != 0);

            while row.len() < 4 {
                row.push(0);
            }
        }

        self
    }

    fn spawn_tiles(&mut self) -> &mut Self {
        let all_zero_coordinates = self.zero_coordinates();
        let mut rng = rand::rng();
        let new_tile_value = if rng.random_bool(0.9) { 2 } else { 4 };
        let random_number = rng.random_range(..all_zero_coordinates.len());
        self.board_game[all_zero_coordinates[random_number].0]
            [all_zero_coordinates[random_number].1] = new_tile_value;

        self
    }

    pub fn make_move(&mut self, player_input: &Move) {

            self.change_board(player_input, false);
            if self.is_move_possible() {
                self.move_tiles()
                    .sum_tiles();
                self.change_board(player_input, true);
                self.spawn_tiles();
            } else {
                self.change_board(player_input, true);
            }

            self.is_game_over();
            
        }

}