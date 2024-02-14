use crate::{
    moves::{move_data::Move, precalculate::bishop::create_bishop_potential_moves_mask_on_the_fly},
    state::{bitboards::BitBoard, game::GameState, pieces::Piece, player::Player, square::Square},
};

pub fn generate_bishops_moves_on_the_fly(
    game: &GameState,
    player: Player,
) -> (Vec<Move>, Vec<Move>) {
    let mut silents: Vec<Move> = vec![];
    let mut captures: Vec<Move> = vec![];

    let mut bishops = game
        .bitboards
        .get_board_by_piece(Piece::Bishop(player))
        .clone();
    let occupied = game.bitboards.get_occupied().clone();
    let opponent_occupied = game.bitboards.get_occupied_by_player(player.opponent());

    while bishops != 0 {
        let pos = bishops.pop_mut();

        let from = Square::from(pos);

        let moves_mask = create_bishop_potential_moves_mask_on_the_fly(pos, occupied);
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
                promotion: Piece::Bishop(player),
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
                promotion: Piece::Bishop(player),
            })
        }
    }

    return (silents, captures);
}
