use crate::{Color, Position, TurnColor};
use std::convert::From;

impl From<(usize, usize)> for Position {
    fn from(other: (usize, usize)) -> Self {
        (&other).into()
    }
}

impl From<&(usize, usize)> for Position {
    fn from(other: &(usize, usize)) -> Self {
        Self {
            row: other.0,
            col: other.1,
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(other: Position) -> Self {
        (other.row, other.col)
    }
}
impl From<&Position> for (usize, usize) {
    fn from(other: &Position) -> Self {
        (other.row, other.col)
    }
}

impl From<TurnColor> for usize {
    fn from(other: TurnColor) -> Self {
        (&other).into()
    }
}
impl From<&TurnColor> for usize {
    fn from(other: &TurnColor) -> Self {
        use TurnColor::*;
        match other {
            Red => 0,
            Blue => 1,
            Yellow => 2,
            Green => 3,
        }
    }
}
impl From<usize> for Color {
    fn from(index: usize) -> Self {
        use TurnColor::*;
        Color::Turn(match index {
            0 => Red,
            1 => Blue,
            2 => Yellow,
            3 => Green,
            _ => return Color::Dead(None),
        })
    }
}
