use crate::{
    moves::{self, move_data::MoveItem, precalculate::cache::PrecalculatedCache},
    state::{game::GameState, player::Player},
};

// #[inline(always)]
pub fn generate_pseudolegal_moves(
    movelist: &mut Vec<MoveItem>,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
    moves::pseudolegal::pawn::generate_pawn_moves(movelist, game, player);
    moves::pseudolegal::knight::generate_knight_moves(movelist, game, player, cache);
    moves::pseudolegal::bishop::generate_bishop_moves(movelist, game, player, cache);
    moves::pseudolegal::rook::generate_rook_moves(movelist, game, player, cache);
    moves::pseudolegal::queen::generate_queen_moves(movelist, game, player, cache);
    moves::pseudolegal::king::generate_king_moves(movelist, game, player, cache);
    moves::pseudolegal::castling::generate_castling_moves(movelist, game, player, cache);
}
