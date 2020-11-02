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

impl fmt::Display for Extra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn royal_helper(f: &mut fmt::Formatter<'_>, value: &Option<Position>) -> fmt::Result {
            if let Some(p) = value {
                write!(f, "'{}'", p)
            } else {
                write!(f, "''")
            }
        }

        fn en_passant_helper(
            f: &mut fmt::Formatter<'_>,
            value: &Option<(Position, Position)>,
        ) -> fmt::Result {
            if let Some((first, second)) = value {
                write!(f, "'{}:{}'", first, second)
            } else {
                write!(f, "''")
            }
        }
        if *self != Extra::default() {
            let mut comma = false;
            if &self.royal != &[None, None, None, None] {
                write!(f, "'royal':(")?;
                royal_helper(f, &self.royal[0])?;
                write!(f, ",")?;
                royal_helper(f, &self.royal[1])?;
                write!(f, ",")?;
                royal_helper(f, &self.royal[2])?;
                write!(f, ",")?;
                royal_helper(f, &self.royal[3])?;
                write!(f, ")")?;
                comma = true;
            }
            if let Some(lives) = self.lives {
                if comma {
                    write!(f, ",")?;
                }
                write!(
                    f,
                    "'lives':({},{},{},{})",
                    lives[0], lives[1], lives[2], lives[3]
                )?;
                comma = true;
            }
            if self.resigned != [false; 4] {
                if comma {
                    write!(f, ",")?;
                }
                write!(
                    f,
                    "'resigned':({:?},{:?},{:?},{:?})",
                    self.resigned[0], self.resigned[1], self.resigned[2], self.resigned[3]
                )?;
                comma = true;
            }
            if self.flagged != [false; 4] {
                if comma {
                    write!(f, ",")?;
                }
                write!(
                    f,
                    "'flagged':({:?},{:?},{:?},{:?})",
                    self.flagged[0], self.flagged[1], self.flagged[2], self.flagged[3]
                )?;
                comma = true;
            }
            if &self.enpassant != &[None, None, None, None] {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, "'enPassant':(")?;
                en_passant_helper(f, &self.enpassant[0])?;
                write!(f, ",")?;
                en_passant_helper(f, &self.enpassant[1])?;
                write!(f, ",")?;
                en_passant_helper(f, &self.enpassant[2])?;
                write!(f, ",")?;
                en_passant_helper(f, &self.enpassant[3])?;
                write!(f, ")")?;
                comma = true;
            }
            if self.pawnbaserank != 2 {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, "'pawnBaseRank':{}", self.pawnbaserank)?;
                comma = true;
            }
            if self.uniquify != 0 {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, "'uniquify':{}", self.uniquify)?;
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
        write!(f, "-{}-", self.draw_ply)?;
        if self.extra_options != Extra::default() {
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
