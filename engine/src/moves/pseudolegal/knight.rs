use crate::moves::move_data::MoveItem;
use crate::moves::precalculate::cache::PrecalculatedCache;
use crate::state::bitboards::BitBoard;
use crate::state::game::GameState;
use crate::state::pieces::Piece;
use crate::state::player::Player;
use crate::state::square::Square;

// single forward non promotion, double, promotion, capture
#[inline(always)]
pub fn generate_knight_moves(
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) -> Vec<MoveItem> {
    let mut silents: Vec<MoveItem> = vec![];
    let mut captures: Vec<MoveItem> = vec![];

    let mut knights = game
        .bitboards
        .get_board_by_piece(Piece::Knight(player))
        .clone();
    let opponent_occupied =  game.bitboards.get_occupied_by_player(player.opponent());

    while knights != 0 {
        let pos = knights.pop_mut();

        let from = Square::from(pos);

        let moves_mask = cache.knight_moves_masks[pos as usize];
        let mut valid_silents = moves_mask & !game.bitboards.get_occupied();
        let mut valid_captures = moves_mask & opponent_occupied;

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            captures.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Knight(player),
                promotion_piece: Piece::Empty,
                captured_piece: game.bitboards.get_piece_by_bit_pos(capture_pos),
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

            silents.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Knight(player),
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

    // temporarily as i figure out how i want to structure things
    let mut moves = Vec::<MoveItem>::new();
    moves.append(&mut silents);
    moves.append(&mut captures);

    return moves;
}
