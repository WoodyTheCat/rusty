// use std::time::Instant;

use crate::{
    movegen::MoveGen,
    types::{
        board_state::BoardState,
        chess_move::{Move, MoveType::*},
        square::{Square::*, SquareIndex},
        EngineError,
    },
};

#[derive(Default)]
pub struct Perft {
    gen: MoveGen,
    pub count: usize,
}

impl Perft {
    pub fn verbose(&mut self, board: &BoardState, depth: i32) -> Result<usize, EngineError> {
        let moves: Vec<Move> = self.gen.all_moves(board)?;

        // if depth <= 1 {
        //     println!("Possible moves: {}", moves.len());
        //     return moves.len();
        // }

        let mut sum: usize = 0;

        let temp: Move = Move {
            from: G8 as SquareIndex,
            to: H6 as SquareIndex,
            kind: Normal,
        };

        let mut trace = false;

        for mv in &moves {
            let new_board: BoardState = board.clone_with_move(mv)?;

            if *mv == temp {
                println!("HIT ↓");
                trace = true;
            }

            let subtotal: usize = self.perft_inner(&new_board, depth - 1, trace)?;

            println!("{}: {}", mv, subtotal);

            trace = false;

            sum += subtotal;
        }

        // drop(temp);

        println!("\nNodes searched: {}", sum);
        println!("Moves searched: {}", moves.len());

        Ok(sum)
    }

    // pub fn perft(&mut self, depth: i32) {
    //     // let now: Instant = Instant::now();
    //     self.count += self.perft_inner(&fen::parse(fen::START), depth, false);

    //     // println!("Time elapsed: {}ms", now.elapsed().as_millis());
    //     println!("Nodes: {}", self.count);
    // }

    fn perft_inner(
        &mut self,
        board: &BoardState,
        depth: i32,
        trace: bool,
    ) -> Result<usize, EngineError> {
        let moves: Vec<Move>;

        if trace {
            moves = self.gen.trace_generate(board)?;
        } else {
            moves = self.gen.all_moves(board)?;
        }

        if depth <= 1 {
            return Ok(moves.len());
        }

        let mut sum: usize = 0;
        for mv in &moves {
            let new_board: BoardState = board.clone_with_move(mv)?;
            sum += self.perft_inner(&new_board, depth - 1, trace)?;
        }
        Ok(sum)
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}
