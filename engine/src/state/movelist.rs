use std::mem::MaybeUninit;

use crate::moves::data::MoveItem;

pub struct MoveList {
    pub moves: [MoveItem; 256],
    pub end: usize,
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            // we use maybeuninit because there is no point of providing
            // a "zero" value for this list. it will only slow us down.
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
        // let mut max = self.moves[index].score;
        // for i in (index + 1)..self.len() {
        //     if self.moves[i].score > max {
        //         max = self.moves[i].score;
        //         self.moves.swap(i, index);
        //     }
        // }

        for i in (index + 1)..self.len() {
            if self.moves[i].score > self.moves[index].score {
                self.moves.swap(index as usize, i as usize);
            }
        }
    }
}