pub const MAX_PLY: u8 = 144;
pub const MAX_MAIN_SEARCH_DEPTH: u8 = 16;
pub const MAX_KILLER_MOVES: usize = 2;
pub const FULL_DEPTH_MOVES: u16 = 4;
pub const REDUCTION_LIMIT: u8 = 10;
// pub const TRANSITION_TABLE_ADDRESSING_BITS: usize = 24;
pub const TRANSITION_TABLE_SIZE: usize =  31734589; // 9737333; // 180096947 // 31734589 // 72034903 // 174440041; // 326851121
pub const TRANSITION_TABLE_ADDRESSING_BITS: usize = 23;
