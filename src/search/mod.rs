// pub mod eval;
pub mod perft;

use crate::movegen::MoveGen;

pub struct NegaMax {
    gen: MoveGen,
    stats: Stats,
}

impl Default for NegaMax {
    fn default() -> Self {
        Self {
            gen: MoveGen::default(),
            stats: Stats::default(),
        }
    }
}

// impl NegaMax {
//     pub fn negamax(
//         &mut self,
//         pos: &mut BoardState,
//         mut alpha: isize,
//         beta: isize,
//         depth: usize,
//     ) -> (i32, Move) {
//         let eval = eval(pos);

//         if self.time_expired() {
//             return eval;
//         }

//         if depth == 0 {
//             return eval;
//         }

//         if eval >= beta {
//             return beta;
//         } else if eval > alpha {
//             alpha = eval;
//         };

//         let is_attacked: bool = self.gen.is_attacked(pos, king_square(pos));

//         let mut moves: Vec<Move> = self.gen.all_moves(pos);

//         if moves.is_empty() && is_attacked {
//             return self.no_move_eval(pos, depth).eval;
//         }

//         for mv in &mut moves {
//             let mut new_pos = pos.clone_with_move(*mv);
//             let eval = -self.negamax(&mut new_pos, -beta, -alpha, depth - 1);
//             if eval >= beta {
//                 return beta;
//             }

//             if eval > alpha {
//                 alpha = eval;
//             }
//         }

//         alpha
//     }

//     /// Returns (relative eval, best move)
//     pub fn search(&mut self) -> (i32, Move) {}
// }

struct Stats {
    nodes: u64,
    leaf_nodes: u64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            nodes: 0,
            leaf_nodes: 0,
        }
    }
}
