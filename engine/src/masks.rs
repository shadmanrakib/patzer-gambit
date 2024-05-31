use crate::{boards::BitBoard, player::Player, square::Square};

pub const RANK_1_MASK: u64 = 0xff;
pub const RANK_2_MASK: u64 = RANK_1_MASK << 8;
pub const RANK_7_MASK: u64 = RANK_1_MASK << 48;
pub const RANK_8_MASK: u64 = RANK_1_MASK << 56;

pub const FILE_A_MASK: u64 = 0x101010101010101;
pub const FILE_H_MASK: u64 = FILE_A_MASK << 7;

const NOT_FILE_A_OR_H_MASK: u64 = !(FILE_A_MASK | FILE_H_MASK);
const NOT_RANK_1_OR_8_MASK: u64 = !(RANK_1_MASK | RANK_8_MASK);

const fn create_inverted_square_masks() -> [u64; 64] {
    let mut bb_squares: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < 64 {
        bb_squares[i] = !(1 << i);
        i += 1;
    }

    bb_squares
}

pub const INVERTED_SQUARE_MASKS: [u64; 64] = create_inverted_square_masks();

/**
 * MOVE GENERATION
 */

pub fn create_bishop_potential_blockers_mask(pos: i8) -> u64 {
    let mut bitboard: u64 = 0;

    let Square { rank, file } = Square::from(pos);

    let directions: [(i8, i8, i8, i8, i8, i8); 4] = [
        (1, 1, 0, 6, 0, 6),
        (1, -1, 0, 6, 1, 7),
        (-1, 1, 1, 7, 0, 6),
        (-1, -1, 1, 7, 1, 7),
    ];

    for (dr, df, min_r, max_r, min_f, max_f) in directions {
        let mut r: i8 = rank + dr;
        let mut f: i8 = file + df;
        while min_r <= r && r <= max_r && min_f <= f && f <= max_f {
            bitboard.set(r * 8 + f);
            r += dr;
            f += df;
        }
    }

    return bitboard;
}

pub fn create_bishop_potential_moves_mask_on_the_fly(pos: i8, occupied: u64) -> u64 {
    let mut bitboard: u64 = 0;

    let Square { rank, file } = Square::from(pos);

    let directions: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for (dr, df) in directions {
        let mut r: i8 = rank + dr;
        let mut f: i8 = file + df;
        while 0 <= r && r <= 7 && 0 <= f && f <= 7 {
            bitboard.set(r * 8 + f);
            if occupied.get(r * 8 + f) {
                break;
            }
            r += dr;
            f += df;
        }
    }

    return bitboard;
}

pub fn create_king_moves_masks() -> [u64; 64] {
    let mut masks: [u64; 64] = [0; 64];
    for pos in 0..64 {
        let deltas = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
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

// #[inline(always)]
pub fn create_rook_potential_blockers_mask(pos: i8) -> u64 {
    let Square { rank, file } = Square::from(pos);

    let rank_mask = (RANK_1_MASK << (rank * 8)) & NOT_FILE_A_OR_H_MASK;
    let file_mask = (FILE_A_MASK << file) & NOT_RANK_1_OR_8_MASK;
    let not_square = INVERTED_SQUARE_MASKS[pos as usize];
    let mask: u64 = (rank_mask | file_mask) & not_square;

    return mask;
}

pub fn create_rook_potential_moves_mask_on_the_fly(pos: i8, occupied: u64) -> u64 {
    let mut bitboard: u64 = 0;

    let Square { rank, file } = Square::from(pos);

    let directions: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (dr, df) in directions {
        let mut r: i8 = rank + dr;
        let mut f: i8 = file + df;
        while 0 <= r && r <= 7 && 0 <= f && f <= 7 {
            bitboard.set(r * 8 + f);
            if occupied.get(r * 8 + f) {
                break;
            }
            r += dr;
            f += df;
        }
    }

    return bitboard;
}

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
