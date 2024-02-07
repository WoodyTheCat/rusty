use crate::types::{
    board_state::BoardState,
    colour::Colour,
    piece::Piece,
    piece_type::PieceType,
    position::Position,
    square::{SquareIndex, SquareIndexMethods},
    EngineError,
};

pub const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn parse(notation: &str) -> Result<BoardState, EngineError> {
    let segments: Vec<String> = notation.split_whitespace().map(str::to_string).collect();

    if segments.len() != 6 {
        return Err(EngineError(String::from(
            "[fen::parse()] Too few or too may arguments to parser",
        )));
    }

    let mut position: Position = Position::default();

    let mut rank: u64 = 7;
    let mut file: u64 = 0;

    for c in segments[0].chars() {
        if c == '/' {
            rank -= 1;
            file = 0;
            continue;
        }

        if c.is_numeric() {
            file += c.to_digit(10).unwrap() as u64;
            continue;
        }

        let colour_index: Colour = if c.is_uppercase() {
            Colour::White
        } else {
            Colour::Black
        };

        let piece_index: PieceType = match c.to_ascii_uppercase() {
            'P' => PieceType::Pawn,
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'R' => PieceType::Rook,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            c => {
                return Err(EngineError(format!(
                    "[fen::parse()] Unexpected piece character: {c}"
                )))
            }
        };

        let square: u64 = rank * 8 + file;

        position.add_piece(colour_index, piece_index, square);

        // Huh?
        // position.colours_bb[colour_index] |= 1 << square;
        // position.pieces_bb[piece_index] |= 1 << square;
        // Duplicate

        file += 1;
    }

    let to_move: Colour = match segments[1].chars().next().unwrap() {
        'w' => Colour::White,
        'b' => Colour::Black,
        x => {
            return Err(EngineError(String::from(format!(
                "[fen::parse()] Unknown player identifier {x} in FEN string",
            ))));
        }
    };

    let rights: String = segments[2].clone();

    let castling_rights: [bool; 4] = [
        rights.contains("K"),
        rights.contains("Q"),
        rights.contains("k"),
        rights.contains("q"),
    ];

    let en_passant_target: Option<SquareIndex> = if segments[3].chars().next().unwrap_or('-') != '-'
    {
        Some(SquareIndex::parse(segments[3].as_str()))
    } else {
        None
    };

    Ok(BoardState {
        position,
        active_player: to_move,
        en_passant: en_passant_target,
        castling_rights,
        half_moves: segments[4].parse().unwrap(),
        full_moves: segments[5].parse().unwrap(),
    })
}

pub fn board_to_fen(board: &BoardState) -> Result<String, EngineError> {
    let mut fen: String = "".to_string();

    for rank in (0..=7).rev() {
        let mut empty_files: i8 = 0;
        for file in 0..=7 {
            let piece: Option<(PieceType, Colour)> = board.at(rank * 8 + file)?;

            if let Some((piece, colour)) = piece {
                if empty_files != 0 {
                    fen += empty_files.to_string().as_str();
                    empty_files = 0;
                }

                let piece_string: Result<String, EngineError> =
                    Piece::from_tuple(piece, colour).into();

                fen += piece_string?.as_str();
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

    Ok(fen)
}
