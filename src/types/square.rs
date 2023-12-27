use super::{
    bitboard::{ToBitboard, BB},
    colour::Colour,
};

crate::types::helpers::simple_enum! {
    #[derive(Clone, Copy, Debug)]
    pub enum File {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H
    }
}
crate::types::helpers::simple_enum! {
    #[derive(Clone, Copy, Debug)]
    pub enum Rank {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
        Sixth,
        Seventh,
        Eighth
    }
}

crate::types::helpers::simple_enum! {
    #[derive(Clone, Copy, Debug)]
    #[repr(u64)]
    pub enum Square {
        A1, B1, C1, D1, E1, F1, G1, H1,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A8, B8, C8, D8, E8, F8, G8, H8
    }
}

crate::types::helpers::enum_char_conv! {
    Rank, RankParseError {
        First = '1',
        Second = '2',
        Third = '3',
        Fourth = '4',
        Fifth = '5',
        Sixth = '6',
        Seventh = '7',
        Eighth = '8'
    }
}

crate::types::helpers::enum_char_conv! {
    File, FileParseError {
        A = 'a',
        B = 'b',
        C = 'c',
        D = 'd',
        E = 'e',
        F = 'f',
        G = 'g',
        H = 'h'
    }
}

pub type SquareIndex = u64;

pub trait SquareIndexMethods {
    fn parse(notation: &str) -> Self;
    fn to_algebraic(&self) -> String;
}

impl SquareIndexMethods for SquareIndex {
    fn parse(notation: &str) -> Self {
        let mut chars = notation.chars();
        let file: u64 = "abcdefgh".find(chars.next().unwrap()).unwrap_or(0) as u64;
        let a: char = chars.next().unwrap();
        let rank: u64 = (a.to_digit(10).unwrap_or(0) - 1) as u64;
        rank * 8 + file
    }

    fn to_algebraic(&self) -> String {
        let rank: String = ((*self >> 3) + 1).to_string();
        let file: String = "abcdefgh"
            .chars()
            .nth(*self as usize % 8)
            .unwrap_or('-')
            .to_string();

        file + rank.as_str()
    }
}

impl ToBitboard for SquareIndex {
    fn to_bitboard(&self) -> BB {
        1 << self
    }
}

impl ToBitboard for File {
    fn to_bitboard(&self) -> BB {
        0x0101010101010101 << *self as u8
    }
}

impl ToBitboard for Rank {
    fn to_bitboard(&self) -> BB {
        0xff << (*self as u8 * 8)
    }
}

impl ToBitboard for Square {
    fn to_bitboard(&self) -> BB {
        1 << *self as u8
    }
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Self::index(((rank as usize) << 3) | file as usize)
    }

    pub fn file(self) -> File {
        File::index(self as usize & 0b000111)
    }

    pub fn rank(self) -> Rank {
        Rank::index(self as usize >> 3)
    }

    pub fn from_algebraic(notation: &str) -> Self {
        let next = || notation.chars().next().unwrap();
        let rank: usize = "abcdefgh".find(next()).unwrap_or(0);
        let file: usize = next().to_digit(10).unwrap_or(0) as usize;
        Self::index(rank * 8 + file)
    }

    pub fn to_square_index(&self) -> SquareIndex {
        *self as u64
    }
}

impl Rank {
    #[inline(always)]
    pub const fn flip(self) -> Self {
        Self::index_const(Self::Eighth as usize - self as usize)
    }

    /// Get a rank bitboard, 1 being A and 8 being H
    pub fn rank(index: u8) -> BB {
        0b11111111 << index - 1
    }

    pub const fn bitboard(self) -> BB {
        0b11111111 << (self as u8 * 8)
    }

    pub fn relative_to(self, colour: Colour) -> Self {
        if Colour::White == colour {
            self
        } else {
            self.flip()
        }
    }
}
