use crate::fen;

use super::{
    bitboard::{ToBitboard, BB},
    colour::Colour,
    piece::Piece,
    piece_type::PieceType,
    position::Position,
    square::SquareIndex,
};

#[derive(Clone, Copy)]
pub struct BoardState {
    pub position: Position,
    pub active_player: Colour,
    pub en_passant: Option<SquareIndex>,
    pub castling_rights: [bool; 4],
    pub half_moves: i32,
    pub full_moves: i32,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            position: Position::default(),
            castling_rights: [false; 4],
            en_passant: None,
            active_player: Colour::White,
            half_moves: 0,
            full_moves: 0,
        }
    }
}

impl BoardState {
    pub fn at(&self, square: SquareIndex) -> Option<(PieceType, Colour)> {
        let sq: BB = square.to_bitboard();
        let pos: Position = self.position;
        let piece: Option<usize> = pos.pieces_bb.iter().position(|&bb| (bb & sq) != 0);
        let colour: Option<usize> = pos.colours_bb.iter().position(|&bb| (bb & sq) != 0);
        // println!("{:?}", piece);
        // println!("{:?}", colour);

        match (piece, colour) {
            (Some(p), Some(c)) => Some((PieceType::index(p), Colour::index(c))),
            _ => None,
        }
    }
}

impl std::fmt::Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..=7).rev() {
            writeln!(f, " +---+---+---+---+---+---+---+---+")?;
            for j in 0..=7 {
                let piece: Option<(PieceType, Colour)> = self.at(i * 8 + j);

                if let Some((piece, colour)) = piece {
                    write!(f, " | {:?}", Piece::from_tuple(piece, colour))?;
                } else {
                    write!(f, " |  ")?;
                }
            }
            write!(f, " | {}\n", i + 1)?;
        }

        writeln!(f, " +---+---+---+---+---+---+---+---+")?;
        writeln!(f, "   a   b   c   d   e   f   g   h  ")?;
        writeln!(f, "\n FEN: {}", fen::board_to_fen(&self))?;
        Ok(())
    }
}
