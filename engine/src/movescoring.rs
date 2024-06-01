use crate::{
    boards::BitBoard,
    magics::hash_with_magic,
    movegen::MoveGenerator,
    moves::{Move, SimpleMove},
    pieces::Piece,
    player::Player,
    position::PositionState,
    searchinfo::SearchInfo,
    settings::MAX_KILLER_MOVES,
    square::Square,
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
    300,   // Knight
    310,   // Bishop
    500,   // Rook
    900,   // Queen
    20000, // King
];

#[derive(Debug, Clone, Copy)]
enum RayDirection {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
    NONE,
}
pub fn static_exchange_evaluation(
    position: &PositionState,
    pos: i8,
    attacker: Piece,
    attacker_pos: i8,
    generator: &MoveGenerator,
) -> i16 {
    let captured = position.bitboards.pos_to_piece[pos as usize];
    let eval = PIECE_POINTS[captured as usize];

    let swap_list = Vec::<i16>::with_capacity(32);

    let mut cur_piece = attacker;

    // we will need to get all the attackers
    let mut attackers = init_attackers(position, pos, generator);
    println!("{:?}", attackers);
    attackers = attackers
        .into_iter()
        .filter(|x| x.2 == attacker_pos)
        .collect();
    let mut white: Vec<&(Piece, RayDirection, i8)> = attackers
        .iter()
        .filter(|x| position.bitboards.pos_to_player[Player::White as usize].get(x.2))
        .collect();
    let black: Vec<&(Piece, RayDirection, i8)> = attackers
        .iter()
        .filter(|x| position.bitboards.pos_to_player[Player::Black as usize].get(x.2))
        .collect();

    println!("{:?}", attackers);
    println!("{:?}", white);
    println!("{:?}", black);

    eval
}

fn move_smallest_attacker_to_end(attackers: &mut Vec<&(Piece, RayDirection, i8)>) {
    let mut i = 0;
    let mut end = attackers.len() - 1;

    while i < end {
        if PIECE_POINTS[attackers[i].0 as usize] < PIECE_POINTS[attackers[i].1 as usize] {
            attackers.swap(i, end);
        }
        i += 1;
    }
}

fn ray_direction(a: i8, b: i8) -> RayDirection {
    let a_sq = Square::from(a);
    let b_sq = Square::from(b);

    let (dr, df) = (a_sq.rank - b_sq.rank, a_sq.file - b_sq.file);

    println!("{a} {b} {dr} {df}");

    match (dr, df) {
        (x, 0) if x > 0 => RayDirection::N, // Moving up a rank (North)
        (x, 0) if x < 0 => RayDirection::S, // Moving down a rank (South)
        (0, x) if x > 0 => RayDirection::E, // Moving right a file (East)
        (0, x) if x < 0 => RayDirection::W, // Moving left a file (West)
        (1, 1) => RayDirection::NE,         // Moving up a rank and right a file (Northeast)
        (1, -1) => RayDirection::NW,        // Moving up a rank and left a file (Northwest)
        (-1, 1) => RayDirection::SE,        // Moving down a rank and right a file (Southeast)
        (-1, -1) => RayDirection::SW,       // Moving down a rank and left a file (Southwest)
        _ => RayDirection::NONE,
    }
}

fn init_attackers(
    position: &PositionState,
    to: i8,
    generator: &MoveGenerator,
) -> Vec<(Piece, RayDirection, i8)> {
    let mut attackers = vec![];

    let occupied = position.bitboards.occupied;

    // knight
    let knight_move_mask = generator.knight_moves_masks[to as usize];
    let mut attacking_knights = knight_move_mask
        & (position
            .bitboards
            .get_board_by_piece(Player::White, Piece::Knight)
            | position
                .bitboards
                .get_board_by_piece(Player::Black, Piece::Knight));
    while attacking_knights != 0 {
        let from = attacking_knights.pop_mut();
        attackers.push((Piece::Knight, ray_direction(from, to), from))
    }

    // rook and queen vertical and horizontal
    let rook_magic_index = hash_with_magic(generator.rook_magics[to as usize], &occupied);
    let rook_moves_mask = generator.rook_magic_attack_tables[rook_magic_index];
    let mut attacking_rooks = rook_moves_mask
        & (position
            .bitboards
            .get_board_by_piece(Player::White, Piece::Rook)
            | position
                .bitboards
                .get_board_by_piece(Player::Black, Piece::Rook));
    let mut attacking_queens_straight = rook_moves_mask
        & (position
            .bitboards
            .get_board_by_piece(Player::White, Piece::Queen)
            | position
                .bitboards
                .get_board_by_piece(Player::Black, Piece::Queen));
    while attacking_rooks != 0 {
        let from = attacking_rooks.pop_mut();
        attackers.push((Piece::Rook, ray_direction(from, to), from))
    }
    while attacking_queens_straight != 0 {
        let from = attacking_queens_straight.pop_mut();
        attackers.push((Piece::Queen, ray_direction(from, to), from))
    }

    // bishop and queen diagonal
    let bishop_magic_index = hash_with_magic(generator.bishop_magics[to as usize], &occupied);
    let bishop_moves_mask = generator.bishop_magic_attack_tables[bishop_magic_index];

    let mut attacking_bishops = bishop_moves_mask
        & (position
            .bitboards
            .get_board_by_piece(Player::White, Piece::Bishop)
            | position
                .bitboards
                .get_board_by_piece(Player::Black, Piece::Bishop));
    let mut attacking_queens_diagonal = bishop_moves_mask
        & (position
            .bitboards
            .get_board_by_piece(Player::White, Piece::Queen)
            | position
                .bitboards
                .get_board_by_piece(Player::Black, Piece::Queen));

    while attacking_bishops != 0 {
        let from = attacking_bishops.pop_mut();
        attackers.push((Piece::Bishop, ray_direction(from, to), from))
    }
    while attacking_queens_diagonal != 0 {
        let from = attacking_queens_diagonal.pop_mut();
        attackers.push((Piece::Queen, ray_direction(from, to), from))
    }

    // pawn attack, doesn't include enpassants
    let white_pawns = position
        .bitboards
        .get_board_by_piece(Player::White, Piece::Pawn);
    let mut white_attacking_mask = generator.pawn_attack_moves_mask
        [Player::White.opponent() as usize][to as usize]
        & white_pawns;
    let black_pawns = position
        .bitboards
        .get_board_by_piece(Player::Black, Piece::Pawn);
    let mut black_attacking_mask = generator.pawn_attack_moves_mask
        [Player::Black.opponent() as usize][to as usize]
        & black_pawns;

    while white_attacking_mask != 0 {
        let from = white_attacking_mask.pop_mut();
        attackers.push((Piece::Pawn, ray_direction(from, to), from))
    }
    while black_attacking_mask != 0 {
        let from = black_attacking_mask.pop_mut();
        attackers.push((Piece::Pawn, ray_direction(from, to), from))
    }

    // king attack
    let king_move_mask = generator.king_moves_masks[to as usize];
    let mut attacking_king = king_move_mask
        & (position
            .bitboards
            .get_board_by_piece(Player::White, Piece::King)
            | position
                .bitboards
                .get_board_by_piece(Player::Black, Piece::King));
    while attacking_king != 0 {
        let from = attacking_king.pop_mut();
        attackers.push((Piece::Pawn, ray_direction(from, to), from))
    }

    attackers
}
