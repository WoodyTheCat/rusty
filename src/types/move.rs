use crate::types::{square::SquareIndex, *};
use std::slice::Iter;

#[derive(Debug)]
pub struct Move {
    pub from: SquareIndex,
    pub to: SquareIndex,
    pub kind: MoveType,
}

#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    Capture,
    EnPassantCapture,
    KnightPromotion,
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    KnightPromotionCapture,
    BishopPromotionCapture,
    RookPromotionCapture,
    QueenPromotionCapture,
    Quiet,
    CastleKing,
    CastleQueen,
    Null,
}

impl MoveType {
    pub fn king_itr() -> Iter<'static, i8> {
        static KING_MOVES: [i8; 8] = [
            WEST,
            NORTH + WEST,
            NORTH,
            NORTH + EAST,
            EAST,
            SOUTH + EAST,
            SOUTH,
            SOUTH + WEST,
        ];
        KING_MOVES.iter()
    }
}
