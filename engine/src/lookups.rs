use std::time::SystemTime;

use xorshift::{Rand, SeedableRng, SplitMix64, Xoroshiro128};

use crate::{
    magics::{find_bishop_magic_numbers, find_rook_magic_numbers, Magic},
    masks::{
        create_bishop_potential_blockers_mask, create_king_moves_masks, create_knight_moves_masks,
        create_pawn_attack_moves_masks, create_rook_potential_blockers_mask,
    },
};

pub fn calculate_bishop_bit_counts() -> [i8; 64] {
    let mut bit_counts: [i8; 64] = [0; 64];

    for pos in 0..64 {
        bit_counts[pos] = create_bishop_potential_blockers_mask(pos.try_into().unwrap())
            .count_ones()
            .try_into()
            .unwrap();
    }

    return bit_counts;
}

pub fn calculate_rook_bit_counts() -> [i8; 64] {
    let mut bit_counts: [i8; 64] = [0; 64];

    for pos in 0..64 {
        bit_counts[pos] = create_rook_potential_blockers_mask(pos.try_into().unwrap())
            .count_ones()
            .try_into()
            .unwrap();
    }

    return bit_counts;
}

#[derive(Debug)]
pub struct Lookups {
    pub pawn_attack_moves_mask: [[u64; 64]; 2],
    pub knight_moves_masks: [u64; 64],
    pub king_moves_masks: [u64; 64],
    pub rook_magics: [Magic; 64],
    pub rook_magic_attack_tables: Vec<u64>,
    pub bishop_magics: [Magic; 64],
    pub bishop_magic_attack_tables: Vec<u64>,
}

impl Lookups {
    pub fn create() -> Lookups {
        let pawn_attack_moves_mask = create_pawn_attack_moves_masks();

        let king_moves_masks = create_king_moves_masks();
        let knight_moves_masks = create_knight_moves_masks();

        let rook_bit_counts = calculate_rook_bit_counts();
        let bishop_bit_counts = calculate_bishop_bit_counts();

        let mut rook_potential_blockers_masks = [0u64; 64];
        let mut bishop_potential_blockers_masks = [0u64; 64];

        for pos in 0..64 {
            rook_potential_blockers_masks[pos] = create_rook_potential_blockers_mask(pos as i8);
            bishop_potential_blockers_masks[pos] = create_bishop_potential_blockers_mask(pos as i8);
        }

        let seed: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut sm: SplitMix64 = SeedableRng::from_seed(seed);
        let mut rng: Xoroshiro128 = Rand::rand(&mut sm);

        let (rook_magics, rook_magic_attack_tables) =
            find_rook_magic_numbers(&mut rng, &rook_bit_counts, &rook_potential_blockers_masks);
        let (bishop_magics, bishop_magic_attack_tables) = find_bishop_magic_numbers(
            &mut rng,
            &bishop_bit_counts,
            &bishop_potential_blockers_masks,
        );

        Lookups {
            pawn_attack_moves_mask,
            knight_moves_masks,
            king_moves_masks,
            rook_magics,
            rook_magic_attack_tables,
            bishop_magics,
            bishop_magic_attack_tables,
        }
    }
}
