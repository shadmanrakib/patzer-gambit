use crate::state::square::Square;
use crate::state::bitboards::BitBoard;

pub fn create_bishop_potential_blockers_mask(pos: i8) -> u64 {
    let mut bitboard: u64 = 0;

    let Square {rank, file} = Square::from(pos);

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
            bitboard.set((r * 8 + f).try_into().unwrap());
            r += dr;
            f += df;
        }
    }

    return bitboard;
}

pub fn create_bishop_potential_moves_mask_on_the_fly(pos: i8, occupied: u64) -> u64 {
    let mut bitboard: u64 = 0;

    let Square {rank, file} = Square::from(pos);

    let directions: [(i8, i8, i8, i8, i8, i8); 4] = [
        (1, 1, 0, 7, 0, 7),
        (1, -1, 0, 7, 0, 7),
        (-1, 1, 0, 7, 0, 7),
        (-1, -1, 0, 7, 0, 7),
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

pub fn calculate_bishop_bit_counts() -> [i8; 64] {
    let mut bit_counts: [i8; 64] = [0; 64];

    for pos in 0..64 {
        bit_counts[pos] = create_bishop_potential_blockers_mask(pos.try_into().unwrap()).count_ones().try_into().unwrap();
    }

    return bit_counts;
}