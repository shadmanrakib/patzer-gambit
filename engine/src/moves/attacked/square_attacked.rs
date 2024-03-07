use crate::{
    moves::precalculate::{cache::PrecalculatedCache, magic_bitboards::hash_with_magic},
    state::{boards::BitBoard, game::GameState, pieces::Piece, player::Player, square::Square},
};

// #[inline(always)]
pub fn times_square_attacked(
    pos: i8,
    attacker: Player,
    game: &GameState,
    cache: &PrecalculatedCache,
) -> i8 {
    let mut attacked_count: i8 = 0;

    let occupied = game.bitboards.occupied.clone();

    let Square { rank, file } = Square::from(pos);

    // knight
    let knight_move_mask = cache.knight_moves_masks[pos as usize];
    let attacking_knights =
        knight_move_mask & game.bitboards.get_board_by_piece(attacker, Piece::Knight);
    attacked_count += attacking_knights.count_ones() as i8;

    // println!("knights");
    // knight_move_mask.print_board();
    // attacking_knights.print_board();

    // rook and queen vertical and horizontal
    let rook_magic_index = hash_with_magic(
        cache.rook_magics[pos as usize],
        occupied,
    );
    let rook_moves_mask = cache.rook_magic_attack_tables[rook_magic_index];
    let attacking_rooks =
        rook_moves_mask & game.bitboards.get_board_by_piece(attacker, Piece::Rook);
    let attacking_queens_straight =
        rook_moves_mask & game.bitboards.get_board_by_piece(attacker, Piece::Queen);

    attacked_count += attacking_rooks.count_ones() as i8;
    attacked_count += attacking_queens_straight.count_ones() as i8;

    // println!("rooks & q straight");
    // rook_moves_mask.print_board();
    // attacking_rooks.print_board();
    // attacking_queens_straight.print_board();

    // bishop and queen diagonal
    let bishop_magic_index = hash_with_magic(
        cache.bishop_magics[pos as usize],
        occupied,
    );
    let bishop_moves_mask = cache.bishop_magic_attack_tables[bishop_magic_index];

    let attacking_bishops =
        bishop_moves_mask & game.bitboards.get_board_by_piece(attacker, Piece::Bishop);
    let attacking_queens_diagonal =
        bishop_moves_mask & game.bitboards.get_board_by_piece(attacker, Piece::Queen);

    attacked_count += attacking_bishops.count_ones() as i8;
    attacked_count += attacking_queens_diagonal.count_ones() as i8;

    // println!("bishops & q diag");
    // bishop_moves_mask.print_board();
    // attacking_bishops.print_board();
    // attacking_queens_diagonal.print_board();

    // pawn attack
    let opponent_pawns = game.bitboards.get_board_by_piece(attacker, Piece::Pawn);
    // println!("pawns");
    // opponent_pawns.print_board();
    // let mut attacking_pawns = 0;
    // left file capture
    let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
    let attacking_pawns = opponent_pawns & attacking_mask;
    attacked_count += attacking_pawns.count_ones() as i8;

    // let attacking_pawn_rank = match attacker {
    //     Player::Black => rank + 1,
    //     Player::White => rank - 1,
    // };
    // if file > 0 && attacking_pawn_rank >= 0 && attacking_pawn_rank <= 7 {
    //     let to_check = Square {
    //         rank: attacking_pawn_rank,
    //         file: file - 1,
    //     };
    //     if opponent_pawns.get(to_check.into()) {
    //         attacked_count += 1;
    //         // println!("pawn left {}", <Square as Into<i8>>::into(to_check));
    //         attacking_pawns.set(to_check.into());
    //     }
    // }
    // // right file capture
    // if file < 7 && attacking_pawn_rank >= 0 && attacking_pawn_rank <= 7 {
    //     let to_check = Square {
    //         rank: match attacker {
    //             Player::Black => rank + 1,
    //             Player::White => rank - 1,
    //         },
    //         file: file + 1,
    //     };
    //     if opponent_pawns.get(to_check.into()) {
    //         attacked_count += 1;
    //         // println!("pawn right {}", <Square as Into<i8>>::into(to_check));
    //         attacking_pawns.set(to_check.into());
    //     }
    // }
    // // attacking_pawns.print_board();

    // king attack
    let king_move_mask = cache.king_moves_masks[pos as usize];
    let attacking_king = king_move_mask & game.bitboards.get_board_by_piece(attacker, Piece::King);
    attacked_count += attacking_king.count_ones() as i8;

    return attacked_count;
}

// #[inline]
pub fn is_square_attacked(
    pos: i8,
    attacker: Player,
    game: &GameState,
    cache: &PrecalculatedCache,
) -> bool {
    return times_square_attacked(pos, attacker, game, cache) > 0;
    // let occupied = game.bitboards.get_occupied().clone();

    // let Square { rank, file } = Square::from(pos);

    // // knight
    // let knight_move_mask = cache.knight_moves_masks[pos as usize];
    // let attacking_knights =
    //     knight_move_mask & game.bitboards.get_board_by_piece(Piece::Knight(attacker));
    
    // if attacking_knights != 0 {
    //     return true;
    // }

    // // rook and queen vertical and horizontal
    // let rook_magic_index = hash_with_magic(
    //     cache.rook_potential_blockers_masks[pos as usize],
    //     occupied,
    //     cache.rook_magics[pos as usize],
    //     cache.rook_bit_counts[pos as usize],
    // );
    // let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
    // let attacking_rooks =
    //     rook_moves_mask & game.bitboards.get_board_by_piece(Piece::Rook(attacker));
    // let attacking_queens_straight =
    //     rook_moves_mask & game.bitboards.get_board_by_piece(Piece::Queen(attacker));

    // if attacking_rooks != 0 || attacking_queens_straight != 0 {
    //     return true;
    // }

    // // bishop and queen diagonal
    // let bishop_magic_index = hash_with_magic(
    //     cache.bishop_potential_blockers_masks[pos as usize],
    //     occupied,
    //     cache.bishop_magics[pos as usize],
    //     cache.bishop_bit_counts[pos as usize],
    // );
    // let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];

    // let attacking_bishops =
    //     bishop_moves_mask & game.bitboards.get_board_by_piece(Piece::Bishop(attacker));
    // let attacking_queens_diagonal =
    //     bishop_moves_mask & game.bitboards.get_board_by_piece(Piece::Queen(attacker));

    // if attacking_bishops != 0 || attacking_queens_diagonal != 0 {
    //     return true;
    // }

    // // pawn attack
    // let opponent_pawns = game.bitboards.get_board_by_piece(Piece::Pawn(attacker));
 
    // let attacking_pawn_rank = match attacker {
    //     Player::Black => rank + 1,
    //     Player::White => rank - 1,
    // };
    // if file > 0 && attacking_pawn_rank >= 0 && attacking_pawn_rank <= 7 {
    //     let to_check = Square {
    //         rank: attacking_pawn_rank,
    //         file: file - 1,
    //     };
    //     if opponent_pawns.get(to_check.into()) {
    //         return true;
    //     }
    // }
    // // right file capture
    // if file < 7 && attacking_pawn_rank >= 0 && attacking_pawn_rank <= 7 {
    //     let to_check = Square {
    //         rank: match attacker {
    //             Player::Black => rank + 1,
    //             Player::White => rank - 1,
    //         },
    //         file: file + 1,
    //     };
    //     if opponent_pawns.get(to_check.into()) {
    //         return true;
    //     }
    // }

    // // king attack
    // let king_move_mask = cache.king_moves_masks[pos as usize];
    // let attacking_king = king_move_mask & game.bitboards.get_board_by_piece(Piece::King(attacker));

    // return attacking_king.count_ones() != 0;
}
