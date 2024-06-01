use crate::{
    boards::BitBoard,
    masks::{
        create_bishop_potential_moves_mask_on_the_fly, create_rook_potential_moves_mask_on_the_fly,
    },
};

extern crate xorshift;
use xorshift::{Rng, Xoroshiro128};

// each bit in the bit set represents whether the corresponding bit in the mask should be present
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

#[derive(Default, Debug, Clone, Copy)]
pub struct Magic {
    magic_num: u64,
    potential_blockers_mask: u64,
    shift: u64,
    offset: u64,
}

#[inline(always)]
pub fn hash_with_magic(magic: Magic, blockers: &u64) -> usize {
    let blockers_on_path = blockers & magic.potential_blockers_mask;
    let hash = blockers_on_path.wrapping_mul(magic.magic_num);
    let index = ((hash >> magic.shift) + magic.offset) as usize;
    return index;
}

pub fn find_rook_magic_numbers(
    rng: &mut Xoroshiro128,
    relevant_bits: &[i8; 64],
    potential_blockers_mask: &[u64; 64],
) -> ([Magic; 64], Vec<u64>) {
    let mut rook_magic_numbers: [Magic; 64] = [Magic::default(); 64];
    let mut rook_magic_moves: Vec<u64> = Vec::new();

    let mut offset = 0;
    for pos in 0..64 {
        let (magic_num, mut moves) = find_magic_number(
            rng,
            pos.try_into().unwrap(),
            relevant_bits[pos as usize],
            potential_blockers_mask[pos as usize],
            &create_rook_potential_moves_mask_on_the_fly,
        )
        .unwrap();
        rook_magic_numbers[pos as usize] = Magic {
            magic_num,
            potential_blockers_mask: potential_blockers_mask[pos as usize],
            shift: 64 - relevant_bits[pos as usize] as u64,
            offset,
        };
        rook_magic_moves.append(&mut moves);

        offset += 1 << relevant_bits[pos as usize];
    }

    rook_magic_moves.shrink_to_fit();

    return (rook_magic_numbers, rook_magic_moves);
}

pub fn find_bishop_magic_numbers(
    rng: &mut Xoroshiro128,
    relevant_bits: &[i8; 64],
    potential_blockers_mask: &[u64; 64],
) -> ([Magic; 64], Vec<u64>) {
    let mut bishop_magic_numbers: [Magic; 64] = [Magic::default(); 64];
    let mut bishop_magic_moves: Vec<u64> = Vec::new();
    let mut offset = 0;
    for pos in 0..64 {
        let (magic_num, mut moves) = find_magic_number(
            rng,
            pos.try_into().unwrap(),
            relevant_bits[pos as usize],
            potential_blockers_mask[pos as usize],
            &create_bishop_potential_moves_mask_on_the_fly,
        )
        .unwrap();
        bishop_magic_numbers[pos as usize] = Magic {
            magic_num,
            potential_blockers_mask: potential_blockers_mask[pos as usize],
            shift: 64 - relevant_bits[pos as usize] as u64,
            offset,
        };
        bishop_magic_moves.append(&mut moves);

        offset += 1 << relevant_bits[pos as usize];
    }
    bishop_magic_moves.shrink_to_fit();

    return (bishop_magic_numbers, bishop_magic_moves);
}

pub fn generate_magic_number(rng: &mut Xoroshiro128) -> u64 {
    // we want a number with few bits, so we "&" random numbers
    // to make finding a number satifying our criteria faster
    return rng.next_u64() & rng.next_u64() & rng.next_u64();
}
#[derive(Debug)]
pub struct FailedToFindMagicNumberError;

fn find_magic_number(
    rng: &mut Xoroshiro128,
    pos: i8,
    relevant_bits: i8,
    potential_blockers_mask: u64,
    generate_potential_moves_on_the_fly: &dyn Fn(i8, u64) -> u64,
) -> Result<(u64, Vec<u64>), FailedToFindMagicNumberError> {
    // essentially 2 raised to the number of relevant_bits, so the number of possible subsets
    let num_subsets = 1 << relevant_bits;

    let mut occupancies = vec![0_u64; num_subsets];
    let mut moves = vec![0_u64; num_subsets];

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
    let max_attempt = 10000000;
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
                // temp magic
                Magic {
                    magic_num,
                    potential_blockers_mask,
                    offset: 0,
                    shift: 64 - relevant_bits as u64,
                },
                &occupancies[bit_set as usize],
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
