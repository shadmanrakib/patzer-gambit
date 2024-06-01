use crate::{
    boards::{BitBoard, Boards},
    magics::hash_with_magic,
    movegen::MoveGenerator,
    moves::{Move, SimpleMove},
    pieces::Piece,
    player::Player,
    position::PositionState,
    searchinfo::SearchInfo,
    settings::MAX_KILLER_MOVES,
};

const MAX_SCORE: i16 = std::i16::MAX;

const MMV_LVA_OFFSET: i16 = std::i16::MAX - 1 - 10000;
const MIN_SCORE: i16 = std::i16::MIN;

const MVV_LVA_SCORE: [[i16; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // victim None, attacker K, Q, R, B, N, P, None
    [0, 150, 140, 130, 120, 110, 100], // victim P, attacker K, Q, R, B, N, P, None
    [0, 250, 240, 230, 220, 210, 200], // victim K, attacker K, Q, R, B, N, P, None
    [0, 350, 340, 330, 320, 310, 300], // victim B, attacker K, Q, R, B, N, P, None
    [0, 450, 440, 430, 420, 410, 400], // victim R, attacker K, Q, R, B, N, P, None
    [0, 550, 540, 530, 520, 510, 500], // victim Q, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0], // victim K, attacker K, Q, R, B, N, P, None
];

#[inline(always)]
fn mmv_lva(victim: Piece, attacker: Piece) -> i16 {
    return MVV_LVA_SCORE[victim as usize][attacker as usize];
}

pub fn score_mmv_lva(move_item: &mut Move) {
    if move_item.capturing {
        move_item.score = mmv_lva(move_item.captured_piece, move_item.piece);
    } else {
        move_item.score = MIN_SCORE;
    }
}

pub fn score_tt_mmv_lva_killer(
    move_item: &mut Move,
    search_cache: &mut SearchInfo,
    ply: usize,
    tt_move: &SimpleMove,
) {
    if tt_move.is_similar(move_item) {
        move_item.score = MAX_SCORE;
    } else if move_item.capturing {
        move_item.score = MVV_LVA_SCORE[move_item.captured_piece as usize]
            [move_item.piece as usize]
            + MMV_LVA_OFFSET;
    } else {
        for i in 0..MAX_KILLER_MOVES {
            if search_cache.killer_moves[ply][i].is_similar(move_item) {
                move_item.score = MMV_LVA_OFFSET - 1000 - ((i * 10) as i16);
                break;
            }
        }
    }
}

const PIECE_POINTS: [i16; 7] = [
    0,     // Empty
    100,   // Pawn
    320,   // Knight
    325,   // Bishop
    500,   // Rook
    925,   // Queen
    20000, // King
];

pub fn static_exchange_evaluation(
    position: &PositionState,
    to: i8,
    attacker_pos: i8,
    generator: &MoveGenerator,
) -> i16 {
    let boards = position.boards.clone();

    let mut occ = boards.occupied;

    let mut from_bitset: u64 = 1 << attacker_pos;
    let mut from = attacker_pos;

    let mut attacks = get_attacks_on_square(&boards, occ, to, generator);
    let mut all_attackers = attacks;

    // create a swap list for us to calculate values of the attacks
    // we will then use this swap list to get back the results
    // of a negamax on the position (essentially a dp implementation of negamax)
    // we are able to do this because the search does not branch
    // and always takes the min attacker of the attacking side
    let mut swap_list = [0_i16; 32];
    let mut depth = 0;
    let mut player = position.side_to_move.opponent();

    // value of first piece to be captured
    swap_list[0] = PIECE_POINTS[boards.pos_to_piece[to as usize] as usize];

    while from_bitset != 0 && depth < 31 {
        depth += 1;

        // we will assume the attacker from last stage is captured
        let captured = boards.pos_to_piece[from as usize];
        swap_list[depth] = PIECE_POINTS[captured as usize] - swap_list[depth - 1];

        // if captured == Piece::King {
        //     break;
        // }

        // pruning, attacking would be always worse, so end now
        // that is the opponent can just stop taking and be up
        if std::cmp::max(-swap_list[depth - 1], swap_list[depth]) < 0 {
            break;
        };

        // used up peice, so "remove", allowing for x-rays to be discovered
        attacks ^= from_bitset;
        occ ^= from_bitset;

        let sliding = get_sliding_attacks(&boards, occ, to, generator);
        attacks |= sliding;
        all_attackers |= attacks;

        // find the next attacker
        (from_bitset, from) = get_least_valauble_attacker(&boards, &attacks, player);
        player = player.opponent();
    }

    // negamax
    depth -= 1;
    while depth > 0 {
        swap_list[depth - 1] = -std::cmp::max(-swap_list[depth - 1], swap_list[depth]);
        depth -= 1;
    }

    swap_list[0]
}
// position fen 1k1r4/1pp4p/p7/4p3/8/P5P1/1PP4P/2K1R3 w - - 0 1
// s e5 e1
// position fen 1k1r3q/1ppn3p/p4b2/4p3/8/P2N2P1/1PP1R1BP/2K1Q3 w - - 0 1
// s e5 d3
fn get_sliding_attacks(boards: &Boards, occ: u64, to: i8, generator: &MoveGenerator) -> u64 {
    let mut attackers = 0;
    let rooks = boards.get_board_by_piece(Player::White, Piece::Rook)
        | boards.get_board_by_piece(Player::Black, Piece::Rook);
    let bishops = boards.get_board_by_piece(Player::White, Piece::Bishop)
        | boards.get_board_by_piece(Player::Black, Piece::Bishop);
    let queens = boards.get_board_by_piece(Player::White, Piece::Queen)
        | boards.get_board_by_piece(Player::Black, Piece::Queen);

    // rook and queen vertical and horizontal
    let rook_magic_index = hash_with_magic(generator.rook_magics[to as usize], &occ);
    let straight_moves_mask = generator.rook_magic_attack_tables[rook_magic_index];
    attackers |= straight_moves_mask & (rooks | queens);

    // bishop and queen diagonal
    let bishop_magic_index = hash_with_magic(generator.bishop_magics[to as usize], &occ);
    let diagnol_moves_mask = generator.bishop_magic_attack_tables[bishop_magic_index];
    attackers |= diagnol_moves_mask & (bishops | queens);

    attackers & occ
}
fn get_attacks_on_square(boards: &Boards, occ: u64, to: i8, generator: &MoveGenerator) -> u64 {
    let mut attackers = get_sliding_attacks(boards, occ, to, generator);

    let kings = boards.get_board_by_piece(Player::White, Piece::King)
        | boards.get_board_by_piece(Player::Black, Piece::King);
    let knights = boards.get_board_by_piece(Player::White, Piece::Knight)
        | boards.get_board_by_piece(Player::Black, Piece::Knight);
    let wpawns = boards.get_board_by_piece(Player::White, Piece::Pawn);
    let bpawns = boards.get_board_by_piece(Player::Black, Piece::Pawn);

    // knight
    let knight_move_mask = generator.knight_moves_masks[to as usize];
    attackers |= knight_move_mask & knights;

    // pawn attack, doesn't include enpassants
    let wpawn_attacking_mask =
        generator.pawn_attack_moves_mask[Player::White.opponent() as usize][to as usize] & wpawns;
    let bpawn_attacking_mask =
        generator.pawn_attack_moves_mask[Player::Black.opponent() as usize][to as usize] & bpawns;
    attackers |= wpawn_attacking_mask | bpawn_attacking_mask;

    // king attack
    let king_move_mask = generator.king_moves_masks[to as usize];
    let attacking_king = king_move_mask & kings;
    attackers |= attacking_king;

    attackers
}

fn get_least_valauble_attacker(boards: &Boards, attacks: &u64, player: Player) -> (u64, i8) {
    let pawns = boards.get_board_by_piece(player, Piece::Pawn);
    if pawns & attacks != 0 {
        let pos = (pawns & attacks).pop_mut();
        return (1 << pos, pos);
    }

    let knights = boards.get_board_by_piece(player, Piece::Knight);
    if knights & attacks != 0 {
        let pos = (knights & attacks).pop_mut();
        return (1 << pos, pos);
    }

    let bishops = boards.get_board_by_piece(player, Piece::Bishop);
    if bishops & attacks != 0 {
        let pos = (bishops & attacks).pop_mut();
        return (1 << pos, pos);
    }

    let rooks = boards.get_board_by_piece(player, Piece::Rook);
    if rooks & attacks != 0 {
        let pos = (rooks & attacks).pop_mut();
        return (1 << pos, pos);
    }

    let queens = boards.get_board_by_piece(player, Piece::Queen);
    if queens & attacks != 0 {
        let pos = (queens & attacks).pop_mut();
        return (1 << pos, pos);
    }

    let kings = boards.get_board_by_piece(player, Piece::King);
    if kings & attacks != 0 {
        let pos = (kings & attacks).pop_mut();
        return (1 << pos, pos);
    }

    (0, 64)
}
