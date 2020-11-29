/// Position on the board e.g. a4
///
/// Both row and col should be in the range 0-13.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

/// Simple enum for colors / players.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TurnColor {
    Red,
    Blue,
    Yellow,
    Green,
}

/// Color modifier for pieces
///
/// Includes normal pieces, dead pieces, and dead pieces that also track which player they came from.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Color {
    Turn(TurnColor),
    Dead(Option<TurnColor>),
}

impl Color {
    pub fn is_dead(self) -> bool {
        match self {
            Self::Dead(_) => true,
            _ => false,
        }
    }
}

/// Simple representation of pieces that allows all types of fairy pieces
/// The trick is to just use the character that the notation uses to represent that piece.
///
/// Walls and empty cells are special, but everything else has a color as well.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Board {
    pub turn: TurnColor,
    pub dead: [bool; 4],
    pub castling_king: [bool; 4],
    pub castling_queen: [bool; 4],
    pub points: [u16; 4],
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

impl Board {
    pub fn chess960(mut N: u16) -> Board {
        N -= 1; // chess.com used 1-960
                // Mapping derived from
                // https://en.wikipedia.org/wiki/Fischer_Random_Chess_numbering_scheme#Direct_derivation
        let B1 = 2 * (N % 4) + 1;
        let N2 = N / 4;
        let B2 = 2 * (N2 % 4);
        let N3 = N2 / 4;
        let Q = N3 % 6;
        let N4 = N3 / 6;
        let knights = N4 % 10; // Instead of making N > 960 invalid, just work on N % 960
        let mut back_row = ['P'; 8];
        back_row[B1 as usize] = 'B';
        back_row[B2 as usize] = 'B';
        fn place(mut empties: u16, to_place: char, buffer: &mut [char]) {
            let mut i = 0;
            loop {
                if empties == 0 && buffer[i] == 'P' {
                    break;
                }
                if buffer[i] == 'P' {
                    empties -= 1;
                }
                i += 1;
            }
            buffer[i] = to_place;
        }
        place(Q, 'Q', &mut back_row);
        let (knight1, knight2) = [
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 2),
            (2, 3),
            (3, 3),
        ][knights as usize];
        place(knight1, 'N', &mut back_row);
        place(knight2, 'N', &mut back_row);
        place(0, 'R', &mut back_row);
        place(0, 'K', &mut back_row);
        place(0, 'R', &mut back_row);
        let mut output = Board::default();
        for i in 0..8 {
            output.board[0][i + 3] = Piece::Normal(Color::Turn(TurnColor::Red), back_row[i]);
            output.board[i + 3][0] = Piece::Normal(Color::Turn(TurnColor::Blue), back_row[i]);
            output.board[13][10 - i] = Piece::Normal(Color::Turn(TurnColor::Yellow), back_row[i]);
            output.board[10 - i][13] = Piece::Normal(Color::Turn(TurnColor::Green), back_row[i]);
        }
        output
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
///   - `'stalemated':(true,true,false,false)`
///     - It is not clear this has any effect, but it might affect the "DeadKingWalking" feature.
///   - `'zombieImmune':(true,false,false,true)`
///     - Makes zombie pieces impossible to capture
///   - `'zombieType':('','','','muncher')`
///     - Used to change the behaviour of zombies
///     - Possible types include muncher, comfuter, checker, ranter, and possibly more
///
/// The labels have a preferred order. The preferred order is the order of the fields of the struct.
///
/// gameOver is an additional option, but only seems to appear in internal messages.
/// It would fit somwhere between flagged and enPassant in terms of preferred order and stores
/// the message that shows up at the end of the game.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Extra {
    pub royal: [Option<Position>; 4],
    pub lives: Option<[usize; 4]>,
    pub resigned: [bool; 4],
    pub flagged: [bool; 4],
    pub stalemated: [bool; 4],
    pub zombie_immune: [bool; 4],
    pub zombie_type: [String; 4],
    pub enpassant: [Option<(Position, Position)>; 4],
    pub pawnbaserank: usize,
    pub uniquify: usize,
    pub std2pc: bool,
}

impl Default for Extra {
    fn default() -> Self {
        Self {
            royal: Default::default(),
            lives: None,
            resigned: Default::default(),
            flagged: Default::default(),
            stalemated: Default::default(),
            zombie_immune: Default::default(),
            zombie_type: Default::default(),
            enpassant: Default::default(),
            pawnbaserank: 2,
            uniquify: 0,
            std2pc: false,
        }
    }
}
