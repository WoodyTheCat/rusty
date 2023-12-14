use crate::types::{bitboard::BB, square::*};

include!("magics.rs");

/// Responsible for generating the precomputed magic move tables
// pub mod magic_generate;

pub struct MagicEntry {
    pub mask: u64,
    pub magic: u64,
    pub shift: u8,
    pub offset: u32,
}

fn magic_index(entry: &MagicEntry, blockers: BB) -> usize {
    let blockers = blockers.0 & entry.mask;
    let hash = blockers.wrapping_mul(entry.magic);
    let index = (hash >> entry.shift) as usize;
    entry.offset as usize + index
}

pub fn get_slider_moves(square: Square, blockers: BB, ortho: bool) -> BB {
    if ortho {
        get_rook_moves(square, blockers)
    } else {
        get_bishop_moves(square, blockers)
    }
}

fn get_rook_moves(square: Square, blockers: BB) -> BB {
    let magic = &ROOK_MAGICS[square as usize];
    BB(ROOK_MOVES[magic_index(magic, blockers)])
}

fn get_bishop_moves(square: Square, blockers: BB) -> BB {
    let magic = &BISHOP_MAGICS[square as usize];
    BB(BISHOP_MOVES[magic_index(magic, blockers)])
}

pub fn test() {
    let blockers = BB(0x20770001400C302);
    let square = Square::F5;
    println!("Blockers: {:#?}", blockers);
    println!("Square: {:#b}", square as u64);
    println!("Rook moves: {:#?}", get_rook_moves(square, blockers));
    println!("Bishop moves: {:#?}", get_bishop_moves(square, blockers));
}
