use crate::{
    moves::{
        move_data::MoveItem,
        precalculate::{cache::PrecalculatedCache, magic_bitboards::hash_with_magic},
    },
    state::{boards::BitBoard, game::GameState, movelist::MoveList, pieces::Piece, player::Player, square::Square},
};

// #[inline(always)]
pub fn generate_rook_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
    let mut rooks = game
        .bitboards
        .get_board_by_piece(player, Piece::Rook)
        .clone();
    let occupied = game.bitboards.occupied.clone();
    let opponent_occupied = game.bitboards.pos_to_player[player.opponent() as usize];

    while rooks != 0 {
        let pos = rooks.pop_mut();

        let from = Square::from(pos);

        let magic_index = hash_with_magic(
            cache.rook_magics[pos as usize],
            occupied,
        );
        let moves_mask = cache.rook_magic_attack_tables[magic_index];

        let mut valid_silents = moves_mask & !occupied;
        let mut valid_captures = moves_mask & opponent_occupied;

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Rook,
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
                piece: Piece::Rook,
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
