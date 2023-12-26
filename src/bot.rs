use crate::{fen, magics, movegen::MoveGen, types::board_state::BoardState};

#[derive(Default)]
pub struct Bot {
    board: BoardState,
}

impl Bot {
    pub fn execute(&mut self, command: &str) {
        let mut iter: std::str::Split<'_, &str> = command.split(" ");
        let cmd: &str = iter.next().unwrap_or("");
        let args: Vec<&str> = iter.collect::<Vec<&str>>();

        match cmd {
            "eval" => self.command_eval(args),
            "search" => self.command_search(args),

            "magictest" => magics::test(),

            "moves" => self.command_moves(args),
            "perft" => self.command_perft(args),

            "clear" => print!("{esc}c", esc = 27 as char),
            "pos" => self.command_pos(args),
            "fen" => self.command_fen(args),
            "d" => self.command_draw(args),
            x => println!("Unexpected command: {}", x),
        }
    }

    fn command_pos(&mut self, args: Vec<&str>) {
        self.board = match args[0] {
            "start" => fen::parse(fen::START),
            "empty" => fen::parse(fen::EMPTY),
            _ => fen::parse(args.join(" ").as_str()),
        };
    }

    fn command_eval(&mut self, _args: Vec<&str>) {}

    fn command_moves(&mut self, _args: Vec<&str>) {
        let mut gen: MoveGen = MoveGen::default();

        gen.all_moves(&self.board);
    }

    fn command_perft(&mut self, _args: Vec<&str>) {}
    fn command_search(&mut self, _args: Vec<&str>) {}

    fn command_fen(&mut self, _args: Vec<&str>) {
        println!("{}", fen::board_to_fen(&self.board));
    }
    fn command_draw(&mut self, _args: Vec<&str>) {
        println!("\n{}", self.board)
    }
}
