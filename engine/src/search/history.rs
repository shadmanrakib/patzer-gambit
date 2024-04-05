pub type HistoryMoves = [[[i16; 64]; 7]; 2]; // history[player][piece][to pos]

pub fn init_history_moves() -> HistoryMoves {
    [[[0; 64]; 7]; 2]
}