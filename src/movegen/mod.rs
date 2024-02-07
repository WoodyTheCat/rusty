use std::time::Instant;

use crate::{
    magics,
    types::{
        bitboard::{PieceItr, Shift, ToBitboard, BB},
        board_state::BoardState,
        chess_move::{MoveType::*, *},
        colour::Colour::{self, *},
        piece_type::PieceType::{self, *},
        square::SquareIndex,
        *,
    },
};

use self::{lookup::Lookup, EngineError};

mod lookup;

pub struct MoveGen {
    board: BoardState,
    lookup: Lookup,
}

impl Default for MoveGen {
    fn default() -> Self {
        Self {
            board: BoardState::default(),
            lookup: Lookup::default(),
        }
    }
}

const MAX_MOVES: usize = 256;

impl MoveGen {
    pub fn trace_generate(&mut self, board: &BoardState) -> Result<Vec<Move>, EngineError> {
        println!("\n -- Trace Move Generation -- \n");

        println!("{}", board);

        // assert_eq!(
        //     &fen::parse("rnbqkb1r/1ppppppp/p6n/8/8/3P4/PPP1PPPP/RN1QKBNR w KQkq - 1 2")?,
        //     board
        // );

        let now: Instant = Instant::now();

        let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

        self.board = *board;

        Self::gen_pseudo_legal_pawn_moves(board, &mut list);
        Self::gen_pseudo_legal_castles(board, &mut list);
        self.gen_pseudo_legal_moves(board, &mut list, Knight);

        let mut king_moves = vec![];

        self.gen_pseudo_legal_moves(board, &mut king_moves, King);

        println!("King moves: {:?}", king_moves);

        list.append(&mut king_moves);

        self.gen_pseudo_legal_moves(board, &mut list, Rook);
        self.gen_pseudo_legal_moves(board, &mut list, Bishop);
        self.gen_pseudo_legal_moves(board, &mut list, Queen);

        let king_square: SquareIndex = board
            .position
            .bb(board.active_player, King)
            .trailing_zeros() as SquareIndex;

        println!("King square: {}", king_square);

        if king_square > 63 {
            return Err(EngineError(String::from(
                "[MoveGen::all_moves()] No king piece found",
            )));
        }

        println!("Before legality: {}ms", now.elapsed().as_millis());

        let blockers: BB = self.calculate_blockers(board, king_square);
        let checkers: BB = self.attacks_to(board, king_square);

        println!("Checkers: {}", checkers);
        println!("Blockers: {}", blockers);

        println!(
            "After blockers and checkers: {}ms",
            now.elapsed().as_millis()
        );

        println!("Before legal check: ");
        for mv in &list {
            println!("{}", mv);
        }

        list.retain(|mv: &Move| self.is_legal(board, mv, blockers, checkers, king_square));

        println!("After legal check: ");
        for mv in &list {
            println!("{}", mv);
        }

        println!("Elapsed: {}ms", now.elapsed().as_millis());

        println!("\n -- End Trace Move Generation -- \n");

        Ok(list)
    }

    pub fn all_moves(&mut self, board: &BoardState) -> Result<Vec<Move>, EngineError> {
        let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

        self.board = *board;

        Self::gen_pseudo_legal_pawn_moves(board, &mut list);
        Self::gen_pseudo_legal_castles(board, &mut list);
        self.gen_pseudo_legal_moves(board, &mut list, Knight);
        self.gen_pseudo_legal_moves(board, &mut list, King);

        self.gen_pseudo_legal_moves(board, &mut list, Rook);
        self.gen_pseudo_legal_moves(board, &mut list, Bishop);
        self.gen_pseudo_legal_moves(board, &mut list, Queen);

        let king_square: SquareIndex = board
            .position
            .bb(board.active_player, King)
            .trailing_zeros() as SquareIndex;

        if king_square > 63 {
            println!("{king_square:?}");
            println!("{board:x?}");
            return Err(EngineError(String::from(
                "[MoveGen::all_moves()] No king piece found",
            )));
        }

        let blockers: BB = self.calculate_blockers(board, king_square);
        let checkers: BB = self.attacks_to(board, king_square);

        list.retain(|mv: &Move| self.is_legal(board, mv, blockers, checkers, king_square));

        Ok(list)
    }

    fn gen_pseudo_legal_moves(&self, board: &BoardState, list: &mut Vec<Move>, piece: PieceType) {
        let us: Colour = board.active_player;
        let pieces: BB = board.position.bb(us, piece);
        let opponent_pieces: BB = board.position.bb_colour(!us);
        let empty_squares: BB = !board.position.bb_all();

        for (square, _) in pieces.iter() {
            let destinations: u64 = match piece {
                PieceType::King | PieceType::Knight => self.lookup.moves(square, piece),
                _ => self
                    .lookup
                    .sliding_moves(square, board.position.bb_all(), piece),
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

        Self::extract_pawn_moves(single, dir.north, Quiet, list);
        Self::extract_pawn_moves(double, dir.north * 2, Quiet, list);
    }

    fn gen_pawn_captures(board: &BoardState, list: &mut Vec<Move>, pawns: BB, dir: PawnDir) {
        let pawns: BB = pawns & !dir.rank7;
        let their_king: BB = board.position.bb(!board.active_player, PieceType::King);
        let valid_captures: BB = board.position.bb_colour(!board.active_player) & !their_king;

        let left_captures: BB = pawns.shift(dir.north + WEST) & valid_captures;
        let right_captures: BB = pawns.shift(dir.north + EAST) & valid_captures;

        Self::extract_pawn_moves(left_captures, dir.north + WEST, Capture, list);
        Self::extract_pawn_moves(right_captures, dir.north + EAST, Capture, list);
    }

    fn gen_en_passant(board: &BoardState, list: &mut Vec<Move>, pawns: BB, dir: PawnDir) {
        let target: Option<SquareIndex> = board.en_passant;

        if let Some(target) = target {
            let en_passant: BB = target.to_bitboard();
            let left_captures: BB = pawns.shift(dir.north + WEST) & en_passant;
            let right_captures: BB = pawns.shift(dir.north + EAST) & en_passant;

            Self::extract_pawn_moves(left_captures, dir.north + WEST, EnPassantCapture, list);
            Self::extract_pawn_moves(right_captures, dir.north + EAST, EnPassantCapture, list);
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

    fn attacks_to(&self, board: &BoardState, square: SquareIndex) -> BB {
        let us: Colour = board.active_player;
        let occupancies: BB = board.position.bb_all() & !board.position.bb(us, PieceType::King);

        let pawn_attacks: BB = Self::pawn_attacks(square, us);
        let rook_attacks: BB = magics::get_slider_moves(square, occupancies, true);
        let bishop_attacks: BB = magics::get_slider_moves(square, occupancies, false);
        let queen_attacks: BB = rook_attacks | bishop_attacks;
        let knight_attacks: BB = self.lookup.moves(square, PieceType::Knight);
        let king_attacks: BB = self.lookup.moves(square, PieceType::King);

        let pawns: BB = pawn_attacks & board.position.bb_piece(Pawn);

        let temp = board.position.bb_piece(Rook);

        let rooks: BB = rook_attacks & temp;
        let bishops: BB = bishop_attacks & board.position.bb_piece(Bishop);
        let queens: BB = queen_attacks & board.position.bb_piece(Queen);
        let knights: BB = knight_attacks & board.position.bb_piece(Knight);
        let king: BB = king_attacks & board.position.bb_piece(King);

        (pawns | rooks | bishops | queens | knights | king) & board.position.bb_colour(!us)
    }

    fn gen_pseudo_legal_castles(board: &BoardState, list: &mut Vec<Move>) {
        let (king_mask, queen_mask) = match board.active_player {
            White => (96, 14),
            Black => (6917529027641081856, 1008806316530991104),
        };

        let (king_rights, queen_rights) = match board.active_player {
            White => (board.castling_rights[0], board.castling_rights[1]),
            Black => (board.castling_rights[2], board.castling_rights[3]),
        };

        let occupied: BB = board.position.bb_all();

        if (occupied & king_mask) == 0 && king_rights {
            let (to, from) = match board.active_player {
                White => (6, 4),
                Black => (62, 60),
            };

            list.push(Move {
                from,
                to,
                kind: CastleKing,
            });
        }

        if (occupied & queen_mask) == 0 && queen_rights {
            let (to, from) = match board.active_player {
                White => (2, 4),
                Black => (58, 60),
            };

            list.push(Move {
                from,
                to,
                kind: CastleQueen,
            });
        }
    }

    pub fn is_legal(
        &self,
        board: &BoardState,
        mv: &Move,
        blockers: BB,
        checkers: BB,
        king_square: SquareIndex,
    ) -> bool {
        let from: SquareIndex = mv.from;
        let is_castle: bool = mv.kind == MoveType::CastleKing || mv.kind == MoveType::CastleQueen;

        if king_square == from && !is_castle {
            !self.is_attacked(board, mv.to)
        } else {
            self.is_legal_non_king_move(board, mv, blockers, checkers, king_square)
        }
    }

    fn is_legal_non_king_move(
        &self,
        board: &BoardState,
        mv: &Move,
        blockers: BB,
        checkers: BB,
        king_square: SquareIndex,
    ) -> bool {
        let num_checkers: u32 = checkers.count_ones();

        // Only king moves are valid in double check
        if num_checkers > 1 {
            return false;
        }

        if mv.kind == EnPassantCapture {
            return self.is_legal_en_passant(board, mv, king_square);
        } else if mv.kind == CastleKing || mv.kind == CastleQueen {
            return self.is_legal_castle(board, mv, num_checkers);
        }

        let pinned: bool = Self::is_absolutely_pinned(mv, blockers);

        // Only one checker, free to catch or move along ray
        if num_checkers == 1 {
            let piece_bb: BB = 1 << mv.to;
            let attacker_square: BB = checkers.trailing_zeros() as u64;
            return if mv.to == attacker_square {
                // We caught the piece
                !pinned
            } else {
                // We remained on the influence ray of the piece
                let attacking_ray: BB = self.lookup.ray_between(king_square, attacker_square);
                !pinned && ((attacking_ray & piece_bb) != 0)
            };
        }

        if !pinned {
            return true;
        }

        self.is_legal_pin_move(board, mv)
    }

    fn is_legal_en_passant(&self, board: &BoardState, mv: &Move, king_square: SquareIndex) -> bool {
        let us: Colour = board.active_player;
        let mut copy: BoardState = board.clone();

        let offset: i8 = match us {
            White => 8,
            Black => -8,
        };

        // Remove the caught piece so that pins are evaluated properly, and so the attacking piece is removed here rather than later on.
        copy.position
            .remove_piece(!us, PieceType::Pawn, (mv.to as i8 - offset) as SquareIndex);

        let temp_mv: Move = Move {
            to: mv.to,
            from: mv.from,
            kind: Capture,
        };

        let blockers: BB = self.calculate_blockers(&copy, king_square);
        let checkers: BB = self.attacks_to(&copy, king_square);
        let is_legal =
            self.is_legal_non_king_move(&copy, &temp_mv, blockers, checkers, king_square);

        // Revert
        copy.position
            .add_piece(!us, Pawn, (mv.to as i8 - offset) as SquareIndex);

        is_legal
    }

    fn is_legal_castle(&self, board: &BoardState, mv: &Move, num_checkers: u32) -> bool {
        if num_checkers != 0 {
            return false;
        }

        let squares: Vec<SquareIndex> = match mv.kind {
            CastleKing => match board.active_player {
                White => vec![5, 6],
                Black => vec![61, 62],
            },
            CastleQueen => match board.active_player {
                White => vec![2, 3],
                Black => vec![58, 59],
            },
            _ => vec![],
        };

        for square in squares {
            if self.is_attacked(board, square) {
                return false;
            }
        }

        true
    }

    fn is_legal_pin_move(&self, board: &BoardState, mv: &Move) -> bool {
        let ray = self.lookup.between(mv.to, mv.from);
        let overlap = ray & board.position.bb(board.active_player, PieceType::King);

        overlap != 0
    }

    fn is_absolutely_pinned(mv: &Move, blockers: BB) -> bool {
        let piece_bb: BB = 1 << mv.from;

        (blockers & piece_bb) != 0
    }

    fn is_attacked(&self, board: &BoardState, square: SquareIndex) -> bool {
        let us: Colour = board.active_player;

        if Self::pawn_attacks(square, us) & board.position.bb(!us, PieceType::Pawn) != 0 {
            return true;
        }

        let occupancies: BB = board.position.bb_all() & !board.position.bb(us, PieceType::King);

        let opponent_queen: BB = board.position.bb(!us, Queen);
        let opponent_ortho: BB = board.position.bb(!us, Rook) | opponent_queen;
        let opponent_diag: BB = board.position.bb(!us, Bishop) | opponent_queen;

        let attacked_by_rook: bool =
            self.lookup
                .sliding_moves(square, occupancies, PieceType::Rook)
                & opponent_ortho
                != 0;

        if attacked_by_rook {
            return true;
        }

        let attacked_by_bishop = self
            .lookup
            .sliding_moves(square, occupancies, PieceType::Bishop)
            & opponent_diag
            != 0;
        if attacked_by_bishop {
            return true;
        }

        let attacked_by_knight = self.lookup.moves(square, PieceType::Knight)
            & board.position.bb(!us, PieceType::Knight)
            != 0;
        if attacked_by_knight {
            return true;
        }

        // attacked by king
        let attacked_by_king = self.lookup.moves(square, King) & board.position.bb(!us, King) != 0;

        attacked_by_king
    }

    fn pawn_attacks(square: SquareIndex, colour: Colour) -> BB {
        let bb: BB = 1 << square;
        match colour {
            White => bb.shift(NORTH + WEST) | bb.shift(NORTH + EAST),
            Black => bb.shift(SOUTH + WEST) | bb.shift(SOUTH + EAST),
        }
    }

    fn calculate_blockers(&self, board: &BoardState, king_square: SquareIndex) -> BB {
        let us: Colour = board.active_player;

        if king_square > 63 {
            println!("UH-OH");
        }

        let opponent_queen: BB = board.position.bb(!us, Queen);
        let opponent_ortho: BB = board.position.bb(!us, Rook) | opponent_queen;
        let opponent_diag: BB = board.position.bb(!us, Bishop) | opponent_queen;

        let attacks_rooks = self.lookup.piece_influence(true, king_square) & opponent_ortho;
        let attacks_bishops = self.lookup.piece_influence(false, king_square) & opponent_diag;

        let snipers: BB = (attacks_rooks | attacks_bishops) & !board.position.bb(us, King);
        let occupancy: BB = board.position.bb_all();

        let mut blockers: BB = 0;

        for (i, _) in snipers.iter() {
            let ignore = 1 << i;
            let potential_blockers = self.lookup.ray_between(king_square, i)
                & occupancy
                & !board.position.bb(us, King)
                & !ignore;

            if potential_blockers.count_ones() == 1 {
                blockers |= potential_blockers;
            }
        }

        blockers
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
            White => Self {
                north: NORTH,
                rank3: RANK3,
                rank7: RANK7,
            },
            Black => Self {
                north: SOUTH,
                rank3: RANK6,
                rank7: RANK2,
            },
        }
    }
}

fn extract_promotions(bitboard: BB, offset: i8, moves: &mut Vec<Move>, kind: PromotionType) {
    for (square, _) in bitboard.iter() {
        let iter = match kind {
            PromotionType::Capture => MoveType::promotion_capture_iter(),
            PromotionType::Push => MoveType::promotion_iter(),
        };

        for promotion in iter {
            let m: Move = Move {
                to: square,
                from: (square as i8 - offset) as u64,
                kind: *promotion,
            };
            moves.push(m)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        fen,
        movegen::MoveGen,
        types::{
            bitboard::BB,
            board_state::BoardState,
            chess_move::{
                Move,
                MoveType::{self, *},
            },
            piece_type::PieceType::*,
            square::{
                Square::{self, *},
                SquareIndex,
            },
            *,
        },
    };

    #[allow(dead_code)]
    fn make_move(to: Square, from: Square) -> Move {
        Move {
            to: to as SquareIndex,
            from: from as SquareIndex,
            kind: Quiet,
        }
    }

    fn king_square(board: &BoardState) -> SquareIndex {
        let us = board.active_player;
        board.position.bb(us, King).trailing_zeros() as SquareIndex
    }

    #[test]
    fn cannot_capture_checking_piece_while_pinned() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("2r5/8/8/2B5/8/8/8/2K3r1 w - - 0 1")
            .ok()
            .unwrap();

        let mv: Move = make_move(G1, C5);

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn cannot_block_checking_piece_while_pinned() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("2r5/8/8/2B5/8/8/8/2K4r w - - 0 1").ok().unwrap();

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        let mv = make_move(G1, C5);
        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn cannot_move_pinned_piece() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/8/8/8/1K1N3r/8/8/8 w - - 0 1").ok().unwrap();

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        let mv = make_move(C6, D4);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            false
        );

        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        let mv = make_move(C2, D4);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn can_move_piece_along_pinned_ray() {
        let gen: MoveGen = MoveGen::default();
        let pos = fen::parse("8/8/8/8/8/8/1K3R1r/8 w - - 0 1").ok().unwrap();

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        // Move towards pinner without capture
        let mv = make_move(G2, F2);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            true
        );

        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        // Move towards pinner with capture
        let mv = make_move(H2, F2);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            true
        );

        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        // Move away from pinner
        let mv = make_move(E2, F2);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            true
        );

        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        // Moving off pin is illegal
        let mv = make_move(F1, F2);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn cannot_move_non_king_with_multiple_checkers() {
        let gen: MoveGen = MoveGen::default();
        let pos = fen::parse("8/1r6/8/8/3N4/8/1K5r/8 w - - 0 1").ok().unwrap();

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        let mv = make_move(D4, C6);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn can_move_king() {
        let gen: MoveGen = MoveGen::default();
        let pos = fen::parse("8/8/8/8/8/8/1K5r/8 w - - 0 1").ok().unwrap();

        let mv = make_move(A2, B2);
        assert_eq!(gen.is_attacked(&pos, mv.to), true);

        let mv = make_move(B1, B2);
        assert_eq!(gen.is_attacked(&pos, mv.to), false);
    }

    #[test]
    fn cannot_block_using_xray() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/8/8/8/8/3B4/3K3r/8 w - - 0 1").ok().unwrap();

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        let mv = make_move(C2, D3);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            false
        );

        let mv = make_move(E2, D3);
        assert_eq!(
            gen.is_legal_non_king_move(&pos, &mv, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn king_cannot_castle_through_check() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/8/8/8/8/3b4/8/R3K2R w KQ - 0 1").ok().unwrap();
        let _mv: Move = make_move(C2, D3);
        let mv: Move = Move {
            to: 0,
            from: 0,
            kind: MoveType::CastleKing,
        };
        assert_eq!(gen.is_legal_castle(&pos, &mv, 0), false);
    }

    #[test]
    fn king_cannot_castle_in_check() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/8/8/8/8/2b5/8/R3K2R w KQ - 0 1").ok().unwrap();
        let mv: Move = Move {
            to: 0,
            from: 0,
            kind: MoveType::CastleKing,
        };
        assert_eq!(gen.is_legal_castle(&pos, &mv, 1), false);
    }

    #[test]
    pub fn en_passant_discovered_check() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/8/8/K2Pp2q/8/8/8/8 w - e6 0 1").ok().unwrap();
        let mv: Move = Move {
            to: E6 as SquareIndex,
            from: D5 as SquareIndex,
            kind: MoveType::EnPassantCapture,
        };

        let king_square = king_square(&pos);

        assert_eq!(gen.is_legal_en_passant(&pos, &mv, king_square), false);
    }

    #[test]
    fn en_passant_out_of_check() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/8/8/3Pp2q/3K4/8/8/8 w - e6 0 1").ok().unwrap();
        let mv: Move = Move {
            to: E6 as SquareIndex,
            from: D5 as SquareIndex,
            kind: EnPassantCapture,
        };

        let king_square = king_square(&pos);

        assert_eq!(gen.is_legal_en_passant(&pos, &mv, king_square), true);
    }

    #[test]
    fn random_fen_1() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/2p5/3p4/KP5r/5R1k/8/4P1P1/8 b - - 0 1")
            .ok()
            .unwrap();
        let mv: Move = Move {
            to: G5 as SquareIndex,
            from: H4 as SquareIndex,
            kind: MoveType::Quiet,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn random_fen_2() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("rnbqk1nr/pppp1ppp/8/4p3/1b1P4/P7/1PP1PPPP/RNBQKBNR w KQkq - 0 1")
                .ok()
                .unwrap();
        let mv: Move = Move {
            to: B4 as SquareIndex,
            from: A3 as SquareIndex,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn random_fen_3() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1PPBBPPP/R3K2R w KQkq - 0 1")
                .ok()
                .unwrap();
        let mv = Move {
            to: A3 as SquareIndex,
            from: B4 as SquareIndex,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn random_fen_4() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q1p/1PPBBPPP/R3K2R w KQkq a3 0 1")
                .ok()
                .unwrap();
        let mv = Move {
            to: A3 as SquareIndex,
            from: B4 as SquareIndex,
            kind: MoveType::EnPassantCapture,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn castle_through_knight_attacks() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("r3k2r/p1ppqpb1/bnN1pnp1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1")
                .ok()
                .unwrap();
        let mv: Move = Move {
            to: C8 as SquareIndex,
            from: E8 as SquareIndex,
            kind: MoveType::CastleQueen,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn castle_through_more_knight_attacks() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("r3k2r/p1ppqpb1/bn2pnN1/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1")
                .ok()
                .unwrap();
        let mv: Move = Move {
            to: G8 as SquareIndex,
            from: E8 as SquareIndex,
            kind: MoveType::CastleKing,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn castle_through_even_more_knight_attacks() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("r3k2r/p1ppqNb1/bn2pn2/3P4/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1")
                .ok()
                .unwrap();
        let mv: Move = Move {
            to: C8 as SquareIndex,
            from: E8 as SquareIndex,
            kind: MoveType::CastleQueen,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn queen_captures() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("r3k2r/p1ppqpb1/1n2pnp1/3PN3/1p2P3/2N2Q1p/PPPBbPPP/R2K3R w KQkq - 0 1")
                .ok()
                .unwrap();
        let mv: Move = Move {
            to: E2 as SquareIndex,
            from: F3 as SquareIndex,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn capture_checker_behind_ray() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("r3k2r/p1pp1pb1/bn2pnp1/1B1PN3/1pq1P3/2N2Q1p/PPPB1PPP/R4K1R w kq - 4 3")
                .ok()
                .unwrap();
        let mv: Move = Move {
            to: C4 as SquareIndex,
            from: B5 as SquareIndex,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn challenge() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState =
            fen::parse("r6r/1bp2pP1/R2qkn2/1P6/1pPQ4/1B3N2/1B1P2p1/4K2R b K c3 0 1")
                .ok()
                .unwrap();
        let mv: Move = Move {
            to: C3 as SquareIndex,
            from: B4 as SquareIndex,
            kind: MoveType::EnPassantCapture,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn castle_pawn_attacks() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/8/8/8/8/8/6p1/4K2R w K - 0 1").ok().unwrap();
        let mv: Move = Move {
            to: E1 as SquareIndex,
            from: G1 as SquareIndex,
            kind: MoveType::CastleKing,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            false
        );
    }

    #[test]
    fn captures_attacker_on_ray() {
        let gen: MoveGen = MoveGen::default();
        let pos: BoardState = fen::parse("8/8/8/8/8/8/1K1R2r1/8 w - - 0 1").ok().unwrap();
        let mv: Move = Move {
            to: G2 as SquareIndex,
            from: D2 as SquareIndex,
            kind: MoveType::Capture,
        };

        let king_square = king_square(&pos);
        let blockers = gen.calculate_blockers(&pos, king_square);
        let checkers = gen.attacks_to(&pos, king_square);

        assert_eq!(
            gen.is_legal(&pos, &mv, blockers, checkers, king_square),
            true
        );
    }

    #[test]
    fn castles_no_obstruction() {
        let pos: BoardState = fen::parse("8/8/8/8/8/8/8/R3K2R w KQ - 0 1").ok().unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn no_castles_with_obstruction() {
        let pos: BoardState = fen::parse("8/8/8/8/8/8/8/R3KB1R w KQ - 0 1").ok().unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);

        let pos: BoardState = fen::parse("8/8/8/8/8/8/8/R1B1K2R w KQ - 0 1").ok().unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn no_castles_without_rights() {
        let pos: BoardState = fen::parse("8/8/8/8/8/8/8/R3K2R w K - 0 1").ok().unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_castles(&pos, &mut list);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn black_queenside_castle() {
        let pos: BoardState =
            fen::parse("r3k2r/p1ppq1b1/bn2pn2/3P2N1/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1")
                .ok()
                .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_castles(&pos, &mut list);
        let _m1 = list.get(0).unwrap();
        let _m2 = list.get(1).unwrap();
        assert_eq!(list.len(), 2);
    }
    #[test]
    fn gen_random_pawn_moves1() {
        let pos: BoardState = fen::parse("3N4/1p1N2R1/kp3PQp/8/p2P4/B7/6p1/b2b2K1 w - - 0 1")
            .ok()
            .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn gen_random_pawn_moves2() {
        let pos: BoardState = fen::parse("8/1P5n/1NB5/2KbQ1P1/2n5/p4R2/Pp2p3/1k2b3 w - - 0 1")
            .ok()
            .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn gen_random_pawn_moves3() {
        let pos: BoardState = fen::parse("3r2r1/P6b/q2pKPk1/4P3/1p1P1R2/5n2/1B2N3/8 w - - 0 1")
            .ok()
            .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 7);
    }

    #[test]
    fn gen_random_pawn_moves4() {
        let pos: BoardState = fen::parse("8/4PP2/2n3p1/6P1/2p1p2q/K1P3k1/b1p1P1B1/2R5 w - - 0 1")
            .ok()
            .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 9);
    }

    #[test]
    fn gen_random_pawn_moves5() {
        let pos: BoardState = fen::parse("3bBr2/8/P7/2PPp3/1N6/3bK2R/2Pp4/1k1qN3 w - d6 0 1")
            .ok()
            .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 7);
    }

    #[test]
    fn extract_basic_pawn_moves() {
        let b = RANK2;
        let mut moves: Vec<Move> = Vec::new();
        MoveGen::extract_pawn_moves(b, NORTH, Quiet, &mut moves);
        assert_eq!(moves.len(), 8);
        assert_eq!(moves.get(0).unwrap().to, 8);
        assert_eq!(moves.get(1).unwrap().to, 9);
    }

    /// Pawns moves for FEN
    /// 3N4/1p1N2R1/kp3PQp/8/p2P4/B7/6p1/b2b2K1 w - - 0 1
    #[test]
    fn extract_random_pawns() {
        let b: BB = 0x200008000000;
        let mut moves: Vec<Move> = Vec::new();
        MoveGen::extract_pawn_moves(b, NORTH, Quiet, &mut moves);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves.get(0).unwrap().to, D4 as SquareIndex);
        assert_eq!(moves.get(0).unwrap().from, D3 as SquareIndex);
        assert_eq!(moves.get(1).unwrap().to, F6 as SquareIndex);
        assert_eq!(moves.get(1).unwrap().from, F5 as SquareIndex);
    }

    #[test]
    fn gen_en_passant() {
        let pos: BoardState = fen::parse("8/8/3p4/KPp4r/5R1k/8/8/8 w - c6 0 1")
            .ok()
            .unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn gen_a3_to_b4() {
        let pos: BoardState = fen::parse("8/8/8/8/1p6/P7/8/8 w - - 0 1").ok().unwrap();
        let mut list: Vec<Move> = Vec::with_capacity(256);
        MoveGen::gen_pseudo_legal_pawn_moves(&pos, &mut list);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn king_move_removed() {
        let board = fen::parse("rnbqkb1r/1ppppppp/p6n/8/8/3P4/PPP1PPPP/RN1QKBNR w KQkq - 1 2")
            .ok()
            .unwrap();
        let gen = MoveGen::default();
        assert_eq!(
            true,
            gen.is_legal(
                &board,
                &Move {
                    from: 4,
                    to: 11,
                    kind: Quiet
                },
                0,
                0,
                4
            )
        );
    }
}
