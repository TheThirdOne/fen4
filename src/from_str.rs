use std::convert::TryInto;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::types::*;

use thiserror::Error;

/// Enum to store all ways [`Position`] can fail to parse
#[derive(Error, PartialEq, Eq, Clone, Debug)]
pub enum PositionParseError {
    #[error("'{0}' is not a valid column. Valid columns are 'a'-'n'")]
    ColumnInvalid(char),
    #[error("'{0}' is not a valid row. Valid rows are 1-14")]
    RowInvalid(usize),
    #[error("All positions are between 2-3 characters long; This position is {0} characters long")]
    BadSize(usize),
    #[error("Row failed to parse as a number because {0}")]
    RowNotNumber(#[from] std::num::ParseIntError),
}

impl FromStr for Position {
    type Err = PositionParseError;
    fn from_str(small: &str) -> Result<Self, Self::Err> {
        let len = small.len();
        if len < 2 || len > 3 {
            return Err(PositionParseError::BadSize(len));
        }
        let mut iter = small.chars();
        let column_letter = iter.next().unwrap(); // Guaranteed to succeed because of `if len` above
        if column_letter > 'n' || column_letter < 'a' {
            return Err(PositionParseError::ColumnInvalid(column_letter));
        }

        let a: u32 = 'a'.into();
        let mut column_num: u32 = column_letter.into();
        column_num -= a;
        let col: usize = column_num.try_into().unwrap(); // The `if column_letter` earlier should guarantee this succeeds

        let number_str = iter.as_str();
        let row = number_str.parse::<usize>()?;
        if row == 0 || row > 14 {
            return Err(PositionParseError::RowInvalid(row));
        }
        Ok(Position { col, row: row - 1 })
    }
}

/// Enum to store all ways [`Piece`] can fail to parse
#[derive(Error, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceParseError {
    #[error("Bad Color '{0}'. Only 'r', 'b', 'y', 'g', and 'd' are valid colors.")]
    BadColor(char),
    #[error("Bad Size {0}. Pieces like \"X\" , \"rK\" or \"drK\" are the only valid types. Longer strings are generally invalid and empty string is purposly left out.")]
    BadSize(usize),
}

impl FromStr for Piece {
    type Err = PieceParseError;
    fn from_str(small: &str) -> Result<Self, Self::Err> {
        use PieceParseError::*;
        if small == "X" {
            return Ok(Piece::Wall);
        }
        let mut iter = small.chars().peekable();
        let color = if let Some(c) = iter.next() {
            use Color::*;
            use TurnColor::*;
            match c {
                'r' => Turn(Red),
                'b' => Turn(Blue),
                'y' => Turn(Yellow),
                'g' => Turn(Green),
                'd' => {
                    let tmp = match iter.peek() {
                        Some('r') => Dead(Some(Red)),
                        Some('b') => Dead(Some(Blue)),
                        Some('y') => Dead(Some(Yellow)),
                        Some('g') => Dead(Some(Green)),
                        _ => Dead(None),
                    };
                    if let Dead(Some(_)) = tmp {
                        iter.next();
                    }
                    tmp
                }
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
fn fen4_castle_helper(four_digits: &str) -> Result<[bool; 4], MetaDataParseError> {
    use MetaDataParseError::BadComma;
    let mut tmp = [false; 4];
    let mut count = 0;
    for (pos, val) in four_digits.split(",").enumerate() {
        if pos > 3 {
            return Err(BadComma);
        }
        count = pos;
        if val != "0" && val != "1" {
            return Err(BadComma);
        }
        tmp[pos] = val != "0";
    }
    if count != 3 {
        return Err(BadComma);
    }
    Ok(tmp)
}

// Turns "0,1,2,3" into Some([0,1,2,3])
fn fen4_point_helper(four_digits: &str) -> Result<[u16; 4], MetaDataParseError> {
    let mut tmp = [0; 4];
    for (pos, val) in four_digits.split(",").enumerate() {
        if pos > 3 {
            return Err(MetaDataParseError::BadComma);
        }
        tmp[pos] = val.parse::<u16>()?;
    }
    Ok(tmp)
}

// Parses the entire metadata minus the last dash and makes a Board with that data filled in
fn parse_meta(meta_data: &str) -> Result<Board, MetaDataParseError> {
    use MetaDataParseError::*;
    let mut meta_sections = meta_data.split("-");

    let color_str = meta_sections.next().ok_or(BadDash)?;
    let turn = match color_str {
        "R" => TurnColor::Red,
        "B" => TurnColor::Blue,
        "Y" => TurnColor::Yellow,
        "G" => TurnColor::Green,
        _ => return Err(BadColor),
    };

    let dead = fen4_castle_helper(meta_sections.next().ok_or(BadDash)?)?;
    let castling_king = fen4_castle_helper(meta_sections.next().ok_or(BadDash)?)?;
    let castling_queen = fen4_castle_helper(meta_sections.next().ok_or(BadDash)?)?;
    let points = fen4_point_helper(meta_sections.next().ok_or(BadDash)?)?;
    let draw_ply = meta_sections.next().ok_or(BadDash)?.parse::<usize>()?;
    let extra_options = if let Some(extra) = meta_sections.next() {
        extra.parse()?
    } else {
        Extra::default()
    };
    if None != meta_sections.next() {
        return Err(BadDash);
    }
    Ok(Board {
        turn,
        dead,
        castling_king,
        castling_queen,
        points,
        draw_ply,
        extra_options,
        board: Default::default(),
    })
}

fn split_array(array: &str) -> Result<[&str; 4], MetaDataParseError> {
    use MetaDataParseError::*;
    let trimmed = array
        .strip_prefix('(')
        .ok_or(BadParen)?
        .strip_suffix(')')
        .ok_or(BadParen)?;
    let mut out = [""; 4];
    let mut i = 0;
    for part in trimmed.split(',') {
        if i > 3 {
            return Err(BadComma);
        }
        out[i] = part;
        i += 1;
    }
    if i == 4 {
        Ok(out)
    } else {
        Err(BadComma)
    }
}

/// Enum to store all ways [`Extra`] can fail to parse
#[derive(Error, Clone, PartialEq, Eq, Debug)]
pub enum MetaDataParseError {
    #[error("There should be either 6 or 7 dashes in the metadata")]
    BadDash,
    #[error("There should be only two curly braces and they should only be present if there are tagged values")]
    BadCurly,
    #[error("All tags should be surrounded by single quotes")]
    BadQuote,
    #[error("Colons should separate tag keys and values")]
    BadColon,
    #[error("Parens should be balanced and only occur within ")]
    BadParen,
    #[error("Only 'R', 'G', 'Y', or 'B' are valid turn colors")]
    BadColor,
    #[error("Commas should separate arrays and extra tags")]
    BadComma,
    #[error("Some tag occurred twice")]
    RepeatedTag,
    #[error("Only true and false are valid boolean values")]
    BadBoolean,
    #[error("Tag '{0}' is not expected")]
    UnknownTag(String),
    #[error("Somewhere a Position was expected it failed to parse because of {0}")]
    BadPosition(#[from] PositionParseError),
    #[error("Somewhere a number was expected it failed to parse becauses of {0}")]
    BadNumber(#[from] ParseIntError),
}

impl FromStr for Extra {
    type Err = MetaDataParseError;
    fn from_str(tagged: &str) -> Result<Self, Self::Err> {
        use MetaDataParseError::*;
        let mut current = tagged.strip_prefix('{').ok_or(BadCurly)?;
        let mut extras: Self = Default::default();
        if current == "}" || !current.ends_with('}') {
            return Err(BadCurly);
        }
        while let Some(separator) = current.find(':') {
            let (label, rest) = current.split_at(separator);
            current = rest.split_at(1).1;
            let label_trimmed = label
                .strip_prefix('\'')
                .ok_or(BadQuote)?
                .strip_suffix('\'')
                .ok_or(BadQuote)?;
            let value_end = if current.starts_with('(') {
                current.find(')').ok_or(BadParen)? + 1
            } else {
                current.find(|c| c == ',' || c == '}').unwrap()
            };
            let (value, tmp) = current.split_at(value_end);
            current = tmp;
            match label_trimmed {
                "enPassant" => {
                    let array = split_array(value)?;
                    let mut i = 0;
                    for pair in &array {
                        if extras.enpassant[i] != None {
                            return Err(RepeatedTag);
                        }
                        let trimmed = pair
                            .strip_prefix('\'')
                            .ok_or(BadQuote)?
                            .strip_suffix('\'')
                            .ok_or(BadQuote)?;
                        if trimmed != "" {
                            let mut split = trimmed.split(':');
                            let first = split.next().ok_or(BadColon)?;
                            let second = split.next().ok_or(BadColon)?;
                            if split.next() != None {
                                return Err(BadColon);
                            }
                            extras.enpassant[i] =
                                Some((first.parse::<Position>()?, second.parse::<Position>()?));
                        }
                        i += 1;
                    }
                }
                "royal" | "kingSquares" => {
                    let array = split_array(value)?;
                    let mut i = 0;
                    for position in &array {
                        if extras.royal[i] != None {
                            return Err(RepeatedTag);
                        }
                        let trimmed = position
                            .strip_prefix('\'')
                            .ok_or(BadQuote)?
                            .strip_suffix('\'')
                            .ok_or(BadQuote)?;
                        if trimmed != "" {
                            extras.royal[i] = Some(trimmed.parse::<Position>()?);
                        }
                        i += 1;
                    }
                }
                "pawnsBaseRank" | "uniquify" => {
                    let number = value.parse::<usize>()?;
                    if label_trimmed == "uniquify" {
                        extras.uniquify = number;
                    } else {
                        extras.pawnbaserank = number;
                    }
                }
                "resigned" | "flagged" | "stalemated" | "zombieImmune" => {
                    let array = split_array(value)?;
                    let mut i = 0;
                    let output = if label_trimmed == "flagged" {
                        &mut extras.flagged
                    } else if label_trimmed == "resigned" {
                        &mut extras.resigned
                    } else if label_trimmed == "stalemated" {
                        &mut extras.stalemated
                    } else {
                        &mut extras.zombie_immune
                    };
                    for truth in &array {
                        if output[i] {
                            return Err(RepeatedTag);
                        }
                        output[i] = match *truth {
                            "true" => true,
                            "false" => false,
                            "null" => false,
                            _ => return Err(BadBoolean),
                        };
                        i += 1;
                    }
                }
                "std2pc" => {
                    extras.std2pc = match value {
                        "true" => true,
                        "false" => false,
                        _ => return Err(BadBoolean),
                    };
                }
                "lives" => {
                    let array = split_array(value)?;
                    let mut i = 0;
                    if extras.lives != None {
                        return Err(RepeatedTag);
                    }
                    let mut tmp = [0; 4];
                    for life in &array {
                        tmp[i] = life.parse::<usize>()?;
                        i += 1;
                    }
                    extras.lives = Some(tmp);
                }
                "zombieType" => {
                    let array = split_array(value)?;
                    let mut i = 0;
                    for pair in &array {
                        if extras.zombie_type[i] != "" {
                            return Err(RepeatedTag);
                        }
                        let trimmed = pair
                            .strip_prefix('\'')
                            .ok_or(BadQuote)?
                            .strip_suffix('\'')
                            .ok_or(BadQuote)?;
                        extras.zombie_type[i] = trimmed.into();
                        i += 1;
                    }
                }
                "gameOver" => {
                    let trimmed = value
                        .strip_prefix('\'')
                        .ok_or(BadQuote)?
                        .strip_suffix('\'')
                        .ok_or(BadQuote)?;
                    extras.game_over = trimmed.into();
                }
                s => {
                    return Err(UnknownTag(String::from(s)));
                }
            }
            if current == "}" {
                break;
            }
            current = current.strip_prefix(',').ok_or(BadComma)?;
        }
        Ok(extras)
    }
}

/// Enum to store all ways [`Board`] can fail to parse
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoardParseError {
    NoDash,
    BadMetaData(MetaDataParseError),
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
        use BoardParseError::*;
        use BoardSize::*;
        match self {
            NoDash => write!(f,"No '-' was found in the fen. Fen4's should start with metadata about castling, turn, and more."),
            BadMetaData(me) => write!(f,"Something went wrong with metadata parsing: {}",me),
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
        use BoardParseError::*;
        use BoardSize::*;
        let last_dash = if let Some(tmp) = fen.rfind("-") {
            tmp
        } else {
            return Err(NoDash);
        };

        let meta_data = &fen[..last_dash];
        let board = &fen[last_dash + 1..];

        let mut board_base = parse_meta(meta_data).map_err(|e| BadMetaData(e))?;
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
