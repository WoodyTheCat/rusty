use crate::types::{bitboard::BB, colour::Colour, piece::Piece, position::Position};

pub const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const EMPTY: &str = "8/8/8/8/8/8/8/8 w - - 0 0";

pub fn parse(position: &str) -> Position {
    let segments: Vec<String> = position.split_whitespace().map(str::to_string).collect();

    let mut bitboards: [BB; 14] = [BB::EMPTY; 14];

    let mut rank: i32 = 0;
    let mut file: i32 = 7;

    for c in segments[0].chars() {
        if c == '/' {
            file -= 1;
            rank = 0;
            continue;
        }

        if c.is_numeric() {
            rank += c.to_digit(10).unwrap() as i32;
            continue;
        }

        let piece: Piece = c.into();

        bitboards[(piece.0 - 1 + (piece.1 as i32) * 8) as usize] |= BB(1 << (file * 8 + rank));
        rank += 1;
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

    let en_passant_file: Option<i8> = if segments[3].chars().next().unwrap() != '-' {
        Some(segments[3].chars().next().unwrap() as i8)
    } else {
        None
    };

    return Position {
        bitboards,
        to_move,
        en_passant_file,
        castling_rights,
        half_moves: segments[4].parse::<i32>().unwrap(),
        full_moves: segments[5].parse::<i32>().unwrap(),
    };
}

pub fn position_to_fen(position: &Position) -> String {
    let mut fen: String = "".to_string();

    for r in (0..=7).rev() {
        let mut empty_files: i8 = 0;
        for f in 0..=7 {
            // TODO: Write a better search here ↓
            let board: Option<usize> = position
                .bitboards
                .iter()
                .position(|bb: &BB| (bb.0 & 1 << r * 8 + f) != 0);

            if let Some(board_id) = board {
                if empty_files != 0 {
                    fen += empty_files.to_string().as_str();
                    empty_files = 0;
                }
                let piece_string: String = Piece(
                    board_id as i32 & 0b0111,
                    if board_id & 8 != 0 {
                        Colour::Black
                    } else {
                        Colour::White
                    },
                )
                .into();
                fen += piece_string.as_str();
            } else {
                empty_files += 1;
            }
        }

        if empty_files != 0 {
            fen += (empty_files).to_string().as_str();
        }
        if r != 0 {
            fen += "/";
        }
    }

    // Side to move
    fen += " ";
    fen += if position.to_move == Colour::White {
        "w"
    } else {
        "b"
    };

    // Castling
    fen += " ";
    fen += if position.castling_rights[0] { "K" } else { "" };
    fen += if position.castling_rights[1] { "Q" } else { "" };
    fen += if position.castling_rights[2] { "k" } else { "" };
    fen += if position.castling_rights[3] { "q" } else { "" };
    fen += if position.castling_rights.contains(&true) == false {
        "-"
    } else {
        ""
    };

    // TODO: En-passant ↓
    fen += " -";
    // let epFileIndex: i8 = position.en_passant_file - 1;
    // let epRankIndex: i8 = if position.to_move == 0 { 5 } else { 2 };

    // if epFileIndex == -1 || !EnPassantCanBeCaptured(epFileIndex, epRankIndex, board) {
    // fen += "-";
    // } else {
    //     fen += utils::square_name_from_index(epFileIndex * 8 + epRankIndex).as_str();
    // }

    // 50 move counter
    fen += " ";
    fen += position.half_moves.to_string().as_str();

    // Full-move count (should be one at start, and increase after each move by black)
    fen += " ";
    fen += position.full_moves.to_string().as_str();

    fen
}
