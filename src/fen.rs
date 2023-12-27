use crate::types::{
    board_state::BoardState,
    colour::Colour,
    piece::Piece,
    piece_type::PieceType,
    position::Position,
    square::{SquareIndex, SquareIndexMethods},
};

pub const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const EMPTY: &str = "8/8/8/8/8/8/8/8 w - - 0 0";

pub fn parse(notation: &str) -> BoardState {
    let segments: Vec<String> = notation.split_whitespace().map(str::to_string).collect();

    let mut position: Position = Position::default();

    let mut rank: i32 = 7;
    let mut file: i32 = 0;

    for c in segments[0].chars() {
        if c == '/' {
            rank -= 1;
            file = 0;
            continue;
        }

        if c.is_numeric() {
            file += c.to_digit(10).unwrap() as i32;
            continue;
        }

        let colour_index: usize = if c.is_uppercase() { 0 } else { 1 };
        let piece_index: usize = match c.to_uppercase().to_string().as_str() {
            "P" => 0,
            "N" => 1,
            "B" => 2,
            "R" => 3,
            "Q" => 4,
            "K" => 5,
            i => panic!("Unknown piece index: {}", i),
        };

        let index = rank * 8 + file;

        position.colours_bb[colour_index] |= 1 << index;
        position.pieces_bb[piece_index] |= 1 << index;

        file += 1;
    }

    let to_move: Colour = match segments[1].chars().next().unwrap() {
        'w' => Colour::White,
        'b' => Colour::Black,
        x => panic!("Unknown player identifier {x} in FEN string"),
    };

    let castling_rights: [bool; 4] = [
        segments[2].contains("K"),
        segments[2].contains("Q"),
        segments[2].contains("k"),
        segments[2].contains("q"),
    ];

    let en_passant_target: Option<SquareIndex> = if segments[3].chars().next().unwrap_or('-') != '-'
    {
        Some(SquareIndex::parse(segments[3].as_str()))
    } else {
        None
    };

    return BoardState {
        position,
        active_player: to_move,
        en_passant: en_passant_target,
        castling_rights,
        half_moves: segments[4].parse().unwrap(),
        full_moves: segments[5].parse().unwrap(),
    };
}

pub fn board_to_fen(board: &BoardState) -> String {
    let mut fen: String = "".to_string();

    for rank in (0..=7).rev() {
        let mut empty_files: i8 = 0;
        for file in 0..=7 {
            let piece: Option<(PieceType, Colour)> = board.at(rank * 8 + file);

            if let Some((piece, colour)) = piece {
                if empty_files != 0 {
                    fen += empty_files.to_string().as_str();
                    empty_files = 0;
                }

                let piece_string: String = Piece::from_tuple(piece, colour).into();
                fen += piece_string.as_str();
            } else {
                empty_files += 1;
            }
        }

        if empty_files != 0 {
            fen += (empty_files).to_string().as_str();
        }
        if rank != 0 {
            fen += "/";
        }
    }

    // Side to move
    fen += " ";
    fen += if board.active_player == Colour::White {
        "w"
    } else {
        "b"
    };

    // Castling
    fen += " ";
    fen += if board.castling_rights[0] { "K" } else { "" };
    fen += if board.castling_rights[1] { "Q" } else { "" };
    fen += if board.castling_rights[2] { "k" } else { "" };
    fen += if board.castling_rights[3] { "q" } else { "" };
    fen += if board.castling_rights.contains(&true) == false {
        "-"
    } else {
        ""
    };

    // En-passant
    fen += " ";

    if let Some(square) = board.en_passant {
        fen += square.to_algebraic().as_str()
    } else {
        fen += "-";
    }

    // 50 move counter
    fen += " ";
    fen += board.half_moves.to_string().as_str();

    // Full-move count (should be one at start, and increase after each move by black)
    fen += " ";
    fen += board.full_moves.to_string().as_str();

    fen
}
