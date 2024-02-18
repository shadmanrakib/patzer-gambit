use std::{clone, ops::BitAnd};

use enum_map::EnumMap;

use super::{game, pieces::Piece, player::Player};

// #[derive(Default,Debug,Clone)]
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
        (self >> index) & 1 != 0
    }

    fn unset(&mut self, index: i8) {
        *self &= !(1 << index);
    }

    fn clear(&mut self) {
        *self = 0;
    }

    fn pop(&self) -> (u64, i8) {
        let trailing = self.trailing_zeros();
        return (self & !(1 << trailing), trailing.try_into().unwrap());
    }

    fn pop_mut(&mut self) -> i8 {
        let trailing: i8 = self.trailing_zeros().try_into().unwrap();
        if trailing >= 64 {
            println!("{self} {:?}", trailing);
        }
        *self &= !(1 << trailing);
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

#[derive(Debug, Clone)]
pub struct Boards {
    pub pos_to_piece: [Piece; 64],
    pub occupied: u64,
    pub boards: EnumMap<Piece, u64>,
}

impl Default for Boards {
    fn default() -> Self {
        Self {
            pos_to_piece: [Piece::Empty; 64],
            boards: Default::default(),
            occupied: Default::default(),
        }
    }
}

impl Boards {
    pub fn get_board_by_piece(&self, piece: Piece) -> &u64 {
        return &self.boards[piece];
    }
    pub fn get_occupied(&self) -> &u64 {
        return &self.occupied;
    }
    pub fn get_occupied_by_player(&self, player: Player) -> u64 {
        let pawns = self.get_board_by_piece(Piece::Pawn(player));
        let knights = self.get_board_by_piece(Piece::Knight(player));
        let bishops = self.get_board_by_piece(Piece::Bishop(player));
        let rooks = self.get_board_by_piece(Piece::Rook(player));
        let queens = self.get_board_by_piece(Piece::Queen(player));
        let king = self.get_board_by_piece(Piece::King(player));

        return pawns | knights | bishops | rooks | queens | king;
    }
    pub fn clear_board(&mut self) {
        self.occupied.clear();

        let pieces = [
            Piece::Pawn(Player::White),
            Piece::Knight(Player::White),
            Piece::Bishop(Player::White),
            Piece::Rook(Player::White),
            Piece::Queen(Player::White),
            Piece::King(Player::White),

            Piece::Pawn(Player::Black),
            Piece::Knight(Player::Black),
            Piece::Bishop(Player::Black),
            Piece::Rook(Player::Black),
            Piece::Queen(Player::Black),
            Piece::King(Player::Black),
        ];

        for piece in pieces {
            self.boards[piece].clear();
        }
    }

    pub fn get_piece_by_bit_pos(&self, pos: i8) -> Piece {
        return self.pos_to_piece[pos as usize];
    }

    pub fn get_piece(&self, rank: i8, file: i8) -> Piece {
        let pos: i8 = rank * 8 + file;
        return self.get_piece_by_bit_pos(pos);
    }

    pub fn set_piece_by_bit_pos(&mut self, piece: Piece, pos: i8) -> bool {
        // if space occupied, return success false
        if self.occupied.get(pos) {
            return false;
        }
        
        self.boards[piece].set(pos);
        self.pos_to_piece[pos as usize] = piece;
        self.occupied.set(pos);

        return true;
    }

    pub fn set_or_replace_piece_by_bit_pos(&mut self, piece: Piece, pos: i8) -> Piece {
        let removed = self.pos_to_piece[pos as usize];
        self.boards[removed].unset(pos);
        self.boards[piece].set(pos);
        self.pos_to_piece[pos as usize] = piece;
        self.occupied.set(pos);

        return removed;
    }

    pub fn unset_by_bit_pos(&mut self, pos: i8) -> Piece {
        let removed = self.pos_to_piece[pos as usize];
        self.boards[removed].unset(pos);
        self.pos_to_piece[pos as usize] = Piece::Empty;
        self.occupied.unset(pos);
        return removed;
    }

    pub fn set_piece(&mut self, piece: Piece, rank: i8, file: i8) -> bool {
        let pos: i8 = rank * 8 + file;
        return self.set_piece_by_bit_pos(piece, pos);
    }
}
