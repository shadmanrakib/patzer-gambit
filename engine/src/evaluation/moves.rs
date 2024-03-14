use crate::state::movelist::MoveList;

// MVV_VLA[attacker][victem]
pub const MVV_LVA_SCORE: [[i16; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // attacker None, victim None, P, N, B, R, Q, K
    [0, 960, 970, 980, 990, 1000, 10000], // attacker P, victim None, P, N, B, R, Q, K
    [0, 860, 870, 880, 890, 900, 10000], // attacker K, victim None, P, N, B, R, Q, K
    [0, 760, 770, 780, 790, 800, 10000], // attacker B, victim None, P, N, B, R, Q, K
    [0, 660, 670, 680, 690, 700, 10000], // attacker R, victim None, P, N, B, R, Q, K
    [0, 560, 570, 580, 590, 600, 10000], // attacker Q, victim None, P, N, B, R, Q, K
    [0, 70, 70, 70, 70, 70, 0], // attacker K, victim None, P, N, B, R, Q, K
];

pub fn score_moves(moveslist: &mut MoveList) {
    for i in 0..moveslist.len() {
        moveslist.moves[i].score = MVV_LVA_SCORE[moveslist.moves[i].piece as usize]
            [moveslist.moves[i].captured_piece as usize];
    }
}
