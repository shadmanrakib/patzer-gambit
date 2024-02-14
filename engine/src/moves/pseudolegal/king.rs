use crate::moves::move_data::Move;
use crate::state::bitboards::BitBoard;
use crate::state::game::GameState;
use crate::state::pieces::Piece;
use crate::state::player::Player;
use crate::state::square::Square;

// single forward non promotion, double, promotion, capture
pub fn generate_king_moves(
    game: &GameState,
    king_moves_masks: &[u64; 64],
    player: Player,
) -> (Vec<Move>, Vec<Move>) {
    let mut silents: Vec<Move> = vec![];
    let mut captures: Vec<Move> = vec![];

    let mut kings = game
        .bitboards
        .get_board_by_piece(Piece::King(player))
        .clone();
    let opponent_occupied = game.bitboards.get_occupied_by_player(player.opponent());

    // should only really run once
    while kings != 0 {
        let pos = kings.pop_mut();

        let from = Square::from(pos);

        let moves_mask = king_moves_masks[pos as usize];
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
                promotion: Piece::King(player),
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
                promotion: Piece::King(player),
            })
        }
    }

    return (silents, captures);
}
