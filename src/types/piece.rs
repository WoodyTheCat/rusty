use super::{colour::Colour, piece_type::PieceType};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Piece(pub i32, pub Colour);

impl Into<String> for Piece {
    fn into(self) -> String {
        let mut c = match self.0 {
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
        c.to_string()
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
                0 => "Null",
                1 => "Pawn",
                2 => "Rook",
                3 => "Knight",
                4 => "Bishop",
                5 => "Queen",
                6 => "King",
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
    pub const fn try_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self(0, Colour::White)),
            1 => Some(Self(1, Colour::White)),
            2 => Some(Self(2, Colour::White)),
            3 => Some(Self(3, Colour::White)),
            4 => Some(Self(4, Colour::White)),
            5 => Some(Self(5, Colour::White)),
            6 => Some(Self(6, Colour::White)),
            _ => None,
        }
    }

    pub fn index(index: usize) -> Self {
        Self::try_index(index).unwrap_or_else(|| panic!("Index {} is out of range.", index))
    }

    pub fn from_tuple(piece: PieceType, colour: Colour) -> Self {
        Self(piece as i32, colour)
    }
}
