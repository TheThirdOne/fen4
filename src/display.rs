use std::fmt;

use crate::types::*;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.row >= 14 || self.col >= 14 {
            eprintln!("BAD POSITION {} {}", self.row, self.col);
            return Err(fmt::Error);
        }
        let column_letter: char = ((self.col as u8) + b'a').into();
        write!(f, "{}{}", column_letter, self.row + 1)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Turn(tc) => match tc {
                    TurnColor::Red => "r",
                    TurnColor::Blue => "b",
                    TurnColor::Yellow => "y",
                    TurnColor::Green => "g",
                },
                Color::Dead(None) => "d",
                Color::Dead(Some(tc)) => match tc {
                    TurnColor::Red => "dr",
                    TurnColor::Blue => "db",
                    TurnColor::Yellow => "dy",
                    TurnColor::Green => "dg",
                },
            }
        )
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Empty => write!(f, ""),
            Piece::Wall => write!(f, "X"),
            Piece::Normal(color, shape) => write!(f, "{}{}", color, shape),
        }
    }
}

impl fmt::Display for TaggedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tags.len() != 0 {
            write!(f, "'{}':{}", self.tags[0].0, self.tags[0].1)?;
            for (label, value) in &self.tags[1..] {
                write!(f, ",'{}':{}", label, value)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write out a line like: R-0,0,0,0-1,1,1,1-1,1,1,1-0,0,0,0-0-\n
        write!(
            f,
            "{}",
            match self.turn {
                TurnColor::Red => "R",
                TurnColor::Blue => "B",
                TurnColor::Yellow => "Y",
                TurnColor::Green => "G",
            }
        )?;
        write!(f, "-{}", if self.dead[0] { "1" } else { "0" })?;
        for d in &self.dead[1..] {
            write!(f, ",{}", if *d { "1" } else { "0" })?;
        }
        write!(f, "-{}", if self.castling_king[0] { "1" } else { "0" })?;
        for c in &self.castling_king[1..] {
            write!(f, ",{}", if *c { "1" } else { "0" })?;
        }
        write!(f, "-{}", if self.castling_queen[0] { "1" } else { "0" })?;
        for c in &self.castling_queen[1..] {
            write!(f, ",{}", if *c { "1" } else { "0" })?;
        }
        write!(f, "-{}", self.points[0])?;
        for p in &self.points[1..] {
            write!(f, ",{}", p)?;
        }
        write!(f, "-0-")?;
        if self.extra_options.tags.len() != 0 {
            write!(f, "{{{}}}-", self.extra_options)?;
        }
        write!(f, "\n")?;

        // Write out 14 lines like: 3,yP,yP,yP,yP,yP,yP,yP,yP,3/\n
        for i in (0..14).rev() {
            let mut empties = 0;
            for j in 0..13 {
                match &self.board[i][j] {
                    Piece::Empty => {
                        empties += 1;
                    }
                    p => {
                        if empties != 0 {
                            write!(f, "{},", empties)?;
                        }
                        write!(f, "{},", p)?;
                        empties = 0;
                    }
                }
            }
            match &self.board[i][13] {
                Piece::Empty => {
                    write!(f, "{}", empties + 1)?;
                }
                p => {
                    if empties != 0 {
                        write!(f, "{},", empties)?;
                    }
                    write!(f, "{}", p)?;
                }
            }

            if i != 0 {
                write!(f, "/\n")?;
            }
        }
        Ok(())
    }
}
