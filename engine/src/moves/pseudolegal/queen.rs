use crate::{
    moves::{
        move_data::Move,
        precalculate::{
            bishop::create_bishop_potential_moves_mask_on_the_fly, cache::PrecalculatedCache, magic_bitboards::hash_with_magic, rook::create_rook_potential_moves_mask_on_the_fly
        },
    },
    state::{bitboards::BitBoard, game::GameState, pieces::Piece, player::Player, square::Square},
};

pub fn generate_queens_moves_on_the_fly(
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) -> (Vec<Move>, Vec<Move>) {
    let mut silents: Vec<Move> = vec![];
    let mut captures: Vec<Move> = vec![];

    let mut queens = game
        .bitboards
        .get_board_by_piece(Piece::Queen(player))
        .clone();
    let occupied = game.bitboards.get_occupied().clone();
    let opponent_occupied = game.bitboards.get_occupied_by_player(player.opponent());

    while queens != 0 {
        let pos = queens.pop_mut();

        let from = Square::from(pos);

        let rook_magic_index = hash_with_magic(
            cache.rook_potential_blockers_masks[pos as usize],
            occupied,
            cache.rook_magics[pos as usize],
            cache.rook_bit_counts[pos as usize],
        );
        let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];

        let bishop_magic_index = hash_with_magic(
            cache.bishop_potential_blockers_masks[pos as usize],
            occupied,
            cache.bishop_magics[pos as usize],
            cache.bishop_bit_counts[pos as usize],
        );
        let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];

        let moves_mask = create_rook_potential_moves_mask_on_the_fly(pos, occupied)
            | create_bishop_potential_moves_mask_on_the_fly(pos, occupied);
        let mut valid_silents = moves_mask & !game.bitboards.get_occupied();
        let mut valid_captures = moves_mask & opponent_occupied;


        let a = rook_moves_mask | bishop_moves_mask;
        if a != moves_mask {
            a.print_board();
            moves_mask.print_board();
            println!("does not");
        }

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            captures.push(Move {
                from_rank: from.rank,
                from_file: from.file,
                to_rank: to.rank,
                to_file: to.file,
                promotion: Piece::Queen(player),
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
                promotion: Piece::Queen(player),
            })
        }
    }

    return (silents, captures);
}
