use crate::{
    mv::{Move, SimpleMove},
    pieces::Piece,
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
