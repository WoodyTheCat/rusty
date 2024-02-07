#![warn(clippy::pedantic)]

mod fen;
mod magics;
mod movegen;
mod search;
mod types;
mod uci;

fn main() {
    let _ = uci::uci_loop();
}
