use crate::masks::INVERTED_SQUARE_MASKS;

use super::{pieces::Piece, player::Player};

pub trait BitBoard {
    fn set(&mut self, index: i8);
    fn get(&self, index: i8) -> bool;
    fn unset(&mut self, index: i8);
    fn clear(&mut self);
    fn pop(&self) -> (Self, i8)
    where
        Self: Sized;
    fn pop_mut(&mut self) -> i8;
    fn print_board(&self);
}

impl BitBoard for u64 {
    fn set(&mut self, index: i8) {
        *self |= 1 << index;
    }
    fn get(&self, index: i8) -> bool {
        self & 1 << index != 0
    }
    fn unset(&mut self, index: i8) {
        *self &= INVERTED_SQUARE_MASKS[index as usize];
    }
    fn clear(&mut self) {
        *self = 0;
    }
    fn pop(&self) -> (u64, i8) {
        let trailing: i8 = self.trailing_zeros().try_into().unwrap();
        return (*self & (*self - 1), trailing);
    }
    fn pop_mut(&mut self) -> i8 {
        let trailing: i8 = self.trailing_zeros().try_into().unwrap();
        *self = *self & (*self - 1);
        return trailing;
    }
    fn print_board(&self) {
        for rank in (0..8).rev() {
            print!("{} | ", rank + 1);
            for file in 0..8 {
                let pos: i8 = 8 * rank + file;
                print!("{} ", (self >> pos) & 1);
            }
            println!();
        }
        println!("  -----------------");
        println!("    A B C D E F G H");
        println!("Hex: {:x}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boards {
    pub occupied: u64,
    pub bitboards: [[u64; 7]; 2],
    pub pos_to_player: [u64; 2],
    pub pos_to_piece: [Piece; 64],
}

impl Default for Boards {
    fn default() -> Self {
        Self {
            pos_to_piece: [Piece::Empty; 64],
            pos_to_player: [0; 2],
            bitboards: Default::default(),
            occupied: Default::default(),
        }
    }
}

impl Boards {
    pub fn get_board_by_piece(&self, player: Player, piece: Piece) -> &u64 {
        return &self.bitboards[player as usize][piece as usize];
    }
    #[inline(always)]
    pub fn place_piece(&mut self, player: Player, piece: Piece, pos: i8) {
        self.bitboards[player as usize][piece as usize].set(pos);
        self.pos_to_piece[pos as usize] = piece;
        self.pos_to_player[player as usize].set(pos);
        self.occupied.set(pos);
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, player: Player, pos: i8) -> Piece {
        let removed = self.pos_to_piece[pos as usize];
        self.pos_to_piece[pos as usize] = Piece::Empty;
        self.bitboards[player as usize][removed as usize].unset(pos);
        self.pos_to_player[player as usize].unset(pos);
        self.occupied.unset(pos);
        return removed;
    }
}
