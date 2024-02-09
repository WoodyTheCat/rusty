use crate::fen;

use super::{
    bitboard::{ToBitboard, BB},
    chess_move::{Move, MoveType::*},
    colour::Colour,
    piece::Piece,
    piece_type::PieceType::{self, *},
    position::Position,
    square::{Square::*, SquareIndex},
    EngineError,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardState {
    pub position: Position,
    pub active_player: Colour,
    pub en_passant: Option<SquareIndex>,
    pub castling_rights: [bool; 4],
    pub half_moves: i32,
    pub full_moves: i32,
}

impl Default for BoardState {
    fn default() -> Self {
        BoardState {
            position: Position::default(),
            castling_rights: [false; 4],
            en_passant: None,
            active_player: Colour::White,
            half_moves: 0,
            full_moves: 0,
        }
    }
}

impl BoardState {
    pub fn at(&self, square: SquareIndex) -> Result<Option<(PieceType, Colour)>, EngineError> {
        let sq: BB = square.to_bitboard();
        let pos: Position = self.position;
        let piece: Option<usize> = pos.pieces_bb.iter().position(|&bb| (bb & sq) != 0);
        let colour: Option<usize> = pos.colours_bb.iter().position(|&bb| (bb & sq) != 0);

        match (piece, colour) {
            (Some(p), Some(c)) => Ok(Some((PieceType::index(p), Colour::index(c)))),
            (None, None) => Ok(None),
            e => Err(EngineError(String::from(format!(
                "[BoardState.at()] Desync between colour and piece bitboard sets {e:?}",
            )))),
        }
    }

    pub fn clone_with_move(&self, mv: &Move) -> Result<BoardState, EngineError> {
        let mut new_pos: BoardState = *self;
        new_pos.make_move(mv)?;
        Ok(new_pos)
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<(), EngineError> {
        if mv.kind == Null {
            return Ok(());
        }

        let Some(kind) = self.position.type_at(mv.from) else {
            return Err(EngineError(String::from(format!(
                "No piece at 'from' whilst executing move: {mv:?} \n {self}",
            ))));
        };
        let us: Colour = self.active_player;

        if kind == King {
            for i in 0..1 {
                self.castling_rights[i + if self.active_player == Colour::White {
                    0
                } else {
                    2
                }] = false;
            }
        }

        if kind == Pawn && ((mv.to as i8) - (mv.from as i8)).abs() == 16 {
            self.en_passant = Some(
                (mv.to as i8
                    + if self.active_player == Colour::White {
                        -8
                    } else {
                        8
                    }) as u64,
            );
        } else {
            self.en_passant = None;
        }

        if kind == Rook {
            self.make_rook_move(&mv);
        }

        if kind == King {
            self.move_king(self.active_player);
        }

        let ep_offset: i8 = match us {
            Colour::White => 8,
            Colour::Black => -8,
        };

        if mv.kind == Normal {
            if self.at(mv.to)?.is_some() {
                self.capture(mv, us);
            } else {
                self.position.remove_piece(us, kind, mv.from);
                self.position.add_piece(us, kind, mv.to);
            }
        } else if mv.kind == EnPassantCapture {
            self.position.remove_piece(us, kind, mv.from);
            self.position
                .remove_piece(!us, kind, (mv.to as i8 - ep_offset) as SquareIndex);
            self.position.add_piece(us, kind, mv.to);
        } else if mv.is_promotion_capture() {
            let capture_kind: PieceType = self.position.type_at(mv.to).unwrap();

            if capture_kind == Rook {
                self.capture_rook(&mv, self.active_player);
            }

            self.position.remove_piece(us, kind, mv.from);
            self.position.remove_piece(!us, capture_kind, mv.to);

            // println!("{:x?} Capture", self.position);

            let add: PieceType = mv.promoted_piece().unwrap();
            self.position.add_piece(us, add, mv.to);
        } else if mv.is_promotion() {
            // println!("{:x?} Non-Capture", self.position);
            self.position.remove_piece(us, kind, mv.from);
            let add = mv.promoted_piece().unwrap();
            self.position.add_piece(us, add, mv.to);
        } else if mv.is_castle() {
            self.position.castle(mv.kind, self.active_player);
            self.castling_rights[self.active_player as usize * 2] = false;
            self.castling_rights[self.active_player as usize * 2 + 1] = false;
        }

        self.switch();

        Ok(())
    }

    fn make_rook_move(&mut self, mv: &Move) {
        if self.active_player == Colour::White {
            if mv.from == 7 {
                self.castling_rights[0] = false;
            }
            if mv.from == 0 {
                self.castling_rights[1] = false;
            }
        } else {
            if mv.from == 63 {
                self.castling_rights[2] = false;
            }
            if mv.from == 56 {
                self.castling_rights[3] = false;
            }
        }
    }

    fn capture(&mut self, mv: &Move, active: Colour) {
        let captured: PieceType = self.position.type_at(mv.to).unwrap();
        if captured == PieceType::Rook {
            self.capture_rook(&mv, active);
        }
        self.position.capture(mv, self.active_player);
    }

    fn capture_rook(&mut self, mv: &Move, active: Colour) {
        match active {
            Colour::White => {
                if mv.to == H8 as SquareIndex {
                    self.castling_rights[2] = false;
                } else if mv.to == A8 as SquareIndex {
                    self.castling_rights[3] = false;
                }
            }
            Colour::Black => {
                if mv.to == H1 as SquareIndex {
                    self.castling_rights[0] = false;
                } else if mv.to == A1 as SquareIndex {
                    self.castling_rights[1] = false;
                }
            }
        }
    }

    fn move_king(&mut self, active: Colour) {
        match active {
            Colour::White => {
                self.castling_rights[0] = false;
                self.castling_rights[1] = false;
            }
            Colour::Black => {
                self.castling_rights[2] = false;
                self.castling_rights[3] = false;
            }
        }
    }

    fn switch(&mut self) {
        self.active_player = !self.active_player;
    }
}

impl std::fmt::Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..=7).rev() {
            writeln!(f, " +---+---+---+---+---+---+---+---+")?;
            for j in 0..=7 {
                let piece: Option<(PieceType, Colour)> =
                    self.at(i * 8 + j).map_err(Into::<std::fmt::Error>::into)?;

                if let Some((piece, colour)) = piece {
                    write!(f, " | {:?}", Piece::from_tuple(piece, colour))?;
                } else {
                    write!(f, " |  ")?;
                }
            }
            write!(f, " | {}\n", i + 1)?;
        }

        writeln!(f, " +---+---+---+---+---+---+---+---+")?;
        writeln!(f, "   a   b   c   d   e   f   g   h  ")?;
        writeln!(
            f,
            "\n FEN: {}",
            fen::board_to_fen(&self).map_err(Into::<std::fmt::Error>::into)?
        )?;
        Ok(())
    }
}
