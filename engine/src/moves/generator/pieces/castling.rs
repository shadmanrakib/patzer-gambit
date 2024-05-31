use crate::{
    boards::BitBoard,
    lookups::Lookups,
    mv::{Move, MoveList},
    pieces::Piece,
    player::Player,
    position::{CastlePermissions, GameState},
};

pub fn generate_castling_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &Lookups,
) {
    // can't castle if in check
    if game.in_check(player, cache) {
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
            if game.castle_permissions.is_allowed(u8::WHITE_KING_SIDE)
                && !occupied.get(5)
                && !occupied.get(6)
                && !game.is_square_attacked(5, opponent, cache)
            {
                // we will mark the king movement
                if game.bitboards.pos_to_piece[7] == Piece::Rook {
                    movelist.push(Move {
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
            if game.castle_permissions.is_allowed(u8::WHITE_QUEEN_SIDE)
                && !occupied.get(1)
                && !occupied.get(2)
                && !occupied.get(3)
                && !game.is_square_attacked(3, opponent, cache)
            {
                if game.bitboards.pos_to_piece[0] == Piece::Rook {
                    // TODO: need to change this to use king
                    movelist.push(Move {
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

            if game.castle_permissions.is_allowed(u8::BLACK_KING_SIDE)
                && !occupied.get(61)
                && !occupied.get(62)
                && !game.is_square_attacked(61, opponent, cache)
            {
                // check if 61 is attacked, king side between transition
                // make sure in between is also empty

                if game.bitboards.pos_to_piece[63] == Piece::Rook {
                    movelist.push(Move {
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
            if game.castle_permissions.is_allowed(u8::BLACK_QUEEN_SIDE)
                && !occupied.get(57)
                && !occupied.get(58)
                && !occupied.get(59)
                && !game.is_square_attacked(59, opponent, cache)
            {
                // check if 58 is attacked, queen side between transition
                // make sure in between is also empty
                if game.bitboards.pos_to_piece[56] == Piece::Rook {
                    movelist.push(Move {
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
