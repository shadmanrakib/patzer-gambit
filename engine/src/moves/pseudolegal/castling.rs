use crate::{
    moves::{
        attacked::{in_check::is_in_check, square_attacked::is_square_attacked},
        move_data::MoveItem,
        precalculate::cache::PrecalculatedCache,
    },
    state::{boards::BitBoard, game::GameState, movelist::MoveList, pieces::Piece, player::Player},
};
// #[inline(always)]

const WHITE_KING_SIDE_CASTLE_PATH: u64 = (1 << 5) | (1 << 6);
const WHITE_QUEEN_SIDE_CASTLE_PATH: u64 = (1 << 1) | (1 << 2) | (1 << 3);
const BLACK_KING_SIDE_CASTLE_PATH: u64 = (1 << 61) | (1 << 62);
const BLACK_QUEEN_SIDE_CASTLE_PATH: u64 = (1 << 57) | (1 << 58) | (1 << 59);

pub fn generate_castling_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
    // can't castle if in check
    if is_in_check(player, game, cache) {
        return;
    }

    let occupied = game.bitboards.occupied;
    let opponent: Player = player.opponent();

    match player {
        Player::White => {
            if game.bitboards.pos_to_piece[4] != Piece::King {
                return;
            }

            // check if 5 is attacked, king side between transition
            // make sure in between is also empty
            if game.castle_permissions.white_king_side
                && !occupied.get(5)
                && !occupied.get(6)
                && !is_square_attacked(5, opponent, game, cache)
            {
                // we will mark the king movement
                if game.bitboards.pos_to_piece[7] == Piece::Rook {
                    movelist.push(MoveItem {
                        from_pos: 4,
                        to_pos: 6,
                        piece: Piece::King,
                        promotion_piece: Piece::Empty,
                        captured_piece: Piece::Empty,
                        promoting: false,
                        capturing: false,
                        double: false,
                        enpassant: false,
                        castling: true,
                        score: 0,
                    });
                }
            }
            // check if 2 is attacked, queen side between transition
            // make sure in between is also empty
            if game.castle_permissions.white_queen_side
                && !occupied.get(1)
                && !occupied.get(2)
                && !occupied.get(3)
                && !is_square_attacked(3, opponent, game, cache)
            {
                if game.bitboards.pos_to_piece[0] == Piece::Rook {
                    // TODO: need to change this to use king
                    movelist.push(MoveItem {
                        from_pos: 4,
                        to_pos: 2,
                        piece: Piece::King,
                        promotion_piece: Piece::Empty,
                        captured_piece: Piece::Empty,
                        promoting: false,
                        capturing: false,
                        double: false,
                        enpassant: false,
                        castling: true,
                        score: 0,
                    });
                }
            }
        }
        Player::Black => {
            if game.bitboards.pos_to_piece[60] != Piece::King {
                return;
            }

            if game.castle_permissions.black_king_side
                && !occupied.get(61)
                && !occupied.get(62)
                && !is_square_attacked(61, opponent, game, cache)
            {
                // check if 61 is attacked, king side between transition
                // make sure in between is also empty

                if game.bitboards.pos_to_piece[63] == Piece::Rook {
                    movelist.push(MoveItem {
                        from_pos: 60,
                        to_pos: 62,
                        piece: Piece::King,
                        promotion_piece: Piece::Empty,
                        captured_piece: Piece::Empty,
                        promoting: false,
                        capturing: false,
                        double: false,
                        enpassant: false,
                        castling: true,
                        score: 0,
                    });
                }
            }
            if game.castle_permissions.black_queen_side
                && !occupied.get(57)
                && !occupied.get(58)
                && !occupied.get(59)
                && !is_square_attacked(59, opponent, game, cache)
            {
                // check if 58 is attacked, queen side between transition
                // make sure in between is also empty

                if game.bitboards.pos_to_piece[56] == Piece::Rook {
                    movelist.push(MoveItem {
                        from_pos: 60,
                        to_pos: 58,
                        piece: Piece::King,
                        promotion_piece: Piece::Empty,
                        captured_piece: Piece::Empty,
                        promoting: false,
                        capturing: false,
                        double: false,
                        enpassant: false,
                        castling: true,
                        score: 0,
                    });
                }
            }
        }
    };
}
