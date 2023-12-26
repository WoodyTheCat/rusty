use crate::{
    magics,
    types::{
        bitboard::{Shift, BB},
        piece_type::PieceType,
        r#move::MoveType,
        square::SquareIndex,
        *,
    },
};

pub struct Lookup {
    knight_table: [BB; 64],
    king_table: [BB; 64],
}

impl Default for Lookup {
    fn default() -> Self {
        let mut knight_table: [BB; 64] = [0; 64];
        let mut king_table: [BB; 64] = [0; 64];

        for i in 0..64 {
            knight_table[i] = knight_destinations(i as SquareIndex);
            king_table[i] = king_destinations(i as SquareIndex);
        }

        Self {
            knight_table,
            king_table,
        }
    }
}

impl Lookup {
    pub fn sliding_moves(&self, square: SquareIndex, blockers: BB, piece: PieceType) -> BB {
        match piece {
            PieceType::Bishop => magics::get_slider_moves(square, blockers, false),
            PieceType::Rook => magics::get_slider_moves(square, blockers, true),
            PieceType::Queen => {
                magics::get_slider_moves(square, blockers, true)
                    | magics::get_slider_moves(square, blockers, false)
            }
            _ => 0,
        }
    }

    pub fn moves(&self, square: SquareIndex, piece: PieceType) -> BB {
        match piece {
            PieceType::Knight => *self.knight_table.get(square as usize).unwrap(),
            PieceType::King => *self.king_table.get(square as usize).unwrap(),
            PieceType::Queen => self.sliding_moves(square, 0, PieceType::Queen),
            _ => 0,
        }
    }
}

fn knight_destinations(square: SquareIndex) -> BB {
    let base_bb: BB = 1 << square;

    let nnw: u64 = base_bb.checked_shl(15).unwrap_or(0) & !FILEH;
    let nww: u64 = base_bb.checked_shl(6).unwrap_or(0) & !(FILEH | FILEG);
    let nne: u64 = base_bb.checked_shl(17).unwrap_or(0) & !FILEA;
    let nee: u64 = base_bb.checked_shl(10).unwrap_or(0) & !(FILEA | FILEB);

    let sse: u64 = base_bb.checked_shr(15).unwrap_or(0) & !FILEA;
    let see: u64 = base_bb.checked_shr(6).unwrap_or(0) & !(FILEA | FILEB);
    let ssw: u64 = base_bb.checked_shr(17).unwrap_or(0) & !FILEH;
    let sww: u64 = base_bb.checked_shr(10).unwrap_or(0) & !(FILEG | FILEH);

    nnw | nww | nne | nee | sww | ssw | sse | see
}

fn king_destinations(square: SquareIndex) -> BB {
    let b: BB = 1 << square;
    let mut r: BB = 0;
    for dir in MoveType::king_itr() {
        r |= b.shift(*dir);
    }

    r
}
