use crate::types::{
    chess_move::{Move, MoveType},
    square::SquareIndex,
    *,
};

pub type BB = u64;

// Piece Iterators

pub trait PieceItr {
    fn iter(&self) -> BBIterator;
}

impl PieceItr for BB {
    fn iter(&self) -> BBIterator {
        BBIterator { bb: *self }
    }
}

pub struct BBIterator {
    bb: BB,
}

impl Iterator for BBIterator {
    type Item = (SquareIndex, BB);

    fn next(&mut self) -> Option<(SquareIndex, BB)> {
        if self.bb == 0 {
            return None;
        }

        let square: u64 = self.bb.trailing_zeros() as u64;
        self.bb &= self.bb - 1;
        Some(square as SquareIndex)
    }
}

fn shift_left(n: u64, i: u8) -> BB {
    n.checked_shl(u32::from(i)).unwrap_or(0)
}

fn shift_right(n: u64, i: u8) -> BB {
    n.checked_shr(u32::from(i)).unwrap_or(0)
}

pub trait Shift {
    fn shift(&self, n: i8) -> BB;
}

impl Shift for BB {
    fn shift(&self, n: i8) -> BB {
        if n == NORTH {
            shift_left(*self, 8)
        } else if n == SOUTH {
            shift_right(*self, 8)
        } else if n == NORTH + NORTH {
            shift_left(*self, 16)
        } else if n == SOUTH + SOUTH {
            shift_right(*self, 16)
        } else if n == EAST {
            shift_left(*self & !FILEH, 1)
        } else if n == WEST {
            shift_right(*self & !FILEA, 1)
        } else if n == NORTH + EAST {
            shift_left(*self & !FILEH, 9)
        } else if n == NORTH + WEST {
            shift_left(*self & !FILEA, 7)
        } else if n == SOUTH + EAST {
            shift_right(*self & !FILEH, 7)
        } else if n == SOUTH + WEST {
            shift_right(*self & !FILEA, 9)
        } else if n > 0 {
            shift_left(*self, n as u8)
        } else {
            shift_right(*self, -n as u8)
        }
    }
}

pub trait BBMethods {
    const EMPTY: Self;
    const FULL: Self;

    fn has(self, square: SquareIndex) -> bool;
    fn is_empty(self) -> bool;
    fn clear_at(&mut self, index: usize);
    fn extract_moves(&mut self, original: SquareIndex, flag: MoveType) -> Vec<Move>;
    fn add_at(&mut self, square: SquareIndex);
    // fn debug(&self);
    fn merge(&mut self, other: BB);
}

impl BBMethods for BB {
    const EMPTY: Self = 0;
    const FULL: Self = !0;

    fn has(self, square: SquareIndex) -> bool {
        !(self & (1 << square)).is_empty()
    }

    fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    fn add_at(&mut self, square: SquareIndex) {
        *self |= 1 << square
    }

    fn clear_at(&mut self, index: usize) {
        *self ^= 1 << index;
    }

    fn extract_moves(&mut self, original: SquareIndex, kind: MoveType) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        while !self.is_empty() {
            let move_index: SquareIndex = self.trailing_zeros() as u64;

            moves.push(Move {
                from: original,
                to: move_index,
                kind,
            });

            self.clear_at(move_index as usize);
        }

        moves
    }

    // fn debug(&self) {
    //     print!("bitboard {{");
    //     for rank in (0..8).rev() {
    //         print!("\n   ");
    //         for file in 0..8 {
    //             if self.has(1 << (rank as u64 * 8 + file as u64)) {
    //                 print!(" X");
    //             } else {
    //                 print!(" .");
    //             }
    //         }
    //     }
    //     print!("\n}}");
    // }

    fn merge(&mut self, other: BB) {
        *self |= other;
    }
}

pub trait ToBitboard {
    fn to_bitboard(&self) -> BB;
}
