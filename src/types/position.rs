use crate::fen;

use super::{
    bitboard::{ToBitboard, BB},
    colour::Colour,
    piece::Piece,
    square::Square,
};

pub struct Position {
    pub bitboards: [BB; 14],
    pub to_move: Colour,
    pub en_passant_file: Option<i8>,
    pub castling_rights: [bool; 4],
    pub half_moves: i32,
    pub full_moves: i32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            bitboards: [BB(0); 14],
            castling_rights: [false; 4],
            en_passant_file: None,
            to_move: Colour::White,
            half_moves: 0,
            full_moves: 0,
        }
    }
}

impl Position {
    pub fn at(&self, square: Square) -> Option<Piece> {
        let sq: BB = square.to_bitboard();
        let out: Option<usize> = self.bitboards.iter().position(|&bb| (bb & sq) != BB(0));

        match out {
            Some(piece) => Some(Piece::index(piece)),
            None => None,
        }
    }
}
impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..=7).rev() {
            writeln!(f, " +---+---+---+---+---+---+---+---+")?;
            for j in 0..=7 {
                let bb = self
                    .bitboards
                    .iter()
                    .position(|bb: &BB| BB(1 << (i * 8 + j)).0 & bb.0 != 0);

                let mut piece = Piece(0, Colour::White);

                match bb {
                    Some(x) => {
                        piece.0 = (x & 0b111) as i32;
                        if x & 8 == 8 {
                            piece.1 = Colour::Black;
                        }
                        write!(f, " | {:?}", piece)?
                    }
                    None => write!(f, " |  ")?,
                };
            }
            write!(f, " | {i}\n")?;
        }

        writeln!(f, " +---+---+---+---+---+---+---+---+")?;
        writeln!(f, "   a   b   c   d   e   f   g   h  ")?;
        writeln!(f, "\n FEN: {}", fen::position_to_fen(&self))?;
        Ok(())
    }
}
