use crate::{
    moves::{
        move_data::Move,
        precalculate::{
            cache::PrecalculatedCache, magic_bitboards::hash_with_magic,
        },
    },
    state::{bitboards::BitBoard, game::GameState, pieces::Piece, player::Player, square::Square},
};

pub fn generate_rooks_moves(
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) -> (Vec<Move>, Vec<Move>) {
    let mut silents: Vec<Move> = vec![];
    let mut captures: Vec<Move> = vec![];

    let mut rooks = game
        .bitboards
        .get_board_by_piece(Piece::Bishop(player))
        .clone();
    let occupied = game.bitboards.get_occupied().clone();
    let opponent_occupied = game.bitboards.get_occupied_by_player(player.opponent());

    while rooks != 0 {
        let pos = rooks.pop_mut();

        let from = Square::from(pos);

        let magic_index = hash_with_magic(
            cache.rook_potential_blockers_masks[pos as usize],
            occupied,
            cache.rook_magics[pos as usize],
            cache.rook_bit_counts[pos as usize],
        );
        let moves_mask = cache.rook_magic_attack_tables[pos as usize][magic_index];

        let mut valid_silents = moves_mask & !occupied;
        let mut valid_captures = moves_mask & opponent_occupied;

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            captures.push(Move {
                from_rank: from.rank,
                from_file: from.file,
                to_rank: to.rank,
                to_file: to.file,
                promotion: Piece::Rook(player),
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
                promotion: Piece::Rook(player),
            })
        }
    }

    return (silents, captures);
}
