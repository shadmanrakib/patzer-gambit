use crate::constants::masks::{RANK_1_MASK, RANK_2_MASK, RANK_7_MASK, RANK_8_MASK};
use crate::moves::move_data::MoveItem;
use crate::moves::precalculate::cache::PrecalculatedCache;
use crate::state::boards::BitBoard;
use crate::state::movelist::MoveList;
use crate::state::square::Square;
use crate::state::{game::GameState, pieces::Piece, player::Player};

// use crate::masks::RANK_1_MASK;

// single forward non promotion, double, promotion, capture
// #[inline(always)]
pub fn generate_pawn_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
    generate_pawn_single_forward_moves(movelist, game, player);
    generate_pawn_double_forward_moves(movelist, game, player);
    generate_pawn_attack_moves(movelist, game, player, cache);
}

#[inline(always)]
pub fn generate_pawn_single_forward_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
) {
    let pawnboard = game.bitboards.get_board_by_piece(player, Piece::Pawn);
    let can_move = pawnboard & !RANK_1_MASK & !RANK_8_MASK;
    let moved_forward = match player {
        Player::White => can_move << 8,
        Player::Black => can_move >> 8,
    };
    let mut valid = moved_forward & !game.bitboards.occupied;

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
            let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop];
            for promotion_piece in promotion_pieces {
                movelist.push(MoveItem {
                    from_pos: from.into(),
                    to_pos: to.into(),
                    piece: Piece::Pawn,
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
                piece: Piece::Pawn,
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

#[inline(always)]
pub fn generate_pawn_double_forward_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
) {
    let pawnboard = game.bitboards.get_board_by_piece(player, Piece::Pawn);
    let occupied = game.bitboards.occupied;

    let can_move = match player {
        Player::White => pawnboard & RANK_2_MASK,
        Player::Black => pawnboard & RANK_7_MASK,
    };
    let moved_forward = match player {
        Player::White => can_move << 16,
        Player::Black => can_move >> 16,
    };
    let mask = !(occupied
        | match player {
            Player::White => occupied << 8,
            Player::Black => occupied >> 8,
        });
    let mut valid = moved_forward & mask;

    while valid != 0 {
        let pos = valid.pop_mut();

        let from = match player {
            Player::White => pos - 16,
            Player::Black => pos + 16,
        };

        movelist.push(MoveItem {
            from_pos: from,
            to_pos: pos,
            piece: Piece::Pawn,
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

#[inline(always)]
pub fn generate_pawn_attack_moves(
    movelist: &mut MoveList,
    game: &GameState,
    player: Player,
    cache: &PrecalculatedCache,
) {
    let mut pawns = game
        .bitboards
        .get_board_by_piece(player, Piece::Pawn)
        .clone();
    let opposite_occupied = game.bitboards.pos_to_player[player.opponent() as usize];
    // pawns.print_board();

    while pawns != 0 {
        let pos = pawns.pop_mut();
        let attacking_mask = cache.pawn_attack_moves_mask[player as usize][pos as usize];
        let mut attacking = opposite_occupied & attacking_mask;
        let mut attacking_enpassant = game.enpassant_square & attacking_mask;

        while attacking != 0 {
            let to = attacking.pop_mut();
            generate_pawn_attack_moves_helper(movelist, game, pos, to, false);
        }
        while attacking_enpassant != 0 {
            let to = attacking_enpassant.pop_mut();
            generate_pawn_attack_moves_helper(movelist, game, pos, to, true);
        }
    }
}

#[inline(always)]
pub fn generate_pawn_attack_moves_helper(
    movelist: &mut MoveList,
    game: &GameState,
    from_pos: i8,
    to: i8,
    enpassant: bool,
) {
    let captured_piece = game.bitboards.pos_to_piece[to as usize];

    println!("to: {}", to);

    // promotion on capture
    if to <= 7 || to >= 56 {
        // we can prob only do queen and knight, should help reduce tree without hurting performance
        // let promotion_pieces = vec![Piece::Queen(player), Piece::Knight(player)];
        let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop];
        for promotion_piece in promotion_pieces {
            movelist.push(MoveItem {
                from_pos,
                to_pos: to,
                piece: Piece::Pawn,
                promotion_piece,
                captured_piece,
                promoting: true,
                capturing: true,
                double: false,
                enpassant,
                castling: false,
                score: 0.,
            })
        }
    } else {
        movelist.push(MoveItem {
            from_pos,
            to_pos: to,
            piece: Piece::Pawn,
            promotion_piece: Piece::Empty,
            captured_piece,
            promoting: false,
            capturing: true,
            double: false,
            enpassant,
            castling: false,
            score: 0.,
        })
    }
}
