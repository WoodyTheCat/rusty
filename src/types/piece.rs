use super::{colour::Colour, piece_type::PieceType, EngineError};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Piece(pub i32, pub Colour);

// i32:
//  Pawn
//  Knight
//  Bishop
//  Rook
//  Queen
//  King

impl Into<Result<String, EngineError>> for Piece {
    fn into(self) -> Result<String, EngineError> {
        let mut c = match self.0 {
            0 => 'P',
            1 => 'N',
            2 => 'B',
            3 => 'R',
            4 => 'Q',
            5 => 'K',
            _ => {
                return Err(EngineError(String::from(format!(
                    "[Piece::Into::into] Unknown piece whilst serialising: {}",
                    self.0
                ))));
            }
        };
        if self.1 == Colour::Black {
            c = c.to_lowercase().to_string().chars().next().unwrap();
        };

        Ok(c.to_string())
    }
}

impl From<char> for Piece {
    fn from(c: char) -> Self {
        let colour: Colour = if c.is_lowercase() {
            Colour::Black
        } else {
            Colour::White
        };
        match c.to_lowercase().to_string().chars().next().unwrap() {
            ' ' => Piece(0, colour),
            'p' => Piece(1, colour),
            'n' => Piece(2, colour),
            'b' => Piece(3, colour),
            'r' => Piece(4, colour),
            'q' => Piece(5, colour),
            'k' => Piece(6, colour),
            _ => panic!("Unexpected piece character: {}", c),
        }
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let piece = match self.0 {
                0 => "Pawn",
                1 => "Knight",
                2 => "Bishop",
                3 => "Rook",
                4 => "Queen",
                5 => "King",
                x => panic!("Unrecognised piece while formatting: {}", x),
            };
            writeln!(
                f,
                "Piece({} {})",
                if self.1 == Colour::Black {
                    "Black"
                } else {
                    "White"
                },
                piece
            )?;
        } else {
            let mut c: char = match self.0 {
                0 => 'P',
                1 => 'N',
                2 => 'B',
                3 => 'R',
                4 => 'Q',
                5 => 'K',
                _ => panic!("Unknown piece whilst serialising: {}", self.0),
            };
            if self.1 == Colour::Black {
                c = c.to_lowercase().to_string().chars().next().unwrap();
            };
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

impl Piece {
    pub fn from_tuple(piece: PieceType, colour: Colour) -> Self {
        Self(piece as i32, colour)
    }
}
