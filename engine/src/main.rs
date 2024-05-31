mod boards;
mod evaluation;
mod fen;
mod incremental;
mod masks;
mod moves;
mod mv;
mod perft;
mod pieces;
mod player;
mod position;
mod scoring;
mod search;
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
