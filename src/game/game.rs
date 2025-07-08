use crate::Player;
use crate::Board;

#[derive(Debug)]
pub struct Game<'a> {
    
    pub player: &'a Player,
    pub board: Board

}

impl<'a> Game<'a> {

    pub fn new(player: &'a Player) -> Self {

        Self {

            player, 
            board: Board::new()

        }

    }


}