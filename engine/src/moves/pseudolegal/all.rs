use crate::{
    moves::{
        self, move_data::MoveItem, precalculate::cache::PrecalculatedCache
    },
    state::{game::GameState, player::Player},
};

#[inline(always)]
pub fn generate_pseudolegal_moves(game: &GameState, player: Player, cache: &PrecalculatedCache) -> Vec<MoveItem> {
    let mut moves = Vec::<MoveItem>::new();
    moves.append(&mut moves::pseudolegal::pawn::generate_pawn_moves(game, player));
    moves.append(&mut moves::pseudolegal::knight::generate_knight_moves(game, player, cache));
    moves.append(&mut moves::pseudolegal::bishop::generate_bishop_moves(game, player, cache));
    moves.append(&mut moves::pseudolegal::rook::generate_rook_moves(game, player, cache));
    moves.append(&mut moves::pseudolegal::queen::generate_queen_moves(game, player, cache));
    moves.append(&mut moves::pseudolegal::king::generate_king_moves(game, player, cache));
    moves.append(&mut moves::pseudolegal::castling::generate_castling_moves(&game, player, cache));
    return moves;
}
