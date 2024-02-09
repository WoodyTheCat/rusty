// pub mod eval;
pub mod eval;
pub mod perft;
use crate::{
    movegen::MoveGen,
    types::{board_state::BoardState, chess_move::Move, EngineError},
};

pub trait Searcher {
    fn search(&mut self, board: BoardState, depth: i32)
        -> Result<(Option<Move>, i32), EngineError>;
    fn inner(
        &mut self,
        board: BoardState,
        ply: i32,
        remaining: i32,
        alpha: i32,
        beta: i32,
    ) -> Result<i32, EngineError>;
}

pub struct NegaMax(MoveGen);

impl Default for NegaMax {
    fn default() -> Self {
        Self(MoveGen::default())
    }
}

const MATE: i32 = 100000;

impl Searcher for NegaMax {
    fn search(
        &mut self,
        board: BoardState,
        depth: i32,
    ) -> Result<(Option<Move>, i32), EngineError> {
        let mut best_mv: Move = Move::NULL;
        let mut best_ev: i32 = i32::MIN;

        let mut alpha: i32 = i32::MIN + 1;
        let beta: i32 = i32::MAX;

        if depth <= 0 {
            return Err(EngineError(String::from(
                "[NegaMax::search()] Only non-zero depth values are allowed",
            )));
        }

        let moves: Vec<Move> = self.0.all_moves(&board)?;

        // childNodes := orderMoves(childNodes)

        for mv in &moves {
            let applied: BoardState = board.clone_with_move(&mv)?;
            let move_ev: i32 = -self.inner(applied, 1, depth - 1, -beta, -alpha)?;

            if move_ev > best_ev {
                best_ev = move_ev;
                best_mv = *mv;
            }

            alpha = i32::max(alpha, best_ev);

            if alpha >= beta {
                break;
            }
        }

        if moves.len() == 0 {
            return if self.0.is_check(&board, board.active_player) {
                Ok((None, 0))
            } else {
                Ok((None, -MATE))
            };
        }

        Ok((Some(best_mv), best_ev))
    }

    fn inner(
        &mut self,
        board: BoardState,
        ply: i32,
        remaining: i32,
        mut alpha: i32,
        beta: i32,
    ) -> Result<i32, EngineError> {
        // if remaining == 0 {
        //     return quiescent(alpha, beta);
        // }

        // let moves: Vec<Move> = self.gen.all_moves(&board)?;

        // let prevBestMove: Move = if plyFromRoot == 0 {
        //     bestMove
        // } else {
        //     transpositionTable.TryGetStoredMove()
        // };
        // moveOrderer.OrderMoves(
        //     prevBestMove,
        //     board,
        //     moves,
        //     moveGenerator.opponentAttackMap,
        //     moveGenerator.opponentPawnAttackMap,
        //     false,
        //     plyFromRoot,
        // );
        // Detect checkmate and stalemate when no legal moves are available

        // if moves.len() == 0 {
        //     return if self.gen.is_check(&board, board.active_player) {
        //         Ok((Move::NULL, -(MATE - ply)))
        //     } else {
        //         Ok((Move::NULL, 0))
        //     };
        // }

        // for mv in moves {
        //     let new: BoardState = board.clone_with_move(&mv);

        // let capture_type: Option<(PieceType, Colour)> = board.at(mv.to)?;
        // let is_capture: bool = capture_type.is_some();

        // Extend the depth of the search in certain interesting cases
        // int extension = 0;
        // if (numExtensions < maxExtentions)
        // {
        // 	int movedPieceType = Piece.PieceType(board.Square[m.TargetSquare]);
        // 	int targetRank = BoardHelper.RankIndex(m.TargetSquare);
        // 	if (board.IsInCheck())
        // 	{
        // 		extension = 1;
        // 	}
        // 	else if (movedPieceType == Piece.Pawn && (targetRank == 1 || targetRank == 6))
        // 	{
        // 		extension = 1;
        // 	}
        // }

        // let (best, mut eval) = self.inner(new, remaining - 1, ply + 1, -alpha - 1, -alpha)?;
        // eval = -eval;

        // Late Move Reductions:
        // Reduce the depth of the search for moves later in the move list as these are less likely to be good
        // (assuming our move ordering is doing a good job)
        // if (i >= 3 && extension == 0 && plyRemaining >= 3 && !isCapture)
        // {
        // 	const int reduceDepth = 1;
        // 	eval = -Search(plyRemaining - 1 - reduceDepth, plyFromRoot + 1, -alpha - 1, -alpha, numExtensions, move, isCapture);
        // If the evaluation turns out to be better than anything we've found so far, we'll need to redo the
        // search at the full depth to get a more accurate result. Note: this does introduce some danger that
        // we might miss a good move if the reduced search cannot see that it is good, but the idea is for
        // the increased search speed to outweigh these occasional errors.
        // 	needsFullSearch = eval > alpha;
        // }

        // Perform a full-depth search
        // if (needsFullSearch)
        // {
        // 	eval = -Search(plyRemaining - 1 + extension, plyFromRoot + 1, -beta, -alpha, numExtensions + extension, move, isCapture);
        // }

        // Move was *too* good, opponent will choose a different move earlier on to avoid this position.
        // (Beta-cutoff / Fail high)
        // if eval >= beta
        // {
        // 	// Store evaluation in transposition table. Note that since we're exiting the search early, there may be an
        // 	// even better move we haven't looked at yet, and so the current eval is a lower bound on the actual eval.
        // 	transpositionTable.StoreEvaluation(plyRemaining, plyFromRoot, beta, TranspositionTable.LowerBound, moves[i]);

        // 	// Update killer moves and history heuristic (note: don't include captures as theres are ranked highly anyway)
        // 	if (!isCapture)
        // 	{
        // 		if ply < MoveOrdering.maxKillerMovePly
        // 		{
        // 			moveOrderer.killerMoves[plyFromRoot].Add(move);
        // 		}
        // 		int historyScore = plyRemaining * plyRemaining;
        // 		moveOrderer.History[board.MoveColourIndex, moves[i].StartSquare, moves[i].TargetSquare] += historyScore;
        // 	}
        // 	if ply > 0
        // 	{
        // 		repetitionTable.TryPop();
        // 	}

        // 	searchDiagnostics.numCutOffs++;
        // 	return beta;
        // }

        // // Found a new best move in this position
        // if eval > alpha
        // {
        // 	evaluationBound = TranspositionTable.Exact;
        // 	bestMoveInThisPosition = mv;

        // 	alpha = eval;
        // 	if ply == 0
        // 	{
        // 		bestMoveThisIteration = mv;
        // 		bestEvalThisIteration = eval;
        // 		hasSearchedAtLeastOneMove = true;
        // 	}
        // }
        // }

        // todo!()

        if remaining == 0 {
            return Ok(eval::eval(&board));
        }

        let moves: Vec<Move> = self.0.all_moves(&board)?;

        if moves.len() == 0 {
            return if self.0.is_check(&board, board.active_player) {
                Ok(-MATE)
            } else {
                Ok(0)
            };
        }

        // childNodes := orderMoves(childNodes)

        let mut eval = i32::MIN;

        for mv in moves {
            let applied: BoardState = board.clone_with_move(&mv)?;
            eval = i32::max(
                eval,
                -self.inner(applied, ply + 1, remaining - 1, -beta, -alpha)?,
            );

            alpha = i32::max(alpha, eval);
            if alpha >= beta {
                break;
            }
        }

        Ok(eval)
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
