mod constants;
mod evaluation;
mod fen;
mod moves;
mod search;
mod state;

mod searcher;
mod uci;
mod controller;
mod time;
mod incremental;
mod zobrist;
mod transposition;

#[cfg(test)]
mod tests;

use incremental::inc_test;
use uci::uci_loop;

fn main() {
    inc_test();
    uci_loop();
}
