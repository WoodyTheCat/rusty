mod bot;
mod fen;
mod magics;
mod movegen;
mod types;

use colored::{Colorize, CustomColor};
use text_io::read;

use bot::*;

fn main() {
    // movegen::magic_generate::main(); // Generate precomputed data

    let mut bot: Bot = Bot::default();

    loop {
        print!(
            "{}",
            Colorize::custom_color(
                "> ",
                CustomColor {
                    r: 128,
                    g: 128,
                    b: 128
                }
            )
        );
        let command: String = read!("{}\r\n");
        if command == "quit" {
            break;
        }
        bot.execute(command.as_str())
    }

    println!("\n-- Finished Executing --");
}
