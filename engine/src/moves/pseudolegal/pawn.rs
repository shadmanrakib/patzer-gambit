use crate::constants::masks::{RANK_1_MASK, RANK_2_MASK, RANK_7_MASK, RANK_8_MASK};
use crate::moves::move_data::MoveItem;
use crate::state::boards::BitBoard;
use crate::state::square::Square;
use crate::state::{game::GameState, pieces::Piece, player::Player};

// use crate::masks::RANK_1_MASK;

// single forward non promotion, double, promotion, capture
// #[inline(always)]
pub fn generate_pawn_moves(movelist: &mut Vec<MoveItem>, game: &GameState, player: Player) {
    generate_pawn_single_forward_moves(movelist, game, player);
    generate_pawn_double_forward_moves(movelist, game, player);
    generate_pawn_attack_moves(movelist, game, player);
}

#[inline(never)]
pub fn generate_pawn_single_forward_moves(
    movelist: &mut Vec<MoveItem>,
    game: &GameState,
    player: Player,
) {
    let pawnboard = game.bitboards.get_board_by_piece(Piece::Pawn(player));
    let can_move = pawnboard & !RANK_1_MASK & !RANK_8_MASK;
    let moved_forward = {
        match player {
            Player::White => can_move << 8,
            Player::Black => can_move >> 8,
        }
    };
    // let (player_occupied, opponent_occupied) = (game.bitboards.get_occupied_by_player(player).uint(), game.bitboards.get_occupied_by_player(player.opponent()).uint());
    let mut valid = moved_forward & !game.bitboards.get_occupied();

    while valid != 0 {
        let pos = valid.pop_mut();

        let to = Square::from(pos);

        let from = Square {
            file: pos % 8,
            rank: match player {
                Player::White => pos / 8 - 1,
                Player::Black => pos / 8 + 1,
            },
        };

        if to.rank == 0 || to.rank == 7 {
            // we can prob only do queen and knight, should help reduce tree without hurting performance
            // let promotion_pieces = vec![Piece::Queen(player), Piece::Knight(player)];
            let promotion_pieces = vec![
                Piece::Queen(player),
                Piece::Rook(player),
                Piece::Knight(player),
                Piece::Bishop(player),
            ];
            for promotion_piece in promotion_pieces {
                movelist.push(MoveItem {
                    from_pos: from.into(),
                    to_pos: to.into(),
                    piece: Piece::Pawn(player),
                    promotion_piece,
                    captured_piece: Piece::Empty,
                    promoting: true,
                    capturing: false,
                    double: false,
                    enpassant: false,
                    castling: false,
                    score: 0.,
                })
            }
        } else {
            movelist.push(MoveItem {
                from_pos: from.into(),
                to_pos: to.into(),
                piece: Piece::Pawn(player),
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

#[inline(never)]
pub fn generate_pawn_double_forward_moves(
    movelist: &mut Vec<MoveItem>,
    game: &GameState,
    player: Player,
) {
    let pawnboard = game.bitboards.get_board_by_piece(Piece::Pawn(player));
    let can_move = pawnboard & {
        match player {
            Player::White => RANK_2_MASK,
            Player::Black => RANK_7_MASK,
        }
    };
    let moved_forward = {
        match player {
            Player::White => can_move << 16,
            Player::Black => can_move >> 16,
        }
    };
    // let (player_occupied, opponent_occupied) = (game.bitboards.get_occupied_by_player(player).uint(), game.bitboards.get_occupied_by_player(player.opponent()).uint());
    let occupied = game.bitboards.get_occupied();
    let mask = !(occupied | {
        match player {
            Player::White => occupied << 8,
            Player::Black => occupied >> 8,
        }
    });
    let mut valid = moved_forward & mask;

    while valid != 0 {
        let pos = valid.pop_mut();

        let to = Square::from(pos);
        let from = Square::from({
            match player {
                Player::White => pos - 16,
                Player::Black => pos + 16,
            }
        });

        movelist.push(MoveItem {
            from_pos: from.into(),
            to_pos: to.into(),
            piece: Piece::Pawn(player),
            promotion_piece: Piece::Empty,
            captured_piece: Piece::Empty,
            promoting: false,
            capturing: false,
            double: true,
            enpassant: false,
            castling: false,
            score: 0.,
        })
    }
}

#[inline(never)]
pub fn generate_pawn_attack_moves(movelist: &mut Vec<MoveItem>, game: &GameState, player: Player) {
    let mut pawns = game
        .bitboards
        .get_board_by_piece(Piece::Pawn(player))
        .clone();
    let opposite_occupied = game.bitboards.get_occupied_by_player(player.opponent());

    // pawns.print_board();

    while pawns != 0 {
        let pos = pawns.pop_mut();

        let Square { rank, file } = Square::from(pos);
        // left file capture
        if file > 0 && rank != 7 && rank != 0 {
            let to = Square {
                rank: match player {
                    Player::White => rank + 1,
                    Player::Black => rank - 1,
                },
                file: file - 1,
            };
            generate_pawn_attack_moves_helper(movelist, game, player, pos, to, &opposite_occupied);
        }
        // right file capture
        if file < 7 && rank != 7 && rank != 0 {
            let to = Square {
                rank: match player {
                    Player::White => rank + 1,
                    Player::Black => rank - 1,
                },
                file: file + 1,
            };
            generate_pawn_attack_moves_helper(movelist, game, player, pos, to, &opposite_occupied);
        }
    }
}

#[inline(never)]
pub fn generate_pawn_attack_moves_helper(
    movelist: &mut Vec<MoveItem>,
    game: &GameState,
    player: Player,
    from_pos: i8,
    to: Square,
    opposite_occupied: &u64,
) {
    let can_enpassant = game.enpassant_square.exists
        && to.rank == game.enpassant_square.pos.rank
        && to.file == game.enpassant_square.pos.file;

    let captured_piece = if can_enpassant {
        let from = Square::from(from_pos);

        let rank = from.rank;
        let file = to.file;

        let captured_square = Square { rank, file };
        game.bitboards.get_piece_by_bit_pos(captured_square.into())
    } else {
        game.bitboards.get_piece_by_bit_pos(to.into())
    };

    if opposite_occupied.get(to.into()) || can_enpassant {
        // promotion on capture
        if to.rank == 0 || to.rank == 7 {
            // we can prob only do queen and knight, should help reduce tree without hurting performance
            // let promotion_pieces = vec![Piece::Queen(player), Piece::Knight(player)];
            let promotion_pieces = vec![
                Piece::Queen(player),
                Piece::Rook(player),
                Piece::Knight(player),
                Piece::Bishop(player),
            ];
            for promotion_piece in promotion_pieces {
                movelist.push(MoveItem {
                    from_pos,
                    to_pos: to.into(),
                    piece: Piece::Pawn(player),
                    promotion_piece,
                    captured_piece,
                    promoting: true,
                    capturing: true,
                    double: false,
                    enpassant: can_enpassant,
                    castling: false,
                    score: 0.,
                })
            }
        } else {
            movelist.push(MoveItem {
                from_pos,
                to_pos: to.into(),
                piece: Piece::Pawn(player),
                promotion_piece: Piece::Empty,
                captured_piece,
                promoting: false,
                capturing: true,
                double: false,
                enpassant: can_enpassant,
                castling: false,
                score: 0.,
            })
        }
    }
}
