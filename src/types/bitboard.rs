use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BB(pub u64);

impl BB {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(!0);

    pub fn has(&self, square: u64) -> bool {
        (self.0 & square) != 0
    }
}

pub trait ToBitboard {
    fn to_bitboard(&self) -> BB;
}

macro_rules! impl_math_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait for BB {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                Self($trait::$fn(self.0, other.0))
            }
        }
    )*};
}

impl_math_ops! {
    BitAnd, bitand;
    BitOr, bitor;
    BitXor, bitxor;
}

macro_rules! impl_math_assign_ops {
    ($($trait:ident,$fn:ident;)*) => {$(
        impl $trait for BB {
            fn $fn(&mut self, other: Self) {
                $trait::$fn(&mut self.0, other.0)
            }
        }
    )*};
}

impl_math_assign_ops! {
    BitAndAssign, bitand_assign;
    BitOrAssign, bitor_assign;
    BitXorAssign, bitxor_assign;
    ShrAssign, shr_assign;
    ShlAssign, shl_assign;
    AddAssign, add_assign;
    SubAssign, sub_assign;
    MulAssign, mul_assign;
    DivAssign, div_assign;
    RemAssign, rem_assign;
}

impl Not for BB {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
