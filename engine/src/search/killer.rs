use crate::{
    constants::search::{MAX_KILLER_MOVES, MAX_PLY},
    moves::move_data::MoveItem,
    state::pieces::Piece,
};

#[derive(Clone, Copy, Debug)]
pub struct SimpleMove {
    to: i8,
    from: i8,
    promotion: Piece,
}

pub type KillerMoves = [[SimpleMove; MAX_KILLER_MOVES]; MAX_PLY as usize];

pub const NULL_MOVE: SimpleMove = SimpleMove {
    from: -1,
    to: -1,
    promotion: Piece::Empty,
};

pub fn init_killer_moves() -> KillerMoves {
    [[NULL_MOVE; MAX_KILLER_MOVES]; MAX_PLY as usize]
}

#[inline(always)]
pub fn is_similar(simple_move: &SimpleMove, move_item: &MoveItem) -> bool {
    simple_move.from == move_item.from_pos
        && simple_move.to == move_item.to_pos
        && simple_move.promotion == move_item.promotion_piece
}

impl Into<SimpleMove> for &MoveItem {
    fn into(self) -> SimpleMove {
        SimpleMove {
            from: self.from_pos,
            to: self.to_pos,
            promotion: self.promotion_piece,
        }
    }
}

#[inline(always)]
pub fn store_killer_move(killers: &mut KillerMoves, current_move: &MoveItem, ply: usize) {
    let first_killer = killers[ply as usize][0];

    // only place if current move isn't killer (to prevent it being filled with one move)
    if !is_similar(&first_killer, current_move) {
        // Shift all the moves one index upward...
        for i in (1..MAX_KILLER_MOVES).rev() {
            killers[ply][i] = killers[ply][i-1];
        }

        // and add the new killer move in the first spot.
        killers[ply as usize][0] = current_move.into();
    }
}
