use super::{
    bishop::calculate_bishop_bit_counts, king::create_king_moves_masks,
    knight::create_knight_moves_masks, rook::calculate_rook_bit_counts,
};

#[derive(Debug)]
pub struct PrecalculatedCache {
    pub knight_moves_masks: [u64; 64],
    pub king_moves_masks: [u64; 64],
    pub bishop_bit_counts: [i8; 64],
    pub rook_bit_counts: [i8; 64],
}

impl PrecalculatedCache {
    pub fn create() -> Self {
        PrecalculatedCache {
            knight_moves_masks: create_knight_moves_masks(),
            king_moves_masks: create_king_moves_masks(),
            rook_bit_counts: calculate_rook_bit_counts(),
            bishop_bit_counts: calculate_bishop_bit_counts(),
        }
    }
}
