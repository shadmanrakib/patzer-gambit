use crate::{
    boards::BitBoard,
    magics::hash_with_magic,
    masks::{
        calculate_bishop_bit_counts, calculate_rook_bit_counts, RANK_1_MASK, RANK_2_MASK,
        RANK_7_MASK, RANK_8_MASK,
    },
    moves::{Move, MoveList},
    pieces::Piece,
    player::Player,
    position::{CastlePermissions, PositionState},
    square::Square,
};

use std::time::SystemTime;

use xorshift::{Rand, SeedableRng, SplitMix64, Xoroshiro128};

use crate::{
    magics::{find_bishop_magic_numbers, find_rook_magic_numbers, Magic},
    masks::{
        create_bishop_potential_blockers_mask, create_king_moves_masks, create_knight_moves_masks,
        create_pawn_attack_moves_masks, create_rook_potential_blockers_mask,
    },
};

#[derive(Debug)]
pub struct MoveGenerator {
    pub pawn_attack_moves_mask: [[u64; 64]; 2],
    pub knight_moves_masks: [u64; 64],
    pub king_moves_masks: [u64; 64],
    pub rook_magics: [Magic; 64],
    pub rook_magic_attack_tables: Vec<u64>,
    pub bishop_magics: [Magic; 64],
    pub bishop_magic_attack_tables: Vec<u64>,
}

impl MoveGenerator {
    pub fn create() -> MoveGenerator {
        let pawn_attack_moves_mask = create_pawn_attack_moves_masks();

        let king_moves_masks = create_king_moves_masks();
        let knight_moves_masks = create_knight_moves_masks();

        let rook_bit_counts = calculate_rook_bit_counts();
        let bishop_bit_counts = calculate_bishop_bit_counts();

        let mut rook_potential_blockers_masks = [0u64; 64];
        let mut bishop_potential_blockers_masks = [0u64; 64];

        for pos in 0..64 {
            rook_potential_blockers_masks[pos] = create_rook_potential_blockers_mask(pos as i8);
            bishop_potential_blockers_masks[pos] = create_bishop_potential_blockers_mask(pos as i8);
        }

        let seed: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut sm: SplitMix64 = SeedableRng::from_seed(seed);
        let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

        let (rook_magics, rook_magic_attack_tables) =
            find_rook_magic_numbers(&mut rng, &rook_bit_counts, &rook_potential_blockers_masks);
        let (bishop_magics, bishop_magic_attack_tables) = find_bishop_magic_numbers(
            &mut rng,
            &bishop_bit_counts,
            &bishop_potential_blockers_masks,
        );

        MoveGenerator {
            pawn_attack_moves_mask,
            knight_moves_masks,
            king_moves_masks,
            rook_magics,
            rook_magic_attack_tables,
            bishop_magics,
            bishop_magic_attack_tables,
        }
    }

    pub fn generate_moves(&self, position: &PositionState) -> MoveList {
        let mut ml = MoveList::new();
        generate_pseudolegal_moves(&mut ml, position, position.side_to_move, self, false);
        ml
    }

    pub fn generate_captures(&self, position: &PositionState) -> MoveList {
        let mut ml = MoveList::new();
        generate_pseudolegal_moves(&mut ml, position, position.side_to_move, self, true);
        ml
    }
}

// #[inline(always)]
fn generate_pseudolegal_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
    only_captures: bool,
) {
    generate_queen_moves(movelist, position, player, generator, only_captures);
    generate_rook_moves(movelist, position, player, generator, only_captures);
    generate_bishop_moves(movelist, position, player, generator, only_captures);
    generate_knight_moves(movelist, position, player, generator, only_captures);
    generate_pawn_moves(movelist, position, player, generator, only_captures);
    generate_king_moves(movelist, position, player, generator, only_captures);
    if !only_captures {
        generate_castling_moves(movelist, position, player, generator);
    }
}

// BISHOPS
// #[inline(always)]
fn generate_bishop_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
    only_captures: bool,
) {
    let mut bishops = position
        .bitboards
        .get_board_by_piece(player, Piece::Bishop)
        .clone();
    let occupied = &position.bitboards.occupied;
    let opponent_occupied = &position.bitboards.pos_to_player[player.opponent() as usize];

    while bishops != 0 {
        let from = bishops.pop_mut();

        let magic_index = hash_with_magic(generator.bishop_magics[from as usize], &occupied);
        let moves_mask = generator.bishop_magic_attack_tables[magic_index];

        let valid_silents = moves_mask & !position.bitboards.occupied;
        let valid_captures = moves_mask & opponent_occupied;

        movelist.push_multiple_moves(
            position,
            from,
            valid_captures,
            Piece::Bishop,
            Piece::Empty,
            false,
            true,
            false,
            false,
            false,
        );

        if !only_captures {
            movelist.push_multiple_moves(
                position,
                from,
                valid_silents,
                Piece::Bishop,
                Piece::Empty,
                false,
                false,
                false,
                false,
                false,
            );
        }
    }
}

// ROOKS
// #[inline(always)]
fn generate_rook_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
    only_captures: bool,
) {
    let mut rooks = position
        .bitboards
        .get_board_by_piece(player, Piece::Rook)
        .clone();
    let occupied = &position.bitboards.occupied;
    let opponent_occupied = &position.bitboards.pos_to_player[player.opponent() as usize];

    while rooks != 0 {
        let from = rooks.pop_mut();

        let magic_index = hash_with_magic(generator.rook_magics[from as usize], occupied);
        let moves_mask = generator.rook_magic_attack_tables[magic_index];

        let valid_silents = moves_mask & !occupied;
        let valid_captures = moves_mask & opponent_occupied;

        movelist.push_multiple_moves(
            position,
            from,
            valid_captures,
            Piece::Rook,
            Piece::Empty,
            false,
            true,
            false,
            false,
            false,
        );

        if !only_captures {
            movelist.push_multiple_moves(
                position,
                from,
                valid_silents,
                Piece::Rook,
                Piece::Empty,
                false,
                false,
                false,
                false,
                false,
            );
        }
    }
}

// QUEENS
// #[inline(always)]
fn generate_queen_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
    only_captures: bool,
) {
    let mut queens = position
        .bitboards
        .get_board_by_piece(player, Piece::Queen)
        .clone();
    let occupied = &position.bitboards.occupied;
    let opponent_occupied = &position.bitboards.pos_to_player[player.opponent() as usize];

    while queens != 0 {
        let from = queens.pop_mut();

        let rook_magic_index = hash_with_magic(generator.rook_magics[from as usize], occupied);
        let rook_moves_mask = generator.rook_magic_attack_tables[rook_magic_index];

        let bishop_magic_index = hash_with_magic(generator.bishop_magics[from as usize], occupied);
        let bishop_moves_mask = generator.bishop_magic_attack_tables[bishop_magic_index];

        let moves_mask = rook_moves_mask | bishop_moves_mask;

        let valid_silents = moves_mask & !position.bitboards.occupied;
        let valid_captures = moves_mask & opponent_occupied;

        movelist.push_multiple_moves(
            position,
            from,
            valid_captures,
            Piece::Queen,
            Piece::Empty,
            false,
            true,
            false,
            false,
            false,
        );

        if !only_captures {
            movelist.push_multiple_moves(
                position,
                from,
                valid_silents,
                Piece::Queen,
                Piece::Empty,
                false,
                false,
                false,
                false,
                false,
            );
        }
    }
}

// KNIGHTS
// #[inline(always)]
fn generate_knight_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
    only_captures: bool,
) {
    let mut knights = position
        .bitboards
        .get_board_by_piece(player, Piece::Knight)
        .clone();
    let opponent_occupied = position.bitboards.pos_to_player[player.opponent() as usize];

    while knights != 0 {
        let from = knights.pop_mut();

        let moves_mask = generator.knight_moves_masks[from as usize];
        let valid_silents = moves_mask & !position.bitboards.occupied;
        let valid_captures = moves_mask & opponent_occupied;

        movelist.push_multiple_moves(
            position,
            from,
            valid_captures,
            Piece::Knight,
            Piece::Empty,
            false,
            true,
            false,
            false,
            false,
        );

        if !only_captures {
            movelist.push_multiple_moves(
                position,
                from,
                valid_silents,
                Piece::Knight,
                Piece::Empty,
                false,
                false,
                false,
                false,
                false,
            );
        }
    }
}

// PAWNS
// #[inline(always)]
fn generate_pawn_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
    only_captures: bool,
) {
    generate_pawn_attack_moves(movelist, position, player, generator);
    if !only_captures {
        generate_pawn_double_forward_moves(movelist, position, player);
        generate_pawn_single_forward_moves(movelist, position, player);
    }
}

// #[inline(always)]
fn generate_pawn_single_forward_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
) {
    let pawnboard = position.bitboards.get_board_by_piece(player, Piece::Pawn);
    let can_move = pawnboard & !RANK_1_MASK & !RANK_8_MASK;
    let moved_forward = match player {
        Player::White => can_move << 8,
        Player::Black => can_move >> 8,
    };
    let mut valid = moved_forward & !position.bitboards.occupied;

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
            let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop];
            for promotion_piece in promotion_pieces {
                movelist.push(Move {
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
                    score: 0,
                })
            }
        } else {
            movelist.push(Move {
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
                score: 0,
            })
        }
    }
}

// #[inline(always)]
pub fn generate_pawn_double_forward_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
) {
    let pawnboard = position.bitboards.get_board_by_piece(player, Piece::Pawn);
    let occupied = position.bitboards.occupied;

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

        movelist.push(Move {
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
            score: 0,
        })
    }
}

// #[inline(always)]
pub fn generate_pawn_attack_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
) {
    let mut pawns = position
        .bitboards
        .get_board_by_piece(player, Piece::Pawn)
        .clone();
    let opposite_occupied = position.bitboards.pos_to_player[player.opponent() as usize];
    // pawns.print_board();

    while pawns != 0 {
        let pos = pawns.pop_mut();
        let attacking_mask = generator.pawn_attack_moves_mask[player as usize][pos as usize];
        let mut attacking = opposite_occupied & attacking_mask;
        let mut attacking_enpassant = position.enpassant_square & attacking_mask;

        while attacking != 0 {
            let to = attacking.pop_mut();
            generate_pawn_attack_moves_helper(movelist, position, pos, to, false);
        }
        while attacking_enpassant != 0 {
            let to = attacking_enpassant.pop_mut();
            generate_pawn_attack_moves_helper(movelist, position, pos, to, true);
        }
    }
}

// #[inline(always)]
pub fn generate_pawn_attack_moves_helper(
    movelist: &mut MoveList,
    position: &PositionState,
    from_pos: i8,
    to: i8,
    enpassant: bool,
) {
    let captured_piece = if enpassant {
        Piece::Pawn
    } else {
        position.bitboards.pos_to_piece[to as usize]
    };

    // promotion on capture
    if to <= 7 || to >= 56 {
        // we can prob only do queen and knight, should help reduce tree without hurting performance
        // let promotion_pieces = vec![Piece::Queen(player), Piece::Knight(player)];
        let promotion_pieces = [Piece::Queen, Piece::Rook, Piece::Knight, Piece::Bishop];
        for promotion_piece in promotion_pieces {
            movelist.push(Move {
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
                score: 0,
            })
        }
    } else {
        movelist.push(Move {
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
            score: 0,
        })
    }
}

// KING
// #[inline(always)]
pub fn generate_king_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
    only_captures: bool,
) {
    let mut kings = position
        .bitboards
        .get_board_by_piece(player, Piece::King)
        .clone();
    let opponent_occupied = position.bitboards.pos_to_player[player.opponent() as usize];

    // should only really run once
    while kings != 0 {
        let from = kings.pop_mut();

        let moves_mask = generator.king_moves_masks[from as usize];
        let valid_silents = moves_mask & !position.bitboards.occupied;
        let valid_captures = moves_mask & opponent_occupied;

        movelist.push_multiple_moves(
            position,
            from,
            valid_captures,
            Piece::King,
            Piece::Empty,
            false,
            true,
            false,
            false,
            false,
        );

        if !only_captures {
            movelist.push_multiple_moves(
                position,
                from,
                valid_silents,
                Piece::King,
                Piece::Empty,
                false,
                false,
                false,
                false,
                false,
            );
        }
    }
}

// CASTLING
// #[inline(always)]
pub fn generate_castling_moves(
    movelist: &mut MoveList,
    position: &PositionState,
    player: Player,
    generator: &MoveGenerator,
) {
    // can't castle if in check
    if position.in_check(player, generator) {
        return;
    }

    let occupied = position.bitboards.occupied;
    let opponent: Player = player.opponent();

    match player {
        Player::White => {
            if position.bitboards.pos_to_piece[4] != Piece::King {
                return;
            }

            // check if 5 is attacked, king side between transition
            // make sure in between is also empty
            if position.castle_permissions.is_allowed(u8::WHITE_KING_SIDE)
                && !occupied.get(5)
                && !occupied.get(6)
                && !position.is_square_attacked(5, opponent, generator)
            {
                // we will mark the king movement
                if position.bitboards.pos_to_piece[7] == Piece::Rook {
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
            if position.castle_permissions.is_allowed(u8::WHITE_QUEEN_SIDE)
                && !occupied.get(1)
                && !occupied.get(2)
                && !occupied.get(3)
                && !position.is_square_attacked(3, opponent, generator)
            {
                if position.bitboards.pos_to_piece[0] == Piece::Rook {
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
            if position.bitboards.pos_to_piece[60] != Piece::King {
                return;
            }

            if position.castle_permissions.is_allowed(u8::BLACK_KING_SIDE)
                && !occupied.get(61)
                && !occupied.get(62)
                && !position.is_square_attacked(61, opponent, generator)
            {
                // check if 61 is attacked, king side between transition
                // make sure in between is also empty

                if position.bitboards.pos_to_piece[63] == Piece::Rook {
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
            if position.castle_permissions.is_allowed(u8::BLACK_QUEEN_SIDE)
                && !occupied.get(57)
                && !occupied.get(58)
                && !occupied.get(59)
                && !position.is_square_attacked(59, opponent, generator)
            {
                // check if 58 is attacked, queen side between transition
                // make sure in between is also empty
                if position.bitboards.pos_to_piece[56] == Piece::Rook {
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
