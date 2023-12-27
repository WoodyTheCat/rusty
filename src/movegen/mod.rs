use crate::types::{
    bitboard::{PieceItr, Shift, ToBitboard, BB},
    board_state::BoardState,
    colour::Colour,
    piece_type::PieceType::{self, *},
    r#move::{MoveType::*, *},
    square::SquareIndex,
    *,
};

use self::lookup::Lookup;

mod lookup;

pub struct MoveGen {
    board: BoardState,
    pin_rays: BB,
    lookup: Lookup,
}

impl Default for MoveGen {
    fn default() -> Self {
        Self {
            board: BoardState::default(),
            lookup: Lookup::default(),
            pin_rays: 0,
        }
    }
}

const MAX_MOVES: usize = 256;

impl MoveGen {
    pub fn all_moves(&mut self, board: &BoardState) {
        let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

        self.board = *board;

        Self::gen_pseudo_legal_pawn_moves(board, &mut list);
        // gen_pseudo_legal_castles(board, &mut list);

        self.gen_pseudo_legal_moves(board, &mut list, Knight);
        self.gen_pseudo_legal_moves(board, &mut list, Rook);
        self.gen_pseudo_legal_moves(board, &mut list, Bishop);
        self.gen_pseudo_legal_moves(board, &mut list, Queen);
        self.gen_pseudo_legal_moves(board, &mut list, King);

        dbg!(list);
    }

    fn gen_pseudo_legal_moves(&self, board: &BoardState, list: &mut Vec<Move>, piece: PieceType) {
        let us: Colour = board.active_player;
        let pieces: BB = board.position.bb(us, piece);
        let opponent_pieces: BB = board.position.bb_colour(!us);
        let empty_squares: BB = !board.position.bb_all();

        for (square, _) in pieces.iter() {
            let destinations: u64 = match piece {
                PieceType::King | PieceType::Knight => self.lookup.moves(square, piece),
                _ => {
                    let hi: u64 = board.position.bb_all();
                    self.lookup.sliding_moves(square, hi, piece)
                }
            };
            let captures: u64 = destinations & opponent_pieces;
            let quiets: u64 = destinations & empty_squares;

            Self::extract_moves(square, captures, list, Capture);
            Self::extract_moves(square, quiets, list, Quiet);
        }
    }

    fn extract_moves(from: SquareIndex, bb: BB, list: &mut Vec<Move>, kind: MoveType) {
        for (square, _) in bb.iter() {
            let m: Move = Move {
                to: square,
                from,
                kind,
            };
            list.push(m);
        }
    }

    fn gen_pseudo_legal_pawn_moves(board: &BoardState, list: &mut Vec<Move>) {
        let pawns: BB = board.position.bb(board.active_player, Pawn);
        let dir: PawnDir = PawnDir::new(board.active_player);

        Self::gen_quiet_pushes(board, list, pawns, dir);
        Self::gen_pawn_captures(board, list, pawns, dir);
        Self::gen_en_passant(board, list, pawns, dir);
        Self::gen_promotions(board, list, pawns, dir);
    }

    fn gen_quiet_pushes(board: &BoardState, list: &mut Vec<Move>, pawns: BB, dir: PawnDir) {
        let pawns: BB = pawns & !dir.rank7;
        let empty_squares: BB = !board.position.bb_all();

        let single: BB = pawns.shift(dir.north) & empty_squares;
        let double: BB = (single & dir.rank3).shift(dir.north) & empty_squares;

        extract_pawn_moves(single, dir.north, Quiet, list);
        extract_pawn_moves(double, dir.north * 2, Quiet, list);
    }

    fn gen_pawn_captures(board: &BoardState, list: &mut Vec<Move>, pawns: BB, dir: PawnDir) {
        let pawns: BB = pawns & !dir.rank7;
        let their_king: BB = board.position.bb(!board.active_player, PieceType::King);
        let valid_captures: BB = board.position.bb_colour(!board.active_player) & !their_king;

        let left_captures: BB = pawns.shift(dir.north + WEST) & valid_captures;
        let right_captures: BB = pawns.shift(dir.north + EAST) & valid_captures;

        extract_pawn_moves(left_captures, dir.north + WEST, Capture, list);
        extract_pawn_moves(right_captures, dir.north + EAST, Capture, list);
    }

    fn gen_en_passant(board: &BoardState, list: &mut Vec<Move>, pawns: BB, dir: PawnDir) {
        let target: Option<SquareIndex> = board.en_passant;

        if let Some(target) = target {
            let en_passant: BB = target.to_bitboard();
            let left_captures: BB = pawns.shift(dir.north + WEST) & en_passant;
            let right_captures: BB = pawns.shift(dir.north + EAST) & en_passant;

            extract_pawn_moves(left_captures, dir.north + WEST, EnPassantCapture, list);
            extract_pawn_moves(right_captures, dir.north + EAST, EnPassantCapture, list);
        }
    }

    fn gen_promotions(board: &BoardState, list: &mut Vec<Move>, pawns: BB, dir: PawnDir) {
        let pawns: BB = pawns & dir.rank7;
        let empty_squares: BB = !board.position.bb_all();
        let their_king: BB = board.position.bb(!board.active_player, PieceType::King);
        let valid_captures: BB = board.position.bb_colour(!board.active_player) & !their_king;

        let pushes: BB = pawns.shift(dir.north) & empty_squares;
        let left_captures: BB = pawns.shift(dir.north + WEST) & valid_captures;
        let right_captures: BB = pawns.shift(dir.north + EAST) & valid_captures;

        extract_promotions(pushes, dir.north, list, PromotionType::Push);
        extract_promotions(
            left_captures,
            dir.north + WEST,
            list,
            PromotionType::Capture,
        );
        extract_promotions(
            right_captures,
            dir.north + EAST,
            list,
            PromotionType::Capture,
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct PawnDir {
    north: i8,
    rank3: BB,
    rank7: BB,
}

impl PawnDir {
    pub fn new(colour: Colour) -> Self {
        match colour {
            Colour::White => Self {
                north: NORTH,
                rank3: RANK3,
                rank7: RANK7,
            },
            Colour::Black => Self {
                north: SOUTH,
                rank3: RANK6,
                rank7: RANK2,
            },
        }
    }
}

pub fn extract_pawn_moves(bitboard: BB, offset: i8, kind: MoveType, moves: &mut Vec<Move>) {
    for (square, _) in bitboard.iter() {
        moves.push(Move {
            to: square,
            from: (square as i8 - offset) as u64,
            kind,
        });
    }
}

fn extract_promotions(bitboard: BB, offset: i8, moves: &mut Vec<Move>, kind: PromotionType) {
    for (square, _) in bitboard.iter() {
        let iter = match kind {
            PromotionType::Capture => MoveType::promotion_capture_iter(),
            PromotionType::Push => MoveType::promotion_iter(),
        };

        for promotion in iter {
            let m = Move {
                to: square,
                from: (square as i8 - offset) as u64,
                kind: *promotion,
            };
            moves.push(m)
        }
    }
}
