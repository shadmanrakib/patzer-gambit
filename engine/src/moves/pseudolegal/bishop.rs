use crate::{
    moves::{
        move_data::MoveItem,
        precalculate::{cache::PrecalculatedCache, magic_bitboards::hash_with_magic},
    },
    state::{bitboards::BitBoard, game::GameState, pieces::Piece, player::Player, square::Square},
};

#[inline(always)]
pub fn generate_bishop_moves(
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) -> Vec<MoveItem> {
    let mut silents: Vec<MoveItem> = vec![];
    let mut captures: Vec<MoveItem> = vec![];

    let mut bishops = game
        .bitboards
        .get_board_by_piece(Piece::Bishop(player))
        .clone();
    let occupied = game.bitboards.get_occupied().clone();
    let opponent_occupied = game.bitboards.get_occupied_by_player(player.opponent());

    while bishops != 0 {
        let pos = bishops.pop_mut();

        let from = Square::from(pos);

        let magic_index = hash_with_magic(
            cache.bishop_potential_blockers_masks[pos as usize],
            occupied,
            cache.bishop_magics[pos as usize],
            cache.bishop_bit_counts[pos as usize],
        );
        let moves_mask = cache.bishop_magic_attack_tables[pos as usize][magic_index];

        let mut valid_silents = moves_mask & !game.bitboards.get_occupied();
        let mut valid_captures = moves_mask & opponent_occupied;

        while valid_captures != 0 {
            let capture_pos = valid_captures.pop_mut();

            let to = Square::from(capture_pos);

            captures.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Bishop(player),
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
                piece: Piece::Bishop(player),
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
