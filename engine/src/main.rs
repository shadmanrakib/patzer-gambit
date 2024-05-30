mod constants;
mod evaluation;
mod fen;
mod moves;
mod search;
mod state;

mod searcher;
mod uci;
mod time;
mod incremental;
mod zobrist;
mod transposition;
mod searchinfo;
mod perft;

#[cfg(test)]
mod tests;

use uci::uci_loop;

fn main() {
    // inc_test();
    uci_loop();
}
