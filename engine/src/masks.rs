pub const RANK_1_MASK: u64 = 0xffu64;
pub const RANK_2_MASK: u64 = RANK_1_MASK << 8;
// pub const RANK_3_MASK: u64 = RANK_1_MASK << 16;
// pub const RANK_4_MASK: u64 = RANK_1_MASK << 24;
// pub const RANK_5_MASK: u64 = RANK_1_MASK << 32;
// pub const RANK_6_MASK: u64 = RANK_1_MASK << 40;
pub const RANK_7_MASK: u64 = RANK_1_MASK << 48;
pub const RANK_8_MASK: u64 = RANK_1_MASK << 56;

pub const FILE_A_MASK: u64 = 0x101010101010101u64;
// pub const FILE_B_MASK: u64 = FILE_A_MASK << 1;
// pub const FILE_C_MASK: u64 = FILE_A_MASK << 2;
// pub const FILE_D_MASK: u64 = FILE_A_MASK << 3;
// pub const FILE_E_MASK: u64 = FILE_A_MASK << 4;
// pub const FILE_F_MASK: u64 = FILE_A_MASK << 5;
// pub const FILE_G_MASK: u64 = FILE_A_MASK << 6;
pub const FILE_H_MASK: u64 = FILE_A_MASK << 7;

const fn create_square_masks() -> [u64; 64] {
    let mut bb_squares: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < 64 {
        bb_squares[i] = 1 << i;
        i += 1;
    }

    bb_squares
}

const fn create_inverted_square_masks() -> [u64; 64] {
    let mut bb_squares: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < 64 {
        bb_squares[i] = !(1 << i);
        i += 1;
    }

    bb_squares
}

pub static SQUARE_MASKS: [u64; 64] = create_square_masks();
pub static INVERTED_SQUARE_MASKS: [u64; 64] = create_inverted_square_masks();