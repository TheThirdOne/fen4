#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Dead,
}

#[derive(Debug, PartialEq)]
pub enum Piece {
    Empty,
    Wall,
    Normal(Color, char),
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Empty
    }
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pub turn: Color,
    pub dead: [bool; 4],
    pub castling_king: [bool; 4],
    pub castling_queen: [bool; 4],
    pub points: [u32; 4],
    pub extra_options: String,
    pub board: [[Piece; 15]; 15],
}

