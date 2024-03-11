use crate::{
    moves::precalculate::cache::PrecalculatedCache,
    state::{boards::BitBoard, game::GameState, pieces::Piece, player::Player},
};

use super::square_attacked::is_square_attacked;

// #[inline(always)]
pub fn is_in_check(player: Player, game: &GameState, cache: &PrecalculatedCache) -> bool {    
    let king = game
        .bitboards
        .get_board_by_piece(player, Piece::King)
        .trailing_zeros() as i8;

    // if king == 64 {
    //     return true;
    // }

    return is_square_attacked(
        king,
        player.opponent(),
        game,
        cache,
    );
}
