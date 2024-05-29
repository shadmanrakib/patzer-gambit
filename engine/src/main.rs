mod constants;
mod evaluation;
mod fen;
mod moves;
mod search;
mod state;
mod utils;

mod searcher;
mod uci;
mod controller;
mod time;
mod incremental;

#[cfg(test)]
mod tests;

use incremental::inc_test;
use uci::uci_loop;

use crate::{
    constants::search::{MAX_MAIN_SEARCH_DEPTH, TRANSITION_TABLE_SIZE, TRANSITION_TABLE_ADDRESSING_BITS},
    moves::data::{MoveItem, UnmakeMoveMetadata},
    search::{transposition::TTable, zobrist::ZobristHasher},
};

fn main() {

    uci_loop();
}
