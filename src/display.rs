use std::fmt;

use crate::types::*;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Empty => write!(f, ""),
            Piece::Wall => write!(f, "X"),
            Piece::Normal(color, shape) => write!(
                f,
                "{}{}",
                match color {
                    Color::Red => "r",
                    Color::Blue => "b",
                    Color::Yellow => "y",
                    Color::Green => "g",
                    Color::Dead => "d",
                },
                shape
            ),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.turn {
                Color::Red => "R",
                Color::Blue => "B",
                Color::Yellow => "Y",
                Color::Green => "G",
                Color::Dead => "D",
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
        if self.extra_options != "" {
            write!(f, "{}-", self.extra_options)?;
        }
        write!(f, "\n")?;
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
