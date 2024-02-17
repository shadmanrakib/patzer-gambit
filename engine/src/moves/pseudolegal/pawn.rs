use crate::constants::masks::{RANK_1_MASK, RANK_2_MASK, RANK_7_MASK, RANK_8_MASK};
use crate::moves::move_data::Move;
use crate::state::bitboards::BitBoard;
use crate::state::square::Square;
use crate::state::{game::GameState, pieces::Piece, player::Player};

// use crate::masks::RANK_1_MASK;

// single forward non promotion, double, promotion, capture
pub fn generate_pawn_moves(
    game: &GameState,
    player: Player,
) -> (Vec<Move>, Vec<Move>, Vec<Move>, Vec<Move>, Vec<Move>) {
    let (single_forward_non_promotion, single_forward_promotion) =
        generate_pawn_single_forward_moves(game, player);
    let double_foreward = generate_pawn_double_forward_moves(game, player);
    let (capture_non_promotion, capture_promotion) = generate_pawn_attack_moves(game, player);

    return (
        single_forward_non_promotion,
        single_forward_promotion,
        double_foreward,
        capture_non_promotion,
        capture_promotion,
    );
}

pub fn generate_pawn_single_forward_moves(
    game: &GameState,
    player: Player,
) -> (Vec<Move>, Vec<Move>) {
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

    let mut non_promotion_moves = Vec::<Move>::new();
    let mut promotion_moves = Vec::<Move>::new();

    while valid != 0 {
        let pos = valid.pop_mut();

        let to_rank = pos / 8;
        let to_file = pos % 8;
        let from_rank = {
            match player {
                Player::White => pos / 8 - 1,
                Player::Black => pos / 8 + 1,
            }
        };
        let from_file = pos % 8;

        if to_rank == 0 || to_rank == 7 {
            // we can prob only do queen and knight, should help reduce tree without hurting performance
            // let promotion_pieces = vec![Piece::Queen(player), Piece::Knight(player)];
            let promotion_pieces = vec![
                Piece::Queen(player),
                Piece::Rook(player),
                Piece::Knight(player),
                Piece::Bishop(player),
            ];
            for promotion in promotion_pieces {
                promotion_moves.push(Move {
                    from_rank,
                    from_file,
                    to_rank,
                    to_file,
                    promotion,
                })
            }
        } else {
            let promotion = Piece::Pawn(player);
            non_promotion_moves.push(Move {
                from_rank,
                from_file,
                to_rank,
                to_file,
                promotion,
            })
        }
    }

    return (non_promotion_moves, promotion_moves);
}

pub fn generate_pawn_double_forward_moves(game: &GameState, player: Player) -> Vec<Move> {
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

    let mut moves = Vec::<Move>::new();

    while valid != 0 {
        let pos = valid.pop_mut();

        let Square {
            rank: to_rank,
            file: to_file,
        } = Square::from(pos);
        let Square {
            rank: from_rank,
            file: from_file,
        } = Square::from({
            match player {
                Player::White => pos - 16,
                Player::Black => pos + 16,
            }
        });

        let promotion = Piece::Pawn(player);
        moves.push(Move {
            from_rank,
            from_file,
            to_rank,
            to_file,
            promotion,
        })
    }

    return moves;
}

pub fn generate_pawn_attack_moves(game: &GameState, player: Player) -> (Vec<Move>, Vec<Move>) {
    let mut pawns = game
        .bitboards
        .get_board_by_piece(Piece::Pawn(player))
        .clone();
    let opposite_occupied = game.bitboards.get_occupied_by_player(player.opponent());

    // pawns.print_board();

    let mut non_promotion_moves: Vec<Move> = vec![];
    let mut promotion_moves: Vec<Move> = vec![];

    while pawns != 0 {
        let pos = pawns.pop_mut();

        let square = Square::from(pos);
        // println!("{} {:?}", pos, square);

        let Square { rank, file } = square;
        let mut to_check: Vec<Square> = vec![];
        // left file capture
        if file > 0 && rank != 7 && rank != 0 {
            to_check.push(Square {
                rank: match player {
                    Player::White => rank + 1,
                    Player::Black => rank - 1,
                },
                file: file - 1,
            });
        }
        // right file capture
        if file < 7 && rank != 7 && rank != 0 {
            to_check.push(Square {
                rank: match player {
                    Player::White => rank + 1,
                    Player::Black => rank - 1,
                },
                file: file + 1,
            });
        }

        for to in to_check {
            // println!("{:?} {:?}", square, to);
            // occupied or en passant capture
            if opposite_occupied.get(to.into())
                || (game.enpassant_square.exists
                    && to.rank == game.enpassant_square.pos.rank
                    && to.file == game.enpassant_square.pos.file)
            {
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
                    for promotion in promotion_pieces {
                        promotion_moves.push(Move {
                            from_rank: rank,
                            from_file: file,
                            to_rank: to.rank,
                            to_file: to.file,
                            promotion,
                        })
                    }
                } else {
                    non_promotion_moves.push(Move {
                        from_rank: rank,
                        from_file: file,
                        to_rank: to.rank,
                        to_file: to.file,
                        promotion: Piece::Pawn(player),
                    })
                }
            }
        }
    }

    return (non_promotion_moves, promotion_moves);
}
