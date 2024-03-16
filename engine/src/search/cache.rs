use super::killer::{init_killer_moves, KillerMoves};

pub struct SearchCache {
    pub killer_moves: KillerMoves,
}

impl SearchCache {
    pub fn init() -> SearchCache {
        SearchCache {
            killer_moves: init_killer_moves(),
        }
    }
}