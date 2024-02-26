use crate::{
    moves::precalculate::cache::PrecalculatedCache,
    state::{boards::BitBoard, game::GameState, pieces::Piece, player::Player},
};

use super::square_attacked::is_square_attacked;

// #[inline(always)]
pub fn is_in_check(player: Player, game: &GameState, cache: &PrecalculatedCache) -> bool {
    let mut king_bitboard = game
        .bitboards
        .get_board_by_piece(Piece::King(player))
        .clone();

    if king_bitboard == 0 {
        game.print_state();
        println!("Checkmate")
    }

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
