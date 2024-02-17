use crate::state::pieces::Piece;

// rough draft as i figure out how i want to structure things
// we will use more than needed initially
#[derive(Debug, Clone)]
pub struct MoveItem {
    pub from_pos: i8,
    pub to_pos: i8,
    pub piece: Piece,
    pub promotion_piece: Piece,
    pub captured_piece: Piece,
    // flags
    pub promoting: bool,
    pub capturing: bool,
    pub double: bool,
    pub enpassant: bool,
    pub castling: bool,
    // for move ordering
    pub score: f32,
}