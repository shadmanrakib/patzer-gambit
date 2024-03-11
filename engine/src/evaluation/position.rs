use crate::state::game::GameState;

const PIECE_POINTS: [i32; 7] = [
    0,     // Empty
    100,   // Pawn
    300,   // Knight
    325,   // Bishop
    500,   // Rook
    925,   // Queen
    10000, // King
];

pub fn simple(game: &GameState, in_check: bool, has_moves: bool, color: i32) -> i32 {
    if !has_moves || game.half_move_clock >= 50 {
        return 0;
    }

    let mut score: i32 = 0;
    for piece in 1..7 {
        score += PIECE_POINTS[piece]
            * ((game.bitboards.boards[0][piece].count_ones()
                - game.bitboards.boards[1][piece].count_ones())) as i32;
    }

    return score;
}
