use crate::state::game::GameState;

use super::piece::STATIC_PIECE_POINTS;

pub fn simple(game: &GameState) -> i32 {
    // if !has_moves || game.half_move_clock >= 50 {
    //     return 0;
    // }

    let mut score: i32 = 0;
    for piece in 1..7 {
        score += STATIC_PIECE_POINTS[piece]
            * (game.bitboards.boards[0][piece].count_ones()
                - game.bitboards.boards[1][piece].count_ones()) as i32;
    }

    return score;
}
