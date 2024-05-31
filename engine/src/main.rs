mod boards;
mod evaluation;
mod fen;
mod incremental;
mod lookups;
mod magics;
mod masks;
mod moves;
mod movescoring;
mod mv;
mod perft;
mod pieces;
mod player;
mod position;
mod searcher;
mod searchinfo;
mod settings;
mod square;
mod time;
mod transposition;
mod uci;
mod zobrist;

#[cfg(test)]
mod tests;

use uci::uci_loop;

fn main() {
    uci_loop();
}
