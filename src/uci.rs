use std::io::{stdin, BufRead};

use crate::{
    fen,
    movegen::MoveGen,
    search::{perft::Perft, NegaMax},
    types::{board_state::BoardState, chess_move::Move, EngineError},
};

pub fn uci_loop() -> Result<(), EngineError> {
    let mut board: BoardState = fen::parse(fen::START).ok().unwrap();
    let mut searcher: NegaMax = NegaMax::default();

    loop {
        match uci_execute(&mut board, &mut searcher) {
            Err(e) => {
                dbg!(e);
            }
            Ok(_) => {}
        };
    }
}

pub fn uci_execute(board: &mut BoardState, searcher: &mut NegaMax) -> Result<(), EngineError> {
    let mut buffer: String = String::new();
    stdin().lock().read_line(&mut buffer).unwrap();
    let input: &str = buffer.trim_matches(char::is_whitespace);

    let (command, rest) = input.split_once(char::is_whitespace).unwrap_or((input, ""));

    match command {
        "uci" => init_uci(),
        "pos" => {
            *board = update_board(rest)?;
        }

        "go" => go(board, searcher, rest),
        "moves" => moves(&board),
        "isready" => println!("readyok"),
        "ucinewgame" => {}
        "d" => println!("\n{}", board),
        "perft" => {
            let mut perft: Perft = Perft::default();
            perft.verbose(
                &board,
                rest.parse::<i32>().map_err(Into::<EngineError>::into)?,
            )?;
        }
        _ => println!("Command not understood"),
    }

    Ok(())
}

fn init_uci() {
    println!("id name Rusty");
    println!("id author Fergus Rorke");
    println!("uciok");
}

fn update_board<'b>(args: &str) -> Result<BoardState, EngineError> {
    let (keyword, rest) = args.split_once(char::is_whitespace).unwrap_or((args, ""));

    if keyword == "start" {
        let mut pos: BoardState = fen::parse(fen::START)?;
        let tokens: Vec<&str> = rest.split(char::is_whitespace).collect();

        if tokens[0] == "move" {
            let mut gen: MoveGen = MoveGen::default();
            for notation in tokens[1..].iter() {
                apply_move(&mut pos, *notation, &mut gen)?;
            }
        }

        Ok(pos)
    } else {
        fen::parse(rest)
    }
}

fn go(pos: &mut BoardState, searcher: &mut NegaMax, data: &str) {
    // let movetime = data[2].parse::<u128>().unwrap();
    // searcher.move_time((movetime / 1000) - 1);
    // searcher.move_time(movetime);
    // let mv = searcher.best_move_depth(pos, 15);
    // println!("eval: {}", mv.eval);
    // println!("static eval: {}", eval(pos));
    // println!("bestmove {}", mv.mv.to_algebraic());
}

fn moves(board: &BoardState) {
    let mut gen: MoveGen = MoveGen::default();
    let move_list: Vec<Move> = gen.trace_generate(board).ok().unwrap();
}

fn apply_move(pos: &mut BoardState, notation: &str, gen: &mut MoveGen) -> Result<(), EngineError> {
    let move_list: Vec<Move> = gen.all_moves(pos)?;
    let mv: Option<&Move> = move_list
        .iter()
        .find(|x: &&Move| x.to_notation() == notation);
    pos.make_move(mv.unwrap());

    Ok(())
}

///////////////////////////////

// pub fn execute(&mut self, command: &str) {
//     let mut iter: std::str::Split<'_, &str> = command.split(" ");
//     let cmd: &str = iter.next().unwrap_or("");
//     let args: Vec<&str> = iter.collect::<Vec<&str>>();

//     match cmd {
//         "eval" => self.command_eval(args),
//         "search" => self.command_search(args),

//         "magictest" => magics::test(),

//         "moves" => self.command_moves(args),
//         "perft" => self.command_perft(args),

//         "clear" => print!("{esc}c", esc = 27 as char),
//         "pos" => self.command_pos(args),
//         "fen" => self.command_fen(args),
//         "d" => self.command_draw(args),

//         // "debug" => movegen::test::en_passant_discovered_check(),
//         x => println!("Unexpected command: {}", x),
//     }
// }

// fn command_pos(&mut self, args: Vec<&str>) {
//     self.board = match args[0] {
//         "start" => fen::parse(fen::START),
//         "empty" => fen::parse(fen::EMPTY),
//         _ => fen::parse(args.join(" ").as_str()),
//     };
// }

// fn command_moves(&mut self, _args: Vec<&str>) {
//     self.gen.all_moves(&self.board);
// }

// fn command_perft(&mut self, args: Vec<&str>) {
//
// }
