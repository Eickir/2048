use rand::Rng;
use rand::prelude::SliceRandom;
use crate::board::constants::{ROWS, COLS};


#[derive(Debug)]
pub struct Board {
    board_game: Vec<Vec<u32>>
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

}