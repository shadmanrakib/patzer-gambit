use super::{
    bishop::create_bishop_potential_moves_mask_on_the_fly,
    rook::create_rook_potential_moves_mask_on_the_fly,
};
use crate::state::bitboards::BitBoard;

extern crate xorshift;
use xorshift::{Rng, Xoroshiro128};

// each bit in the bit set represents whether the corresponding bit in the mask should be present
#[inline]
pub fn get_subset_of_mask_by_bit_set(mask: u64, mask_bit_count: i8, bit_set: u64) -> u64 {
    let mut mask_mut = mask.clone();
    let mut subset_mask = 0;

    for i in 0..mask_bit_count {
        let pos = mask_mut.pop_mut();
        if bit_set >> i & 1 == 1 {
            subset_mask.set(pos);
        }
    }

    return subset_mask;
}

#[inline]
pub fn hash_with_magic(
    potential_blockers_mask: u64,
    blockers: u64,
    magic_num: u64,
    bit_count: i8,
) -> usize {
    let blockers_on_path = blockers & potential_blockers_mask;
    let hash = blockers_on_path.wrapping_mul(magic_num);
    let index = (hash >> (64 - bit_count)) as usize;
    return index;
}

const ARRAY_REPEAT_VALUE: Vec<u64> = Vec::new();

pub fn find_rook_magic_numbers(
    rng: &mut Xoroshiro128,
    relevant_bits: &[i8; 64],
    potential_blockers_mask: &[u64; 64],
) -> ([u64; 64], [Vec<u64>; 64]) {
    let mut rook_magic_numbers: [u64; 64] = [0; 64];
    let mut rook_magic_moves = [ARRAY_REPEAT_VALUE; 64];

    for pos in 0..64 {
        let (magic_num, moves) = find_magic_number(
            rng,
            pos.try_into().unwrap(),
            relevant_bits[pos as usize],
            potential_blockers_mask,
            &create_rook_potential_moves_mask_on_the_fly,
        )
        .unwrap();
        rook_magic_numbers[pos as usize] = magic_num;
        rook_magic_moves[pos as usize] = moves;
    }
    return (rook_magic_numbers, rook_magic_moves);
}

pub fn find_bishop_magic_numbers(
    rng: &mut Xoroshiro128,
    relevant_bits: &[i8; 64],
    potential_blockers_mask: &[u64; 64],
) -> ([u64; 64], [Vec<u64>; 64]) {
    let mut magic_numbers: [u64; 64] = [0; 64];
    let mut magic_moves = [ARRAY_REPEAT_VALUE; 64];

    for pos in 0..64 {
        let (magic_num, moves) = find_magic_number(
            rng,
            pos.try_into().unwrap(),
            relevant_bits[pos as usize],
            potential_blockers_mask,
            &create_bishop_potential_moves_mask_on_the_fly,
        )
        .unwrap();
        magic_numbers[pos as usize] = magic_num;
        magic_moves[pos as usize] = moves;
    }
    return (magic_numbers, magic_moves);
}

#[inline]
pub fn generate_magic_number(rng: &mut Xoroshiro128) -> u64 {
    return rng.next_u64() & rng.next_u64() & rng.next_u64();
}
#[derive(Debug)]
pub struct FailedToFindMagicNumberError;

#[inline]
fn find_magic_number(
    rng: &mut Xoroshiro128,
    pos: i8,
    relevant_bits: i8,
    potential_blockers_mask: &[u64; 64],
    generate_potential_moves_on_the_fly: &dyn Fn(i8, u64) -> u64,
) -> Result<(u64, Vec<u64>), FailedToFindMagicNumberError> {
    // essentially 2 raised to the number of relevant_bits, so the number of possible subsets
    let num_subsets = 1 << relevant_bits;

    let mut occupancies = vec![0_u64; num_subsets];
    let mut moves = vec![0_u64; num_subsets];

    let potential_blockers_mask = potential_blockers_mask[pos as usize];

    // we can use the number as a bitmap for which parts of the mask to include
    // using the properties of 2 complements, we can go through all subsets since
    // the the bits of the numbers from 0 to num_subset-1 can be used to represent the combo of
    // bits included.
    for bit_set in 0..num_subsets {
        occupancies[bit_set as usize] =
            get_subset_of_mask_by_bit_set(potential_blockers_mask, relevant_bits, bit_set as u64);
        moves[bit_set as usize] = generate_potential_moves_on_the_fly(pos, occupancies[bit_set]);
    }

    // try finding a magic number for this square
    let max_attempt = 1000000;
    for _ in 0..max_attempt {
        let magic_num: u64 = generate_magic_number(rng);
        let mut used_moves = vec![0_u64; num_subsets];
        // // Skip inappropriate magic numbers
        if (potential_blockers_mask.wrapping_mul(magic_num) & 0xFFF0_0000_0000_0000).count_ones()
            < 6
        {
            continue;
        }

        let mut fail = false;

        for bit_set in 0..num_subsets {
            let magic_index = hash_with_magic(
                potential_blockers_mask,
                occupancies[bit_set as usize],
                magic_num,
                relevant_bits,
            );
            if used_moves[magic_index as usize] == 0 {
                used_moves[magic_index as usize] = moves[bit_set as usize];
            } else if used_moves[magic_index] != moves[bit_set as usize] {
                fail = true;
                break;
            }
        }

        if !fail {
            return Ok((magic_num, used_moves));
        }
    }

    return Err(FailedToFindMagicNumberError);
}
