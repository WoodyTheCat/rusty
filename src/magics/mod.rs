use crate::types::{bitboard::BB, square::*};

include!("magics.rs");

/// Responsible for generating the precomputed magic move tables
// pub mod magic_generate;

#[derive(Debug)]
pub struct MagicEntry {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

fn magic_index(entry: &MagicEntry, blockers: BB) -> usize {
    let blockers: u64 = blockers & entry.mask;
    let hash: u64 = blockers.wrapping_mul(entry.magic);
    let index: usize = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

pub fn get_slider_moves(square: SquareIndex, blockers: BB, ortho: bool) -> BB {
    if ortho {
        get_rook_moves(square, blockers)
    } else {
        get_bishop_moves(square, blockers)
    }
}

fn get_rook_moves(square: SquareIndex, blockers: BB) -> BB {
    let magic: &MagicEntry = &ROOK_MAGICS[square as usize];
    ROOK_MOVES[magic_index(magic, blockers)]
}

fn get_bishop_moves(square: SquareIndex, blockers: BB) -> BB {
    let magic: &MagicEntry = &BISHOP_MAGICS[square as usize];
    BISHOP_MOVES[magic_index(magic, blockers)]
}

pub fn _test() {
    let blockers: BB = 0x20770001400C302;
    let square: Square = Square::C8;
    println!("Blockers: {:#?}", blockers);
    println!("Square: {:?}", square);
    println!("Rook moves: {:#?}", get_rook_moves(square as u64, blockers));
    println!(
        "Bishop moves: {:#?}",
        get_bishop_moves(square as u64, blockers)
    );
}
