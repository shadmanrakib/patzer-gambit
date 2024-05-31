use crate::{boards::BitBoard, player::Player, square::Square};

pub fn create_pawn_attack_moves_masks() -> [[u64; 64]; 2] {
    let mut masks: [[u64; 64]; 2] = [[0; 64]; 2];
    let rank_dir = [1, -1];

    for player in [Player::White, Player::Black] {
        for pos in 0..64 {
            let file_deltas = [-1, 1];
            let Square { rank, file } = Square::from(pos);
            for d_file in file_deltas {
                let d_rank = rank_dir[player as usize];
                let new_rank = rank + d_rank;
                let new_file = file + d_file;

                if 0 <= new_rank && new_rank <= 7 && 0 <= new_file && new_file <= 7 {
                    masks[player as usize][pos as usize].set(new_rank * 8 + new_file);
                }
            }
        }
    }
    return masks;
}
