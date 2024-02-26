use crate::{
    moves::{
        move_data::MoveItem,
        precalculate::{cache::PrecalculatedCache, magic_bitboards::hash_with_magic},
    },
    state::{boards::BitBoard, game::GameState, pieces::Piece, player::Player, square::Square},
};

// #[inline(always)]
pub fn generate_queen_moves(
    movelist: &mut Vec<MoveItem>,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
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

        let moves_mask = rook_moves_mask | bishop_moves_mask;

        let mut valid_silents = moves_mask & !game.bitboards.get_occupied();
        let mut valid_captures = moves_mask & opponent_occupied;

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Queen(player),
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

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Queen(player),
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
