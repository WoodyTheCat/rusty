use crate::types::{
    chess_move::MoveType::*,
    piece_type::PieceType::{self, *},
    square::SquareIndex,
    *,
};
use std::{fmt::Display, slice::Iter};

use super::square::SquareIndexMethods;

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub from: SquareIndex,
    pub to: SquareIndex,
    pub kind: MoveType,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from.to_algebraic(), self.to.to_algebraic())?;

        if self.is_promotion() {
            let into = match self.promoted_piece().unwrap() {
                Rook => "r",
                Knight => "k",
                Bishop => "b",
                Queen => "q",
                _ => "?",
            };

            write!(f, "={}", into)?;
        }

        Ok(())
    }
}

impl Move {
    pub fn is_promotion_capture(&self) -> bool {
        static PROMOTIONS: [MoveType; 4] = [
            KnightPromotionCapture,
            BishopPromotionCapture,
            RookPromotionCapture,
            QueenPromotionCapture,
        ];

        PROMOTIONS.contains(&self.kind)
    }

    pub fn is_promotion(&self) -> bool {
        static PROMOTIONS: [MoveType; 8] = [
            KnightPromotion,
            BishopPromotion,
            RookPromotion,
            QueenPromotion,
            KnightPromotionCapture,
            BishopPromotionCapture,
            RookPromotionCapture,
            QueenPromotionCapture,
        ];

        PROMOTIONS.contains(&self.kind)
    }

    pub fn is_castle(&self) -> bool {
        self.kind == CastleKing || self.kind == CastleQueen
    }

    pub fn promoted_piece(&self) -> Option<PieceType> {
        match self.kind {
            RookPromotionCapture | RookPromotion => Some(Rook),
            KnightPromotionCapture | KnightPromotion => Some(Knight),
            BishopPromotionCapture | BishopPromotion => Some(Bishop),
            QueenPromotionCapture | QueenPromotion => Some(Queen),
            _ => None,
        }
    }

    pub fn to_notation(&self) -> String {
        let mut notation: String = "".to_string();

        notation += self.from.to_algebraic().as_str();
        notation += self.to.to_algebraic().as_str();

        notation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn king_iter() -> Iter<'static, i8> {
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

    pub fn promotion_iter() -> Iter<'static, MoveType> {
        static PROMOTIONS: [MoveType; 4] = [
            MoveType::KnightPromotion,
            MoveType::BishopPromotion,
            MoveType::RookPromotion,
            MoveType::QueenPromotion,
        ];
        PROMOTIONS.iter()
    }

    pub fn promotion_capture_iter() -> Iter<'static, MoveType> {
        static PROMOTIONS: [MoveType; 4] = [
            MoveType::KnightPromotionCapture,
            MoveType::BishopPromotionCapture,
            MoveType::RookPromotionCapture,
            MoveType::QueenPromotionCapture,
        ];
        PROMOTIONS.iter()
    }
}

pub enum PromotionType {
    Capture,
    Push,
}
