use crate::{
    moves::generator::{pieces, precalculated_lookups::cache::PrecalculatedCache},
    mv::MoveList,
    player::Player,
    position::GameState,
};

pub fn generate_pseudolegal_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
    only_captures: bool,
) {
    pieces::queen::generate_queen_moves(movelist, game, player, cache, only_captures);
    pieces::rook::generate_rook_moves(movelist, game, player, cache, only_captures);
    pieces::bishop::generate_bishop_moves(movelist, game, player, cache, only_captures);
    pieces::knight::generate_knight_moves(movelist, game, player, cache, only_captures);
    pieces::pawn::generate_pawn_moves(movelist, game, player, cache, only_captures);
    pieces::king::generate_king_moves(movelist, game, player, cache, only_captures);
    if !only_captures {
        pieces::castling::generate_castling_moves(movelist, game, player, cache);
    }
}
