use crate::moves::move_data::MoveItem;
use crate::moves::precalculate::cache::PrecalculatedCache;
use crate::state::boards::BitBoard;
use crate::state::game::GameState;
use crate::state::movelist::MoveList;
use crate::state::pieces::Piece;
use crate::state::player::Player;
use crate::state::square::Square;

// single forward non promotion, double, promotion, capture
// #[inline(always)]
pub fn generate_knight_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
    let mut knights = game
        .bitboards
        .get_board_by_piece(player, Piece::Knight)
        .clone();
    let opponent_occupied = game.bitboards.pos_to_player[player.opponent() as usize];

    while knights != 0 {
        let pos = knights.pop_mut();

        let from = Square::from(pos);

        let moves_mask = cache.knight_moves_masks[pos as usize];
        let mut valid_silents = moves_mask & !game.bitboards.occupied;
        let mut valid_captures = moves_mask & opponent_occupied;

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Knight,
                promotion_piece: Piece::Empty,
                captured_piece: game.bitboards.pos_to_piece[capture_pos as usize],
                promoting: false,
                capturing: true,
                double: false,
                enpassant: false,
                castling: false,
                score: 0.,
            })
        }

        while valid_silents != 0 {
            let silent_pos = valid_silents.pop_mut();

            let to = Square::from(silent_pos);

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Knight,
                promotion_piece: Piece::Empty,
                captured_piece: Piece::Empty,
                promoting: false,
                capturing: false,
                double: false,
                enpassant: false,
                castling: false,
                score: 0.,
            })
        }
    }
}
