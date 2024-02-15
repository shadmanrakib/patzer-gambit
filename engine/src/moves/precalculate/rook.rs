use crate::constants::masks::{FILE_A_MASK, FILE_H_MASK, RANK_1_MASK, RANK_7_MASK, RANK_8_MASK};
use crate::state::square::Square;
use crate::state::bitboards::BitBoard;

static  NOT_FILE_A_OR_H_MASK: u64= !(FILE_A_MASK | FILE_H_MASK);
static NOT_RANK_1_OR_8_MASK: u64 = !(RANK_1_MASK | RANK_8_MASK);

#[inline(always)]
pub fn create_rook_potential_blockers_mask(pos: i8) -> u64 {
    // let mut bitboard: u64 = 0;

    // let Square {rank, file} = Square::from(pos);

    // let directions_and_restraints: [(i8, i8, i8, i8, i8, i8); 4] = [
    //     (1, 0, 0, 6, 0, 7),
    //     (-1, 0, 1, 7, 0, 7),
    //     (0, 1, 0, 7, 0, 6),
    //     (0, -1, 0, 7, 1, 7),
    // ];

    // for (dr, df, min_r, max_r, min_f, max_f) in directions_and_restraints {
    //     let mut r = rank + dr;
    //     let mut f = file + df;
    //     while min_r <= r && r <= max_r && min_f <= f && f <= max_f {
    //         bitboard.set(r * 8 + f);
    //         r += dr;
    //         f += df;
    //     }
    // }

    // return bitboard;

    let rank_mask = (RANK_1_MASK << (rank * 8)) & NOT_FILE_A_OR_H_MASK;
    let file_mask = (FILE_A_MASK << file) & NOT_RANK_1_OR_8_MASK;
    let not_square = !(1 << pos);
    let a: u64 = (rank_mask | file_mask) & not_square;

    if a != bitboard {
        a.print_board();
        bitboard.print_board();
        println!("DOESNT WORK");
    }
    // if rank != 0 {
    //     a &= !RANK_1_MASK;
    // }
    // if rank != 7 {
    //     a &= !RANK_8_MASK;
    // }
    // if file != 0 {
    //     a &= !FILE_A_MASK;
    // }
    // if file != 7 {
    //     a &= !FILE_H_MASK;
    // }

    return a;
}

pub fn create_rook_potential_moves_mask_on_the_fly(pos: i8, occupied: u64) -> u64 {
    let mut bitboard: u64 = 0;

    let Square {rank, file} = Square::from(pos);

    let directions: [(i8, i8, i8, i8, i8, i8); 4] = [
        (1, 0, 0, 7, 0, 7),
        (-1, 0, 0, 7, 0, 7),
        (0, 1, 0, 7, 0, 7),
        (0, -1, 0, 7, 0, 7),
    ];

    for (dr, df, min_r, max_r, min_f, max_f) in directions {
        let mut r: i8 = rank + dr;
        let mut f: i8 = file + df;
        while min_r <= r && r <= max_r && min_f <= f && f <= max_f {
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


pub fn calculate_rook_bit_counts() -> [i8; 64] {
    let mut bit_counts: [i8; 64] = [0; 64];

    for pos in 0..64 {
        bit_counts[pos] = create_rook_potential_blockers_mask(pos.try_into().unwrap()).count_ones().try_into().unwrap();
    }

    return bit_counts;
}
