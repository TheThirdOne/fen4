//! Fen4 provides a mapping from a simple representation of a 4 player chess board and the fen4 file format used by
//! [Chess.com](https://www.chess.com/4-player-chess).
//!
//! ### Quick Start
//!
//! The [`Board`] struct is the important type in this crate. All other types are present to support all the features of [`Board`]. The most common ways to get a [`Board`] would be via [`FromStr`](`std::str::FromStr`) or [`Default`](`std::default::Default`).
//!
//! ```rust
//! # fn main() -> Result<(),fen4::BoardParseError> {
//! let empty_fen = "R-0,0,0,0-0,0,0,0-0,0,0,0-0,0,0,0-0-14/14/14/14/14/14/14/14/14/14/14/14/14/14";
//! let board :  Result<fen4::Board,fen4::BoardParseError> = empty_fen.parse();
//! println!("{}",board?);
//! # Ok(())
//! # }
//! ```

mod conversions;
mod display;
mod from_str;
mod types;

pub use from_str::BoardParseError;
pub use from_str::PieceParseError;
pub use from_str::PositionParseError;
pub use types::*;
