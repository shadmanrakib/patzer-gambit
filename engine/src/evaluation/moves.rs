use crate::{
    constants::search::{MAX_KILLER_MOVES, MAX_PLY},
    search::{
        cache::SearchCache,
        killer::{is_similar, SimpleMove},
    },
    state::{movelist::MoveList, player::Player},
};

pub const MVV_LVA_SCORE: [[i16; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // victim None, attacker K, Q, R, B, N, P, None
    [0, 150, 140, 130, 120, 110, 100], // victim P, attacker K, Q, R, B, N, P, None
    [0, 250, 240, 230, 220, 210, 200], // victim K, attacker K, Q, R, B, N, P, None
    [0, 350, 340, 330, 320, 310, 300], // victim B, attacker K, Q, R, B, N, P, None
    [0, 450, 440, 430, 420, 410, 400], // victim R, attacker K, Q, R, B, N, P, None
    [0, 550, 540, 530, 520, 510, 500], // victim Q, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0], // victim K, attacker K, Q, R, B, N, P, None
];

const MAX_SCORE: i16 = std::i16::MAX;
const MMV_LVA_OFFSET: i16 = std::i16::MAX - 1 - 10000;
const MIN_SCORE: i16 = std::i16::MIN;

#[allow(dead_code)]
pub const PROMOTION_POINTS: [i16; 7] = [
    0,  // Empty
    0,  // Pawn
    15, // Knight
    0,  // Bishop
    0,  // Rook
    20, // Queen
    0,  // King
];

pub fn score_captures(moveslist: &mut MoveList) {
    for i in 0..moveslist.len() {
        let move_item = &mut moveslist.moves[i];
        if move_item.capturing {
            move_item.score =
                MVV_LVA_SCORE[move_item.captured_piece as usize][move_item.piece as usize];
        } else {
            move_item.score = MIN_SCORE;
        }
    }
}

#[allow(dead_code)]
pub fn score_moves(
    moveslist: &mut MoveList,
    search_cache: &mut SearchCache,
    ply: usize,
    player: Player,
    // pv: &Vec<SimpleMove>,
    pv_table: &mut [[SimpleMove; MAX_PLY as usize]; MAX_PLY as usize],
    pv_size: &mut [usize; MAX_PLY as usize],
) {
    for i in 0..moveslist.len() {
        let move_item = &mut moveslist.moves[i];
        if pv_table[0][ply].from == move_item.from_pos
            && pv_table[0][ply].to == move_item.to_pos
            && pv_table[0][ply].promotion == move_item.promotion_piece
        {
            move_item.score = MAX_SCORE;
        } else if move_item.capturing {
            move_item.score = MVV_LVA_SCORE[move_item.captured_piece as usize]
                [move_item.piece as usize]
                + MMV_LVA_OFFSET;
        } else {
            // move_item.score += <i32>::try_into(
            //     (OPENING_PSQT_TABLES[move_item.piece as usize][move_item.to_pos as usize]
            //         - OPENING_PSQT_TABLES[move_item.piece as usize][move_item.from_pos as usize])
            //         / 4,
            // )
            // .unwrap_or(0);
            move_item.score = search_cache.history_moves[player as usize][move_item.piece as usize]
                [move_item.to_pos as usize];

            for i in 0..MAX_KILLER_MOVES {
                if is_similar(&search_cache.killer_moves[ply][i], move_item) {
                    move_item.score = MMV_LVA_OFFSET - 1000 - ((i * 10) as i16);
                    break;
                }
            }
        }
    }
}
