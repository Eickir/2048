use rand::Rng;
use rand::prelude::SliceRandom;
use crate::board::constants::{ROWS, COLS};


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

    pub fn move_tiles(&mut self) -> &mut Self {
            for row in self.board_game.iter_mut() {
                row.retain(|&x| x != 0);
            }
            self
        }

    pub fn sum_tiles(&mut self) -> &mut Self {
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

    pub fn spawn_tiles(&mut self) -> &mut Self {
        let all_zero_coordinates = self.zero_coordinates();
        let mut rng = rand::rng();
        let new_tile_value = if rng.random_bool(0.9) { 2 } else { 4 };
        let random_number = rng.random_range(..all_zero_coordinates.len());
        self.board_game[all_zero_coordinates[random_number].0]
            [all_zero_coordinates[random_number].1] = new_tile_value;

        self
    }

}