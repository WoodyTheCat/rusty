use crate::types::{
    board_state::BoardState,
    colour::Colour::{self, *},
    piece_type::PieceType::*,
};

const PAWN_VALUE: u32 = 1;
const KNIGHT_VALUE: u32 = 3;
const BISHOP_VALUE: u32 = 3;
const ROOK_VALUE: u32 = 5;
const QUEEN_VALUE: u32 = 9;

pub fn eval(board: &BoardState) -> i32 {
    let mut eval: [i32; 2] = [0; 2];
    let coefficient: i32 = if board.active_player == White { 1 } else { -1 };

    // Score based on number (and type) of pieces on board
    eval[0] = get_material_score(board, White) - get_material_score(board, Black);

    // TODO Score based on positions of pieces
    // TODO Encourage using own king to push enemy king to edge of board in winning endgame

    coefficient * eval.iter().sum::<i32>()
}

fn get_material_score(board: &BoardState, colour: Colour) -> i32 {
    let mut pieces: [u32; 5] = [0; 5];

    pieces[0] = board.position.bb(colour, Pawn).count_ones() * PAWN_VALUE;
    pieces[1] = board.position.bb(colour, Knight).count_ones() * KNIGHT_VALUE;
    pieces[2] = board.position.bb(colour, Bishop).count_ones() * BISHOP_VALUE;
    pieces[3] = board.position.bb(colour, Rook).count_ones() * ROOK_VALUE;
    pieces[4] = board.position.bb(colour, Queen).count_ones() * QUEEN_VALUE;

    pieces.iter().sum::<u32>() as i32
}

// fn mop_up_eval(&self, colour: Colour,  our_material: MaterialInfo,  their_material: MaterialInfo) -> i32
// {
//     if our_material.material_score > their_material.material_score + PAWN_VALUE * 2 && their_material.endgame_t > 0.0
//     {
//         let mut mop_up_score: i32 = 0;

//         let our_king_square: SquareIndex = self.0.position.bb(colour, King).trailing_zeros() as SquareIndex;
//         let their_king_square: SquareIndex = self.0.position.bb(colour, King).trailing_zeros() as SquareIndex;

//         mop_up_score += (14 - PrecomputedMoveData.OrthogonalDistance[our_king_square, their_king_square]) * 4;
//         mop_up_score +=

//         mop_up_score += PrecomputedMoveData.CentreManhattanDistance[opponent_king_square] * 10;
//         return (mop_up_score as f32 * their_material.endgame_t) as i32;
//     }

//      0
// }

// #[derive(Default)]
// struct MaterialInfo {
//     pub material_score: i32,
//     pub num_pawns: i32,
//     pub num_majors: i32,
//     pub num_minors: i32,
//     pub num_bishops: i32,
//     pub num_queens: i32,
//     pub num_rooks: i32,

//     pub pawns: BB,
//     pub enemy_pawns: BB,

//     pub endgame_t: f32,
// }

// impl MaterialInfo {
//     pub fn new(
//         num_pawns: i32,
//         num_knights: i32,
//         num_bishops: i32,
//         num_queens: i32,
//         num_rooks: i32,
//         our_pawns: BB,
//         their_pawns: BB,
//     ) -> Self {
//         let mut new = Self::default();

//         new.num_pawns = num_pawns;
//         new.num_bishops = num_bishops;
//         new.num_queens = num_queens;
//         new.num_rooks = num_rooks;
//         new.pawns = our_pawns;
//         new.enemy_pawns = their_pawns;

//         new.num_majors = num_rooks + num_queens;
//         new.num_minors = num_bishops + num_knights;

//         let mut material_score: i32 = 0;

//         material_score += num_pawns * PAWN_VALUE;
//         material_score += num_knights * KNIGHT_VALUE;
//         material_score += num_bishops * BISHOP_VALUE;
//         material_score += num_rooks * ROOK_VALUE;
//         material_score += num_queens * QUEEN_VALUE;

//         new.material_score = material_score;

//         // Endgame Transition (0->1)
//         const QUEEN_ENDGAME_WEIGHT: i32 = 45;
//         const ROOK_ENDGAME_WEIGHT: i32 = 20;
//         const BISHOP_ENDGAME_WEIGHT: i32 = 10;
//         const KNIGHT_ENDGAME_WEIGHT: i32 = 10;

//         let endgame_start_weight: i32 = 2 * ROOK_ENDGAME_WEIGHT
//             + 2 * BISHOP_ENDGAME_WEIGHT
//             + 2 * KNIGHT_ENDGAME_WEIGHT
//             + QUEEN_ENDGAME_WEIGHT;
//         let endgame_weight_sum: i32 = num_queens * QUEEN_ENDGAME_WEIGHT
//             + num_rooks * ROOK_ENDGAME_WEIGHT
//             + num_bishops * BISHOP_ENDGAME_WEIGHT
//             + num_knights * KNIGHT_ENDGAME_WEIGHT;

//         new.endgame_t =
//             1.0 - f32::min(1.0, endgame_weight_sum as f32 / endgame_start_weight as f32);

//         new
//     }
// }
