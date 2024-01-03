#![warn(clippy::pedantic)]

mod fen;
mod magics;
mod movegen;
mod search;
mod types;
mod uci;

fn main() {
    uci::uci_loop();

    println!("\n-- Finished Executing --");
}
