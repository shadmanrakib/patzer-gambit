use std::mem::MaybeUninit;

use crate::moves::data::MoveItem;

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