use crate::state::{boards::BitBoard, square::Square};

pub fn create_knight_moves_masks() -> [u64; 64] {
    let mut masks: [u64; 64] = [0; 64];
    for pos in 0..64 {
        let deltas = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (2, -1),
            (2, 1),
            (1, -2),
            (1, 2),
        ];
        let Square { rank, file } = Square::from(pos);
        for (d_rank, d_file) in deltas {
            let new_rank = rank + d_rank;
            let new_file = file + d_file;

            if 0 <= new_rank && new_rank <= 7 && 0 <= new_file && new_file <= 7 {
                masks[pos as usize].set(new_rank * 8 + new_file);
            }
        }
    }
    return masks;
}
