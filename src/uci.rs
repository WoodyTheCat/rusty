use std::io::{stdin, BufRead};

use itertools::Itertools;

use crate::{
    fen,
    movegen::MoveGen,
    search::{perft::Perft, NegaMax},
    types::{board_state::BoardState, r#move::Move},
};

pub fn uci_loop() {
    let mut board: BoardState = fen::parse(fen::START);
    let mut searcher: NegaMax = NegaMax::default();
    loop {
        let mut buffer: String = String::new();
        stdin().lock().read_line(&mut buffer).unwrap();

        let key: Vec<&str> = buffer.split_ascii_whitespace().collect_vec();

        match &(*key.first().unwrap()).to_string()[..] {
            "quit" => break,
            "uci" => init_uci(),
            "pos" => board = update_board(&key[1..].join(" ")),
            "go" => go(&mut board, &mut searcher, &key),
            "isready" => println!("readyok"),
            "ucinewgame" => {}
            "d" => println!("\n{}", board),
            "perft" => {
                let mut perft: Perft = Perft::default();
                perft.verbose(&board, key[1].parse::<i32>().ok().unwrap());
            }
            _ => println!("Command not understood"),
        }
    }
}

fn init_uci() {
    println!("id name Rusty");
    println!("id author Fergus Rorke");
    println!("uciok");
}

fn update_board(args: &String) -> BoardState {
    let tokens: Vec<&str> = args.split_ascii_whitespace().collect_vec();
    let keyword: &&str = tokens.first().unwrap();
    let mut pos: BoardState = match &keyword[..] {
        "start" => fen::parse(fen::START),
        "fen" => return fen::parse(&args[4..]),
        _ => panic!("Unknown parameter to position!"),
    };

    let move_keyword: Option<&&str> = tokens.get(1);

    if move_keyword.is_some() {
        apply_moves(&mut pos, &tokens[2..]);
    }

    pos
}

fn go(pos: &mut BoardState, searcher: &mut NegaMax, data: &[&str]) {
    // let movetime = data[2].parse::<u128>().unwrap();
    // searcher.move_time((movetime / 1000) - 1);
    // searcher.move_time(movetime);
    // let mv = searcher.best_move_depth(pos, 15);
    // println!("eval: {}", mv.eval);
    // println!("static eval: {}", eval(pos));
    // println!("bestmove {}", mv.mv.to_algebraic());
}

fn apply_moves(pos: &mut BoardState, moves: &[&str]) {
    let mut gen: MoveGen = MoveGen::default();

    for mv_str in moves.iter() {
        let move_list: Vec<Move> = gen.all_moves(pos);
        let mv: Option<&Move> = move_list
            .iter()
            .find(|x: &&Move| x.to_notation() == *mv_str);
        pos.make_move(mv.unwrap());
    }
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
