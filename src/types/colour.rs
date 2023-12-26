use std::ops::{Index, IndexMut};

use super::{bitboard::BB, piece_type::PieceType};

crate::types::helpers::simple_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum Colour {
        White,
        Black
    }
}

impl Default for Colour {
    fn default() -> Self {
        Self::White
    }
}

crate::types::helpers::enum_char_conv! {
    Colour, ColourParseError {
        White = 'w',
        Black = 'b'
    }
}

impl Index<PieceType> for [BB; 6] {
    type Output = BB;

    fn index(&self, piece: PieceType) -> &Self::Output {
        match piece {
            PieceType::Pawn => &self[0],
            PieceType::Rook => &self[1],
            PieceType::Knight => &self[2],
            PieceType::Bishop => &self[3],
            PieceType::Queen => &self[4],
            PieceType::King => &self[5],
        }
    }
}

impl IndexMut<PieceType> for [BB; 6] {
    fn index_mut(&mut self, piece: PieceType) -> &mut Self::Output {
        match piece {
            PieceType::Pawn => &mut self[0],
            PieceType::Rook => &mut self[1],
            PieceType::Knight => &mut self[2],
            PieceType::Bishop => &mut self[3],
            PieceType::Queen => &mut self[4],
            PieceType::King => &mut self[5],
        }
    }
}

impl Index<Colour> for [BB; 2] {
    type Output = BB;

    fn index(&self, color: Colour) -> &Self::Output {
        match color {
            Colour::White => &self[0],
            Colour::Black => &self[1],
        }
    }
}

impl IndexMut<Colour> for [BB; 2] {
    fn index_mut(&mut self, color: Colour) -> &mut Self::Output {
        match color {
            Colour::White => &mut self[0],
            Colour::Black => &mut self[1],
        }
    }
}

impl core::ops::Not for Colour {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
