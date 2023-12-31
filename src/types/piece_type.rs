crate::types::helpers::simple_enum! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[repr(usize)]
    pub enum PieceType {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King
    }
}

impl PieceType {
    pub fn to_piece(&self) -> i32 {
        *self as i32
    }
}
