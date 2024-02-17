use crate::moves::move_data::Move;
use crate::moves::precalculate::cache::PrecalculatedCache;
use crate::state::bitboards::BitBoard;
use crate::state::game::GameState;
use crate::state::pieces::Piece;
use crate::state::player::Player;
use crate::state::square::Square;

// single forward non promotion, double, promotion, capture
pub fn generate_knight_moves(
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) -> (Vec<Move>, Vec<Move>) {
    let mut silents: Vec<Move> = vec![];
    let mut captures: Vec<Move> = vec![];

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

            captures.push(Move {
                from_rank: from.rank,
                from_file: from.file,
                to_rank: to.rank,
                to_file: to.file,
                promotion: Piece::Knight(player),
            })
        }


        while valid_silents != 0 {
            let silent_pos = valid_silents.pop_mut();

            let to = Square::from(silent_pos);

            silents.push(Move {
                from_rank: from.rank,
                from_file: from.file,
                to_rank: to.rank,
                to_file: to.file,
                promotion: Piece::Knight(player),
            })
        }
    }

    return (silents, captures);
}
