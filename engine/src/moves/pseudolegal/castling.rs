use crate::{
    moves::{
        attacked::{in_check::is_in_check, square_attacked::is_square_attacked},
        move_item::MoveItem,
        precalculate::cache::PrecalculatedCache,
    },
    state::{bitboards::BitBoard, game::GameState, pieces::Piece, player::Player},
};

pub fn generate_castling_moves(
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) -> Vec<MoveItem> {
    let mut moves: Vec<MoveItem> = vec![];

    // can't castle if in check
    if is_in_check(player, game, cache) {
        return moves;
    }

    let occupied = game.bitboards.get_occupied();

    match player {
        Player::White => {
            // check if 5 is attacked, king side between transition
            // make sure in between is also empty
            if game.castle_permissions.white_king_side
                && !occupied.get(5)
                && !occupied.get(6)
                && !is_square_attacked(5, player.opponent(), game, cache)
            {
                // we will mark the king movement
                moves.push(MoveItem {
                    from_pos: 4,
                    to_pos: 6,
                    piece: Piece::King(player),
                    promotion_piece: Piece::Empty,
                    captured_piece: Piece::Empty,
                    promoting: false,
                    capturing: false,
                    double: false,
                    enpassant: false,
                    castling: true,
                    score: 0.,
                });
            }
            // check if 2 is attacked, queen side between transition
            // make sure in between is also empty
            if game.castle_permissions.white_queen_side
                && !occupied.get(1)
                && !occupied.get(2)
                && !occupied.get(3)
                && !is_square_attacked(3, player.opponent(), game, cache)
            {
                moves.push(MoveItem {
                    from_pos: 4,
                    to_pos: 2,
                    piece: Piece::King(player),
                    promotion_piece: Piece::Empty,
                    captured_piece: Piece::Empty,
                    promoting: false,
                    capturing: false,
                    double: false,
                    enpassant: false,
                    castling: true,
                    score: 0.,
                });
            }
        }
        Player::Black => {
            if game.castle_permissions.black_king_side
                && !occupied.get(61)
                && !occupied.get(62)
                && !is_square_attacked(61, player.opponent(), game, cache)
            {
                // check if 61 is attacked, king side between transition
                // make sure in between is also empty

                moves.push(MoveItem {
                    from_pos: 60,
                    to_pos: 62,
                    piece: Piece::King(player),
                    promotion_piece: Piece::Empty,
                    captured_piece: Piece::Empty,
                    promoting: false,
                    capturing: false,
                    double: false,
                    enpassant: false,
                    castling: true,
                    score: 0.,
                });
            }
            if game.castle_permissions.black_queen_side
                && !occupied.get(57)
                && !occupied.get(58)
                && !occupied.get(59)
                && !is_square_attacked(59, player.opponent(), game, cache)
            {
                // check if 58 is attacked, queen side between transition
                // make sure in between is also empty

                moves.push(MoveItem {
                    from_pos: 60,
                    to_pos: 58,
                    piece: Piece::King(player),
                    promotion_piece: Piece::Empty,
                    captured_piece: Piece::Empty,
                    promoting: false,
                    capturing: false,
                    double: false,
                    enpassant: false,
                    castling: true,
                    score: 0.,
                });
            }
        }
    };

    return moves;
}
