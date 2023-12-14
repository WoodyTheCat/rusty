crate::types::helpers::simple_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum Colour {
        White,
        Black
    }
}

impl Default for Colour {
    fn default() -> Self {
        Self::White
    }
}

crate::types::helpers::enum_char_conv! {
    Colour, ColourParseError {
        White = 'w',
        Black = 'b'
    }
}

impl core::ops::Not for Colour {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
