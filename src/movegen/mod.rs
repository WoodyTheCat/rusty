use crate::types::{
    bitboard::{PieceItr, BB},
    board_state::BoardState,
    colour::Colour,
    piece_type::PieceType,
    r#move::{MoveType::*, *},
    square::SquareIndex,
};

use self::lookup::Lookup;

mod lookup;

pub struct MoveGen {
    enemy_attack: BB,
    board: BoardState,
    pin_rays: BB,
    lookup: Lookup,
}

impl Default for MoveGen {
    fn default() -> Self {
        Self {
            enemy_attack: 0,
            board: BoardState::default(),
            pin_rays: 0,
            lookup: Lookup::default(),
        }
    }
}

const MAX_MOVES: usize = 256;

impl MoveGen {
    pub fn all_moves(&mut self, board: &BoardState) {
        let mut list: Vec<Move> = Vec::with_capacity(MAX_MOVES);

        // gen_pseudo_legal_pawn_moves(board, &mut list);
        // gen_pseudo_legal_castles(board, &mut list);

        self.gen_pseudo_legal_moves(board, &mut list, PieceType::Knight);
        self.gen_pseudo_legal_moves(board, &mut list, PieceType::Rook);
        self.gen_pseudo_legal_moves(board, &mut list, PieceType::Bishop);
        self.gen_pseudo_legal_moves(board, &mut list, PieceType::Queen);
        self.gen_pseudo_legal_moves(board, &mut list, PieceType::King);

        dbg!(list);
    }

    fn gen_pseudo_legal_moves(&self, board: &BoardState, list: &mut Vec<Move>, piece: PieceType) {
        let us: Colour = board.active_player;
        let pieces: BB = board.position.bb(us, piece);
        let valid_pieces: BB = board.position.bb_color(!us);
        let empty_squares: BB = !board.position.bb_all();

        for (square, _) in pieces.iter() {
            let destinations = match piece {
                PieceType::King | PieceType::Knight => self.lookup.moves(square, piece),
                _ => self
                    .lookup
                    .sliding_moves(square, board.position.bb_all(), piece),
            };
            let captures: u64 = destinations & valid_pieces;
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

    // fn generate_sliding_moves(&mut self, colour: Colour) {
    //     let colour_index = if colour == Colour::White { 0 } else { 6 };
    //     let mut bishops: BB = self.board.bitboards[colour_index + 3];
    //     let mut rooks: BB = self.board.bitboards[colour_index + 4];
    //     let mut queens: BB = self.board.bitboards[colour_index + 5];

    //     let mut moves: Vec<Move> = vec![];

    //     let blockers = self.board.bitboards;

    //     while !bishops.is_empty() {
    //         let index: usize = bishops.0.trailing_zeros() as usize;
    //         bishops.clear_bit(index);

    //         let moves_bb: BB = magics::get_slider_moves(Square::index(index), blockers, false);

    //         let moves_from: Vec<Move> = moves_bb.extract_moves(Square::index(index), 0);

    //         let m: Move = Move {
    //             to: Square::index(index),
    //             from: Square::index(index),
    //             flag: 0,
    //         };
    //         moves.push(m);
    //     }
    // }
}
