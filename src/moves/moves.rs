use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Move {
    Left, 
    Right,
    Up, 
    Down
}