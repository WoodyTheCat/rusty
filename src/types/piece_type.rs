use super::EngineError;

crate::types::helpers::simple_enum! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum PieceType {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King
    }
}

impl PieceType {
    pub fn to_piece(&self) -> i32 {
        *self as i32
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Pawn => 'P',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }

    pub fn from_char(mut c: char) -> Result<Self, EngineError> {
        c.make_ascii_uppercase();
        Ok(match c {
            'P' => Self::Pawn,
            'N' => Self::Knight,
            'B' => Self::Bishop,
            'R' => Self::Rook,
            'Q' => Self::Queen,
            'K' => Self::King,
            e => {
                return Err(EngineError(format!("Unexpected piece character '{e}'")));
            }
        })
    }
}
