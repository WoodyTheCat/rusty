use self::bitboard::BB;

pub mod bitboard;
pub mod board_state;
pub mod colour;
pub mod helpers;
pub mod r#move;
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
pub const RANK4: BB = RANK1 << (8 * 3);
pub const RANK5: BB = RANK1 << (8 * 4);
pub const RANK6: BB = RANK1 << (8 * 5);
pub const RANK7: BB = RANK1 << (8 * 6);
pub const RANK8: BB = RANK1 << (8 * 7);

pub const FILEA: BB = 0b1_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_u64;
pub const FILEB: BB = FILEA << 1;
pub const FILEC: BB = FILEA << 2;
pub const FILED: BB = FILEA << 3;
pub const FILEE: BB = FILEA << 4;
pub const FILEF: BB = FILEA << 5;
pub const FILEG: BB = FILEA << 6;
pub const FILEH: BB = FILEA << 7;
