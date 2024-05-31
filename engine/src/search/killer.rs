use std::fmt::Debug;

use crate::{
    settings::{MAX_KILLER_MOVES, MAX_PLY},
    moves::data::MoveItem,
    state::{pieces::Piece, square::Square},
};

#[derive(Clone, Copy)]
pub struct SimpleMove {
    pub to: i8,
    pub from: i8,
    pub promotion: Piece,
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

impl Debug for SimpleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.to_string()))
    }
}

impl SimpleMove {
    pub fn to_string(&self) -> String {
        let from_square = Square::from(self.from).stringify();
        let to_square = Square::from(self.to).stringify();
        let promotion = {
            if self.promotion != Piece::Empty {
                self.promotion.to_string().to_lowercase()
            } else {
                "".to_string()
            }
        };
        format!("{}{}{}", from_square, to_square, promotion)
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
