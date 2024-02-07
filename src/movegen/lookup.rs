use crate::{
    magics,
    types::{
        bitboard::{Shift, BB},
        chess_move::MoveType,
        piece_type::PieceType::{self, *},
        square::SquareIndex,
        *,
    },
};

use itertools::Itertools;

pub struct Lookup {
    knight_table: [BB; 64],
    king_table: [BB; 64],
    ortho_influence: [BB; 64],
    diag_influence: [BB; 64],
    between: [[BB; 64]; 64],
}

impl Default for Lookup {
    fn default() -> Self {
        let mut knight_table: [BB; 64] = [0; 64];
        let mut king_table: [BB; 64] = [0; 64];

        for i in 0..64 {
            knight_table[i] = knight_destinations(i as SquareIndex);
            king_table[i] = king_destinations(i as SquareIndex);
        }

        let (ortho_influence, diag_influence) = Self::init_influence();

        let between: [[BB; 64]; 64] = Self::init_between();

        Self {
            knight_table,
            king_table,
            ortho_influence,
            diag_influence,
            between,
        }
    }
}

impl Lookup {
    pub fn sliding_moves(&self, square: SquareIndex, blockers: BB, piece: PieceType) -> BB {
        match piece {
            Bishop => magics::get_slider_moves(square, blockers, false),
            Rook => magics::get_slider_moves(square, blockers, true),
            Queen => {
                magics::get_slider_moves(square, blockers, true)
                    | magics::get_slider_moves(square, blockers, false)
            }
            _ => 0,
        }
    }

    pub fn moves(&self, square: SquareIndex, piece: PieceType) -> BB {
        match piece {
            Knight => *self.knight_table.get(square as usize).unwrap(),
            King => *self.king_table.get(square as usize).unwrap(),
            Queen => self.sliding_moves(square, 0, PieceType::Queen),
            _ => 0,
        }
    }

    pub fn ray_between(&self, s1: SquareIndex, s2: SquareIndex) -> BB {
        let full: BB = !0;
        let bb1: BB = 1 << s1;
        let bb2: BB = 1 << s2;
        self.between(s1, s2) & ((full << s1) ^ (full << s2)) | bb1 | bb2
    }

    pub fn between(&self, s1: SquareIndex, s2: SquareIndex) -> BB {
        self.between[s1 as usize][s2 as usize]
    }

    pub fn piece_influence(&self, ortho: bool, square: SquareIndex) -> BB {
        if ortho {
            self.ortho_influence[square as usize]
        } else {
            self.diag_influence[square as usize]
        }
    }

    fn init_influence() -> ([BB; 64], [BB; 64]) {
        let mut ortho: [BB; 64] = [0; 64];
        let mut diag: [BB; 64] = [0; 64];

        for i in 0..64 {
            ortho[i] = magics::get_slider_moves(i as u64, 0, true);
            diag[i] = magics::get_slider_moves(i as u64, 0, false);
        }

        (ortho, diag)
    }

    fn init_between() -> [[BB; 64]; 64] {
        let mut b: [[BB; 64]; 64] = [[0; 64]; 64];

        for piece in &[Rook, Bishop] {
            for (i, j) in (0..64).cartesian_product(0..64) {
                let bitboard_i: BB = 1 << i;
                let bitboard_j: BB = 1 << j;
                let attacks_i = magics::get_slider_moves(i, 0, *piece == Rook);

                if attacks_i & bitboard_j != 0 {
                    match piece {
                        Rook => {
                            b[i as usize][j as usize] = attacks_i
                                & magics::get_slider_moves(j, 0, true)
                                | bitboard_i
                                | bitboard_j
                        }
                        Bishop => {
                            b[i as usize][j as usize] = attacks_i
                                & magics::get_slider_moves(j, 0, false)
                                | bitboard_i
                                | bitboard_j
                        }
                        // Can't happen ↓
                        _ => {}
                    }
                }
            }
        }
        b
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
    for dir in MoveType::king_iter() {
        r |= b.shift(*dir);
    }

    r
}
