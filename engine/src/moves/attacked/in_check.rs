use crate::{
    moves::precalculate::cache::PrecalculatedCache,
    state::{bitboards::BitBoard, game::GameState, pieces::Piece, player::Player},
};

use super::square_attacked::is_square_attacked;

#[inline]
pub fn is_in_check(player: Player, game: &GameState, cache: &PrecalculatedCache) -> bool {
    return is_square_attacked(
        game.bitboards
            .get_board_by_piece(Piece::King(player))
            .clone()
            .pop_mut(),
        player.opponent(),
        game,
        cache,
    );
}
