use super::{
    bitboard::BB,
    colour::Colour,
    piece_type::PieceType,
    r#move::{
        Move,
        MoveType::{self, *},
    },
    square::{Square::*, SquareIndex},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub pieces_bb: [BB; 6],
    pub colours_bb: [BB; 2],
}

impl Default for Position {
    fn default() -> Self {
        Self {
            pieces_bb: [0; 6],
            colours_bb: [0; 2],
        }
    }
}

impl Position {
    pub fn bb(&self, colour: Colour, piece: PieceType) -> BB {
        let out: BB = self.pieces_bb[piece as usize];
        out & self.colours_bb[colour as usize]
    }

    pub fn bb_colour(&self, colour: Colour) -> BB {
        self.colours_bb[colour as usize]
    }

    pub fn bb_piece(&self, piece: PieceType) -> BB {
        self.pieces_bb[piece as usize]
    }

    pub fn bb_all(&self) -> BB {
        self.colours_bb[0] | self.colours_bb[1]
    }

    pub fn add_piece(&mut self, colour: Colour, piece: PieceType, square: SquareIndex) {
        self.colours_bb[colour as usize] |= 1 << square;
        self.pieces_bb[piece as usize] |= 1 << square;
    }

    pub fn remove_piece(&mut self, colour: Colour, piece: PieceType, square: SquareIndex) {
        self.colours_bb[colour as usize] ^= 1 << square;
        self.pieces_bb[piece as usize] ^= 1 << square;
    }

    pub fn type_at(&self, square: SquareIndex) -> Option<PieceType> {
        let sq: u64 = 1 << square;
        let piece: Option<usize> = self.pieces_bb.iter().position(|&bb| (bb & sq) != 0);

        match piece {
            Some(x) => Some(PieceType::index(x)),
            _ => None,
        }
    }

    pub fn capture(&mut self, mv: &Move, active: Colour) {
        let captured: PieceType = self.type_at(mv.to).unwrap();
        let kind: PieceType = self.type_at(mv.from).unwrap();
        self.remove_piece(active, kind, mv.from);
        self.remove_piece(!active, captured, mv.to);
        self.add_piece(active, kind, mv.to);
    }

    pub fn castle(&mut self, kind: MoveType, color: Colour) {
        match kind {
            CastleKing => self.castle_king(color),
            CastleQueen => self.castle_queen(color),
            _ => {}
        }
    }

    fn castle_king(&mut self, color: Colour) {
        match color {
            Colour::White => {
                self.remove_piece(color, PieceType::King, E1 as SquareIndex);
                self.remove_piece(color, PieceType::Rook, H1 as SquareIndex);
                self.add_piece(color, PieceType::King, G1 as SquareIndex);
                self.add_piece(color, PieceType::Rook, F1 as SquareIndex);
            }
            Colour::Black => {
                self.remove_piece(color, PieceType::King, E8 as SquareIndex);
                self.remove_piece(color, PieceType::Rook, H8 as SquareIndex);
                self.add_piece(color, PieceType::King, G8 as SquareIndex);
                self.add_piece(color, PieceType::Rook, F8 as SquareIndex);
            }
        }
    }

    fn castle_queen(&mut self, color: Colour) {
        match color {
            Colour::White => {
                self.remove_piece(color, PieceType::King, E1 as SquareIndex);
                self.remove_piece(color, PieceType::Rook, A1 as SquareIndex);
                self.add_piece(color, PieceType::King, C1 as SquareIndex);
                self.add_piece(color, PieceType::Rook, D1 as SquareIndex);
            }
            Colour::Black => {
                self.remove_piece(color, PieceType::King, E8 as SquareIndex);
                self.remove_piece(color, PieceType::Rook, A8 as SquareIndex);
                self.add_piece(color, PieceType::King, C8 as SquareIndex);
                self.add_piece(color, PieceType::Rook, D8 as SquareIndex);
            }
        }
    }
}
