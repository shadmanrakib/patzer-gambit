use super::{history::{init_history_moves, HistoryMoves}, killer::{init_killer_moves, KillerMoves}};

pub struct SearchCache {
    pub killer_moves: KillerMoves,
    pub history_moves: HistoryMoves,
}

impl SearchCache {
    pub fn init() -> SearchCache {
        SearchCache {
            killer_moves: init_killer_moves(),
            history_moves: init_history_moves(),
        }
    }
}