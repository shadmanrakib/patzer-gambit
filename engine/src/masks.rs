pub const RANK_1_MASK: u64 = 0xff;
pub const RANK_2_MASK: u64 = RANK_1_MASK << 8;
pub const RANK_7_MASK: u64 = RANK_1_MASK << 48;
pub const RANK_8_MASK: u64 = RANK_1_MASK << 56;

pub const FILE_A_MASK: u64 = 0x101010101010101;
pub const FILE_H_MASK: u64 = FILE_A_MASK << 7;

const fn create_inverted_square_masks() -> [u64; 64] {
    let mut bb_squares: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < 64 {
        bb_squares[i] = !(1 << i);
        i += 1;
    }

    bb_squares
}

pub static INVERTED_SQUARE_MASKS: [u64; 64] = create_inverted_square_masks();