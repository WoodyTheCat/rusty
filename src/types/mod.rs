use std::num::ParseIntError;

use self::bitboard::BB;

pub mod bitboard;
pub mod board_state;
pub mod chess_move;
pub mod colour;
pub mod helpers;
pub mod piece;
pub mod piece_type;
pub mod position;
pub mod square;

pub const NORTH: i8 = 8;
pub const SOUTH: i8 = -8;
pub const EAST: i8 = 1;
pub const WEST: i8 = -1;

pub const RANK1: BB = 0xFF;
pub const RANK2: BB = RANK1 << 8;
pub const RANK3: BB = RANK1 << (8 * 2);
pub const _RANK4: BB = RANK1 << (8 * 3);
pub const _RANK5: BB = RANK1 << (8 * 4);
pub const RANK6: BB = RANK1 << (8 * 5);
pub const RANK7: BB = RANK1 << (8 * 6);
pub const _RANK8: BB = RANK1 << (8 * 7);

pub const FILEA: BB = 0x101010101010101;
pub const FILEB: BB = FILEA << 1;
pub const _FILEC: BB = FILEA << 2;
pub const _FILED: BB = FILEA << 3;
pub const _FILEE: BB = FILEA << 4;
pub const _FILEF: BB = FILEA << 5;
pub const FILEG: BB = FILEA << 6;
pub const FILEH: BB = FILEA << 7;

#[derive(Debug)]
pub struct EngineError(pub String);

impl Into<std::fmt::Error> for EngineError {
    fn into(self) -> std::fmt::Error {
        std::fmt::Error
    }
}

impl From<ParseIntError> for EngineError {
    fn from(value: ParseIntError) -> Self {
        EngineError(String::from(format!(
            "[ParseInt] Error code {}",
            value.kind().clone() as i8
        )))
    }
}
