mod constants;
mod evaluation;
mod fen;
mod moves;
mod search;
mod state;

mod incremental;
mod perft;
mod searcher;
mod searchinfo;
mod time;
mod transposition;
mod uci;
mod zobrist;

#[cfg(test)]
mod tests;

use incremental::inc_test;
use searcher::Searcher;
use uci::uci_loop;

fn main() {
    uci_loop();
}
