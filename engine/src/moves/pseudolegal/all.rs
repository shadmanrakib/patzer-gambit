use crate::{
    moves::{self, precalculate::cache::PrecalculatedCache},
    state::{game::GameState, movelist::MoveList, player::Player},
};

// #[inline(always)]
pub fn generate_pseudolegal_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
    moves::pseudolegal::pawn::generate_pawn_moves(movelist, game, player, cache);
    moves::pseudolegal::knight::generate_knight_moves(movelist, game, player, cache);
    moves::pseudolegal::bishop::generate_bishop_moves(movelist, game, player, cache);
    moves::pseudolegal::rook::generate_rook_moves(movelist, game, player, cache);
    moves::pseudolegal::queen::generate_queen_moves(movelist, game, player, cache);
    moves::pseudolegal::king::generate_king_moves(movelist, game, player, cache);
    moves::pseudolegal::castling::generate_castling_moves(movelist, game, player, cache);
}
