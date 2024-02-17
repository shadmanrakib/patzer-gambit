use crate::constants::masks::{RANK_1_MASK, RANK_2_MASK, RANK_7_MASK, RANK_8_MASK};
use crate::moves::move_data::MoveItem;
use crate::state::bitboards::BitBoard;
use crate::state::square::Square;
use crate::state::{game::GameState, pieces::Piece, player::Player};

// use crate::masks::RANK_1_MASK;

// single forward non promotion, double, promotion, capture
#[inline(always)]
pub fn generate_pawn_moves(game: &GameState, player: Player) -> Vec<MoveItem> {
    let mut single_forward_moves = generate_pawn_single_forward_moves(game, player);
    let mut double_foreward_moves = generate_pawn_double_forward_moves(game, player);
    let mut capture_moves = generate_pawn_attack_moves(game, player);

    let mut moves = Vec::<MoveItem>::new();
    moves.append(&mut capture_moves);
    moves.append(&mut single_forward_moves);
    moves.append(&mut double_foreward_moves);

    return moves;
}

pub fn generate_pawn_single_forward_moves(game: &GameState, player: Player) -> Vec<MoveItem> {
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

    let mut non_promotion_moves = Vec::<MoveItem>::new();
    let mut promotion_moves = Vec::<MoveItem>::new();

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
                promotion_moves.push(MoveItem {
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
            let promotion = Piece::Pawn(player);
            non_promotion_moves.push(MoveItem {
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

    // temporarily as i figure out how i want to structure things
    let mut moves = Vec::<MoveItem>::new();
    moves.append(&mut non_promotion_moves);
    moves.append(&mut promotion_moves);

    return moves;
}

pub fn generate_pawn_double_forward_moves(game: &GameState, player: Player) -> Vec<MoveItem> {
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

    let mut moves = Vec::<MoveItem>::new();

    while valid != 0 {
        let pos = valid.pop_mut();

        let to = Square::from(pos);
        let from = Square::from({
            match player {
                Player::White => pos - 16,
                Player::Black => pos + 16,
            }
        });

        let promotion = Piece::Pawn(player);
        moves.push(MoveItem {
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

    return moves;
}

pub fn generate_pawn_attack_moves(game: &GameState, player: Player) -> Vec<MoveItem> {
    let mut pawns = game
        .bitboards
        .get_board_by_piece(Piece::Pawn(player))
        .clone();
    let opposite_occupied = game.bitboards.get_occupied_by_player(player.opponent());

    // pawns.print_board();

    let mut non_promotion_moves: Vec<MoveItem> = vec![];
    let mut promotion_moves: Vec<MoveItem> = vec![];

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
            let can_enpassant = game.enpassant_square.exists
                && to.rank == game.enpassant_square.pos.rank
                && to.file == game.enpassant_square.pos.file;

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
                        promotion_moves.push(MoveItem {
                            from_pos: pos,
                            to_pos: to.into(),
                            piece: Piece::Pawn(player),
                            promotion_piece,
                            captured_piece: game.bitboards.get_piece_by_bit_pos(to.into()),
                            promoting: true,
                            capturing: true,
                            double: false,
                            enpassant: can_enpassant,
                            castling: false,
                            score: 0.,
                        })
                    }
                } else {
                    non_promotion_moves.push(MoveItem {
                        from_pos: pos,
                        to_pos: to.into(),
                        piece: Piece::Pawn(player),
                        promotion_piece: Piece::Empty,
                        captured_piece: game.bitboards.get_piece_by_bit_pos(to.into()),
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
    }

    // temporarily as i figure out how i want to structure things
    let mut moves = Vec::<MoveItem>::new();
    moves.append(&mut non_promotion_moves);
    moves.append(&mut promotion_moves);

    return moves;
}
