use crate::{
    moves::{
        move_data::MoveItem,
        precalculate::{cache::PrecalculatedCache, magic_bitboards::hash_with_magic},
    },
    state::{boards::BitBoard, game::GameState, movelist::MoveList, pieces::Piece, player::Player, square::Square},
};

// #[inline(always)]
pub fn generate_bishop_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
    let mut bishops = game
        .bitboards
        .get_board_by_piece(player, Piece::Bishop)
        .clone();
    let occupied = game.bitboards.occupied.clone();
    let opponent_occupied = game.bitboards.pos_to_player[player.opponent() as usize];

    while bishops != 0 {
        let pos = bishops.pop_mut();

        let from = Square::from(pos);

        let magic_index = hash_with_magic(
            cache.bishop_magics[pos as usize],
            occupied,
        );
        let moves_mask = cache.bishop_magic_attack_tables[magic_index];

        let mut valid_silents = moves_mask & !game.bitboards.occupied;
        let mut valid_captures = moves_mask & opponent_occupied;

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Bishop,
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

        while valid_silents != 0 {
            let silent_pos = valid_silents.pop_mut();

            let to = Square::from(silent_pos);

            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Bishop,
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
