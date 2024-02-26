use std::time::SystemTime;

use xorshift::{Rand, SeedableRng, SplitMix64, Xoroshiro128};

use super::{
    bishop::calculate_bishop_bit_counts,
    king::create_king_moves_masks,
    knight::create_knight_moves_masks,
    magic_bitboards::{find_bishop_magic_numbers, find_rook_magic_numbers},
    pawn::create_pawn_attack_moves_masks,
    rook::calculate_rook_bit_counts,
};

use super::{
    bishop::create_bishop_potential_blockers_mask, rook::create_rook_potential_blockers_mask,
};

#[derive(Debug)]
pub struct PrecalculatedCache {
    pub pawn_attack_moves_mask: [[u64; 64]; 2],
    pub knight_moves_masks: [u64; 64],
    pub king_moves_masks: [u64; 64],
    pub bishop_potential_blockers_masks: [u64; 64],
    pub rook_potential_blockers_masks: [u64; 64],
    pub bishop_bit_counts: [i8; 64],
    pub rook_bit_counts: [i8; 64],
    pub rook_magics: [u64; 64],
    pub rook_magic_attack_tables: [Vec<u64>; 64],
    pub bishop_magics: [u64; 64],
    pub bishop_magic_attack_tables: [Vec<u64>; 64],
}

impl PrecalculatedCache {
    pub fn create() -> PrecalculatedCache {
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

        PrecalculatedCache {
            pawn_attack_moves_mask,
            knight_moves_masks,
            king_moves_masks,
            rook_bit_counts,
            bishop_bit_counts,
            rook_potential_blockers_masks,
            bishop_potential_blockers_masks,
            rook_magics,
            rook_magic_attack_tables,
            bishop_magics,
            bishop_magic_attack_tables,
        }
    }
}
