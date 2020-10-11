use std::num::ParseIntError;
use std::str::FromStr;

use crate::types::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceParseError {
    BadColor(char),
    BadSize(usize),
}

impl std::fmt::Display for PieceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PieceParseError::*;
        match self {
            BadColor(c) => write!(f,"Bad Color '{}'. Only 'r', 'b', 'y', 'g', and 'd' are valid colors.",c),
            BadSize(s) => write!(f,"Bad Size {}. Pieces like \"X\" or \"rK\" are the only valid types. Longer strings are generally invalid and empty string is purposly left out.",s),
        }
    }
}

impl std::error::Error for PieceParseError {}

impl FromStr for Piece {
    type Err = PieceParseError;
    fn from_str(small: &str) -> Result<Self, Self::Err> {
        use PieceParseError::*;
        if small == "X" {
            return Ok(Piece::Wall);
        }
        let mut iter = small.chars();
        let color = if let Some(c) = iter.next() {
            match c {
                'r' => Color::Red,
                'b' => Color::Blue,
                'y' => Color::Yellow,
                'g' => Color::Green,
                'd' => Color::Dead,
                _ => return Err(BadColor(c)),
            }
        } else {
            return Err(BadSize(0));
        };
        let shape = iter.next().ok_or(BadSize(1))?;
        if iter.next() != None {
            return Err(BadSize(iter.count() + 3));
        }
        Ok(Piece::Normal(color, shape))
    }
}
// Turns "0,1,1,0" into Some([false,true,true,false])
fn fen4_castle_helper(four_digits: &str) -> Option<[bool; 4]> {
    let mut tmp = [false; 4];
    let mut count = 0;
    for (pos, val) in four_digits.split(",").enumerate() {
        if pos > 3 {
            return None;
        }
        count = pos;
        if val != "0" && val != "1" {
            return None;
        }
        tmp[pos] = val != "0";
    }
    if count != 3 {
        return None;
    }
    Some(tmp)
}

// Turns "0,1,2,3" into Some([0,1,2,3])
fn fen4_point_helper(four_digits: &str) -> Option<[u32; 4]> {
    let mut tmp = [0; 4];
    for (pos, val) in four_digits.split(",").enumerate() {
        if pos > 3 {
            return None;
        }
        tmp[pos] = val.parse::<u32>().ok()?;
    }
    Some(tmp)
}

// Parses the entire metadata minus the last dash and makes a Board with that data filled in
fn parse_meta(meta_data: &str) -> Option<Board> {
    let mut meta_sections = meta_data.split("-");

    let color_str = meta_sections.next()?;
    let turn = match color_str {
        "R" => Color::Red,
        "B" => Color::Blue,
        "Y" => Color::Yellow,
        "G" => Color::Green,
        _ => return None,
    };

    let dead = fen4_castle_helper(meta_sections.next()?)?;
    let castling_king = fen4_castle_helper(meta_sections.next()?)?;
    let castling_queen = fen4_castle_helper(meta_sections.next()?)?;
    let points = fen4_point_helper(meta_sections.next()?)?;
    if "0" != meta_sections.next()? {
        return None;
    }
    let extra_options = if let Some(extra) = meta_sections.next() {
        extra.to_owned()
    } else {
        "".to_owned()
    };
    if None != meta_sections.next() {
        return None;
    }
    Some(Board {
        turn,
        dead,
        castling_king,
        castling_queen,
        points,
        extra_options,
        board: Default::default(),
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoardParseError {
    NoDash,
    BadMetaData,
    BadBoardSize(BoardSize, usize),
    EmptySegment(usize, usize),
    BadSegmentNumber(usize, usize, ParseIntError),
    BadSegmentPiece(usize, usize, PieceParseError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardSize {
    TooManyColumns,
    TooFewColumns,
    TooManyRows,
    TooFewRows,
}

impl std::fmt::Display for BoardParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BoardSize::*;
        use BoardParseError::*;
        match self {
            NoDash => write!(f,"No '-' was found in the fen. Fen4's should start with metadata about castling, turn, and more."),
            BadMetaData => write!(f,"Something went wrong with metadata parsing"),
            BadBoardSize(bs,row) => match bs {
                TooManyColumns => write!(f,"Too many columns in row {}.",row),
                TooFewColumns=> write!(f,"Not enough columns in row {}.", row),
                TooManyRows=> write!(f,"Too many rows overall. Make sure there is not a leading or trailing '/'"),
                TooFewRows=> write!(f,"{} too few rows overall. Make sure there is not a missing row.",row),
            },
            EmptySegment(row,col)=> write!(f,"Segment at ({},{}) is empty which is not valid.",row,col), 
            BadSegmentNumber(row,col,int_error) => write!(f,"Segment at ({},{}) starts with a digit but cannot be parsed as a number because of {}",row,col,int_error),
            BadSegmentPiece(row,col,piece_error) => write!(f,"Segment at ({},{}) cannot be parsed as piece because of {}",row,col,piece_error),
        }
    }
}

impl std::error::Error for BoardParseError {}

impl FromStr for Board {
    type Err = BoardParseError;
    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        use BoardSize::*;
        use BoardParseError::*;
        let last_dash = if let Some(tmp) = fen.rfind("-") {
            tmp
        } else {
            return Err(NoDash);
        };

        let meta_data = &fen[..last_dash];
        let board = &fen[last_dash + 1..];

        let mut board_base = parse_meta(meta_data).ok_or(BadMetaData)?;
        let mut row = 14;
        // There is a lot of error handling obscuring the fact that this is actually really simple
        // We keep track of where we are, starting at (14,0) and move to the right as we fill in cells. Finishing a row decreases our row by 1 and resets our column.
        // Cells can be either a number that shifts us thta much to the right or a Piece which we put on the Board and shift by 1.
        for line in board.split("/") {
            if row == 0 {
                return Err(BadBoardSize(TooManyRows, row));
            }
            row -= 1;
            let mut col = 0;
            for segment in line.split(",") {
                if col >= 14 {
                    return Err(BadBoardSize(TooManyColumns, row));
                }
                let trimmed = segment.trim();
                if trimmed
                    .chars()
                    .next()
                    .ok_or(EmptySegment(row, col))?
                    .is_digit(10)
                {
                    let spaces = trimmed
                        .parse::<usize>()
                        .map_err(|e| BadSegmentNumber(row, col, e))?;
                    col += spaces;
                } else {
                    board_base.board[row][col] = trimmed
                        .parse::<Piece>()
                        .map_err(|e| BadSegmentPiece(row, col, e))?;
                    col += 1;
                }
            }
            if col != 14 {
                return Err(BadBoardSize(TooFewColumns, row));
            }
        }
        if row != 0 {
            return Err(BadBoardSize(TooFewRows, row));
        }
        Ok(board_base)
    }
}
