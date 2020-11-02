/// Position on the board e.g. a4
///
/// Both row and col should be in the range 0-13.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

/// Simple enum for colors / players.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TurnColor {
    Red,
    Blue,
    Yellow,
    Green,
}

/// Color modifier for pieces
///
/// Includes normal pieces, dead pieces, and dead pieces that also track which player they came from.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Turn(TurnColor),
    Dead(Option<TurnColor>),
}

/// Simple representation of pieces that allows all types of fairy pieces
/// The trick is to just use the character that the notation uses to represent that piece.
///
/// Walls and empty cells are special, but everything else has a color as well.
#[derive(Debug, PartialEq, Eq, Clone)]
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
/// The board representation of a 4 player chess game.
/// Board can be converted to and from a String in the fen4 format
///     
/// The fen4 file format is very similar to the [fen](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
/// format for normal chess, but it uses a larger board and uses prefixes for the
/// color of pieces instead of capitalization.
///
/// It also includes the metadata about castling rights at the beginning rather
/// than the end and has some differences in what metadata is stored.
///
/// The default position (with no whitespace) is:
/// ```text
///     R-0,0,0,0-1,1,1,1-1,1,1,1-0,0,0,0-0-
///     3,yR,yN,yB,yK,yQ,yB,yN,yR,3/
///     3,yP,yP,yP,yP,yP,yP,yP,yP,3/
///     14/
///     bR,bP,10,gP,gR/
///     bN,bP,10,gP,gN/
///     bB,bP,10,gP,gB/
///     bK,bP,10,gP,gQ/
///     bQ,bP,10,gP,gK/
///     bB,bP,10,gP,gB/
///     bN,bP,10,gP,gN/
///     bR,bP,10,gP,gR/
///     14/
///     3,rP,rP,rP,rP,rP,rP,rP,rP,3/
///     3,rR,rN,rB,rQ,rK,rB,rN,rR,3
/// ```
/// This format is shared for the Free-for-all and Teams mode so the dead and point
/// metadata is useless in Teams, but is there because the format is shared with the FFA mode.
///
/// The metadata format is parsed into the struct as shown here:
/// ```text
///     v turn    v king          v points  v extra
///     R-1,0,0,0-1,1,1,0-0,1,0,0-1,2,3,4-0-{'lifes':(2,2,2,2)}-
///       ^ dead          ^ queen         ^ ply since last pawn move or capture
/// ```
/// It starts with a Color, followed by 4 integer arrays of length 4, followed by
/// 0, followed by an optional extra data section. Each of these is separated by a
/// '-'. All of the arrays are information about the players with the leftmost data
/// about Red and proceding clockwise.
#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    pub turn: TurnColor,
    pub dead: [bool; 4],
    pub castling_king: [bool; 4],
    pub castling_queen: [bool; 4],
    pub points: [u32; 4],
    pub draw_ply: usize,
    pub extra_options: Extra,
    pub board: [[Piece; 14]; 14],
}

const DEFAULT_FEN: &str = "R-0,0,0,0-1,1,1,1-1,1,1,1-0,0,0,0-0-
3,yR,yN,yB,yK,yQ,yB,yN,yR,3/
3,yP,yP,yP,yP,yP,yP,yP,yP,3/
14/
bR,bP,10,gP,gR/
bN,bP,10,gP,gN/
bB,bP,10,gP,gB/
bK,bP,10,gP,gQ/
bQ,bP,10,gP,gK/
bB,bP,10,gP,gB/
bN,bP,10,gP,gN/
bR,bP,10,gP,gR/
14/
3,rP,rP,rP,rP,rP,rP,rP,rP,3/
3,rR,rN,rB,rQ,rK,rB,rN,rR,3";

impl Default for Board {
    fn default() -> Self {
        DEFAULT_FEN.parse::<Self>().unwrap()
    }
}

/// Additional options in the FEN4 format stored as a list of key value pairs.
///
/// In addition to the always present options in a FEN4, there are also options
/// that are only present if rule variants are enabled. Notable examples would
/// be "en passant" (because its not considered standard) and "N-Check"
///
/// The format uses a series of labeled elements separated by commas. The highlevel
/// structure looks like `{'label':value,'label2':value2}`.
///
/// Values seem to be able to take several types. But the ones that are confirmed are:
///   - Strings `'string_value'`
///   - Numbers `65900`
///   - Booleans `true` and `false`
///   - Arrays `(valueRed, valueBlue, valueYellow, valueGreen)`
///
/// Examples of known labels used are:
///   - `'enPassant':('j3:j4','','','')`
///     - The first position is where a pawn can capture and the second is where the passing pawn should be removed in the event of a capture.
///     - This is neccessary notation because some types of fairy pawns move diagonally. Without the extra information it could be ambiguous.
///   - `'lives':(9,6,4,0)`
///     - Indicates number of lives left for the N-check variant
///   - `'pawnsBaseRank':8`
///     - The rank on which pawns can jump forward 2 square (default:2)
///     - a value of `0` indicates pawns never move more than 1
///   - `'royal':('a4','b5','c6','d7')`
///     - Used to indicate which piece should be considered the king for purposes of checks
///     - Neccessary in cases of multiple kings (of the same color) or for making a different piece act as the leader
///     - This seems to have previously been called 'kingSquares'
///   - `'uniquify':94403`
///     - It is not clear what this actually does
///   - `'resigned':(true,true,false,false)` and `'flagged':(false,true,false,false)`
///     - Both seem to be neccesssary for the "DeadKingWalking" feature.
///
/// The labels have a preferred order. For the known lables this is royal/kingSquares,
/// lives, resigned, flagged, enPassant, pawnBaseRank. uniquify is after lives,
/// but otherwise does not have a clear position.
///
/// gameOver is an additional option, but only seems to appear in internal messages.
/// It would fit between flagged and enPassant in terms of preferred order and stores
/// the message that shows up at the end of the game.
#[derive(Debug, PartialEq, Clone)]
pub struct Extra {
    pub royal: [Option<Position>; 4],
    pub lives: Option<[usize; 4]>,
    pub resigned: [bool; 4],
    pub flagged: [bool; 4],
    // TODO: zombie options
    pub enpassant: [Option<(Position, Position)>; 4],
    pub pawnbaserank: usize,
    pub uniquify: usize,
}

impl Default for Extra {
    fn default() -> Self {
        Self {
            royal: Default::default(),
            lives: None,
            resigned: Default::default(),
            flagged: Default::default(),
            enpassant: Default::default(),
            pawnbaserank: 2,
            uniquify: 0,
        }
    }
}
