use crate::state::bitboards::BitBoard;

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
