/// Simple enum for color
///
/// Dead also counts as a color, because dead pieces can go the board.
#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Dead,
}

/// Simple representation of pieces that allows all types of fairy pieces
/// The trick is to just use the character that the notation uses to represent that piece.
///
/// Walls and empty cells are special, but everything else has a color as well.
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
///       ^ dead          ^ queen         ^ custom position?
/// ```
/// It starts with a Color, followed by 4 integer arrays of length 4, followed by
/// 0, followed by an optional extra data section. Each of these is separated by a
/// '-'. All of the arrays are information about the players with the leftmost data
///  about Red and proceding clockwise. Most of this is parsed in one go, but the
/// extra_options is just stored as a string and can be parsed if it is needed.
///
/// The final integer in the metadata format does not have a clear meaning I have
/// been able to find. It seems to trigger a setting for custom position, but all
/// fen4's are custom except the default. If you find its use, please make and issue.
#[derive(Debug, PartialEq)]
pub struct Board {
    pub turn: Color,
    pub dead: [bool; 4],
    pub castling_king: [bool; 4],
    pub castling_queen: [bool; 4],
    pub points: [u32; 4],
    pub extra_options: TaggedData,
    pub board: [[Piece; 15]; 15],
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
/// Rather than parse the known options, TaggedData stores the label as the characters
/// between the `'`s and the value as the characters between `:` and `,` or `}`.
/// All of the tagged values are stored a list of `(label,value)` pairs.
#[derive(Debug, PartialEq)]
pub struct TaggedData {
    pub tags: Vec<(String, String)>,
}
