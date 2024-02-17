use crate::state::bitboards::BitBoard;

pub fn create_knight_moves_masks() -> [u64;64] {
    let mut masks: [u64; 64] = [0; 64];
    for pos in 0..64 {
        let deltas: [i8; 8] = [
            (-2) * 8 + (-1),
            (-2) * 8 + (1),
            (-1) * 8 + (-2),
            (-1) * 8 + (2),
            (2) * 8 + (-1),
            (2) * 8 + (1),
            (1) * 8 + (-2),
            (1) * 8 + (2),
        ];
        for delta in deltas {
            let cur_pos = pos + delta;
            if 0 <= cur_pos && cur_pos <= 63 {
                masks[pos as usize].set(cur_pos);
            }
        }
    }
    return masks;
}
