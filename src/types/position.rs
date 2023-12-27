use super::{bitboard::BB, colour::Colour, piece_type::PieceType};

#[derive(Clone, Copy)]
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
        out & self.colours_bb[colour]
    }

    pub fn bb_colour(&self, color: Colour) -> BB {
        self.colours_bb[color]
    }

    pub fn bb_piece(&self, piece: PieceType) -> BB {
        self.pieces_bb[piece]
    }

    pub fn bb_all(&self) -> BB {
        self.colours_bb[0] | self.colours_bb[1]
    }
}
