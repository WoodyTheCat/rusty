use crate::{fen, types::position::Position};

#[derive(Default)]
pub struct Bot {
    position: Position,
}

impl Bot {
    pub fn execute(&mut self, command: &str) {
        let mut iter: std::str::Split<'_, &str> = command.split(" ");
        let cmd: &str = iter.next().unwrap_or("");
        let args: Vec<&str> = iter.collect::<Vec<&str>>();

        match cmd {
            "eval" => self.command_eval(args),
            "search" => self.command_search(args),

            "moves" => self.command_moves(args),
            "perft" => self.command_perft(args),

            "pos" => self.command_pos(args),
            "fen" => self.command_fen(args),
            "d" => self.command_draw(args),
            x => println!("Unexpected command: {}", x),
        }
    }

    fn command_pos(&mut self, args: Vec<&str>) {
        self.position = match args[0] {
            "start" => fen::parse(fen::START),
            "empty" => fen::parse(fen::EMPTY),
            _ => fen::parse(args.join(" ").as_str()),
        };
    }

    fn command_eval(&mut self, _args: Vec<&str>) {}
    fn command_moves(&mut self, args: Vec<&str>) {}
    fn command_perft(&mut self, args: Vec<&str>) {}
    fn command_search(&mut self, args: Vec<&str>) {}

    fn command_fen(&mut self, _args: Vec<&str>) {
        println!("{}", fen::position_to_fen(&self.position));
    }
    fn command_draw(&mut self, _args: Vec<&str>) {
        println!("\n{}", self.position)
    }
}
