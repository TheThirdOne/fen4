use crate::Position;
use std::convert::From;

impl From<&(usize, usize)> for Position {
    fn from(other: &(usize, usize)) -> Self {
        Self {
            row: other.0,
            col: other.1,
        }
    }
}
impl From<&Position> for (usize, usize) {
    fn from(other: &Position) -> Self {
        (other.row, other.col)
    }
}
