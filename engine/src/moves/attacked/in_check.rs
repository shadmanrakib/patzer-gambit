use crate::{
    moves::precalculate::cache::PrecalculatedCache,
    state::{bitboards::BitBoard, game::GameState, pieces::Piece, player::Player},
};

use super::square_attacked::times_square_attacked;

#[inline]
pub fn is_in_check(pos: i8, player: Player, game: &GameState, cache: &PrecalculatedCache) -> bool {
    return times_square_attacked(
        game.bitboards
            .get_board_by_piece(Piece::King(player))
            .clone()
            .pop_mut(),
        player.opponent(),
        &game,
        &cache,
    ) == 0;
}
