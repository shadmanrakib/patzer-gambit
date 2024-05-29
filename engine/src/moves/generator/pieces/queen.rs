use crate::{
    moves::{
        data::MoveItem,
        generator::precalculated_lookups::{cache::PrecalculatedCache, magic_bitboards::hash_with_magic},
    },
    state::{boards::BitBoard, game::GameState, moves::MoveList, pieces::Piece, player::Player, square::Square},
};

// #[inline(always)]
pub fn generate_queen_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
    only_captures: bool,
) {
    let mut queens = game
        .bitboards
        .get_board_by_piece(player, Piece::Queen)
        .clone();
    let occupied = game.bitboards.occupied.clone();
    let opponent_occupied = game.bitboards.pos_to_player[player.opponent() as usize];

    while queens != 0 {
        let pos = queens.pop_mut();

        let from = Square::from(pos);

        let rook_magic_index = hash_with_magic(
            cache.rook_magics[pos as usize],
            occupied,
        );
        let rook_moves_mask = cache.rook_magic_attack_tables[rook_magic_index];

        let bishop_magic_index = hash_with_magic(
            cache.bishop_magics[pos as usize],
            occupied,
        );
        let bishop_moves_mask = cache.bishop_magic_attack_tables[bishop_magic_index];

        let moves_mask = rook_moves_mask | bishop_moves_mask;

        let mut valid_silents = moves_mask & !game.bitboards.occupied;
        let mut valid_captures = moves_mask & opponent_occupied;

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Queen,
                promotion_piece: Piece::Empty,
                captured_piece: game.bitboards.pos_to_piece[capture_pos as usize],
                promoting: false,
                capturing: true,
                double: false,
                enpassant: false,
                castling: false,
                score: 0,
            })
        }

        if only_captures {
            continue;
        }

        while valid_silents != 0 {
            let silent_pos = valid_silents.pop_mut();

            let to = Square::from(silent_pos);

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Queen,
                promotion_piece: Piece::Empty,
                captured_piece: Piece::Empty,
                promoting: false,
                capturing: false,
                double: false,
                enpassant: false,
                castling: false,
                score: 0,
            })
        }
    }
}
