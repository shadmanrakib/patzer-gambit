use crate::moves::data::MoveItem;
use crate::pieces::Piece;
use crate::square::Square;
use std::fmt::Debug;
use std::mem::MaybeUninit;

pub struct MoveList {
    pub moves: [MoveItem; 256],
    pub end: usize,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            // we use maybeuninit to initialize list without "zeroing" for performance.
            moves: unsafe {
                let m = MaybeUninit::uninit();
                m.assume_init()
            },
            end: 0,
        }
    }
    pub fn push(&mut self, move_item: MoveItem) {
        self.moves[self.end] = move_item;
        self.end += 1;
    }
    pub fn len(&self) -> usize {
        self.end
    }
    pub fn sort_move(&mut self, index: usize) {
        for i in (index + 1)..self.len() {
            if self.moves[i].score > self.moves[index].score {
                self.moves.swap(index as usize, i as usize);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct SimpleMove {
    pub to: i8,
    pub from: i8,
    pub promotion: Piece,
}

impl SimpleMove {
    pub const NULL_MOVE: SimpleMove = SimpleMove {
        from: 0,
        to: 0,
        promotion: Piece::Empty,
    };
    #[inline(always)]
    pub fn is_similar(&self, move_item: &MoveItem) -> bool {
        self.from == move_item.from_pos
            && self.to == move_item.to_pos
            && self.promotion == move_item.promotion_piece
    }
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
impl ToString for SimpleMove {
    fn to_string(&self) -> String {
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
