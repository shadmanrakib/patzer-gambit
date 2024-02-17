use std::{clone, ops::BitAnd};

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

#[derive(Default, Debug, Clone)]
pub struct BitBoards {
    occupied: u64,
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
}

impl BitBoards {
    pub fn get_board_by_piece(&self, piece: Piece) -> &u64 {
        match piece {
            Piece::Pawn(Player::White) => &self.white_pawns,
            Piece::Knight(Player::White) => &self.white_knights,
            Piece::Bishop(Player::White) => &self.white_bishops,
            Piece::Rook(Player::White) => &self.white_rooks,
            Piece::Queen(Player::White) => &self.white_queens,
            Piece::King(Player::White) => &self.white_king,
            Piece::Pawn(Player::Black) => &self.black_pawns,
            Piece::Knight(Player::Black) => &self.black_knights,
            Piece::Bishop(Player::Black) => &self.black_bishops,
            Piece::Rook(Player::Black) => &self.black_rooks,
            Piece::Queen(Player::Black) => &self.black_queens,
            Piece::King(Player::Black) => &self.black_king,
            Piece::Empty => panic!("Cannot pass empty"),
        }
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

        self.white_pawns.clear();
        self.white_knights.clear();
        self.white_bishops.clear();
        self.white_queens.clear();
        self.white_knights.clear();
        self.white_king.clear();

        self.black_pawns.clear();
        self.black_knights.clear();
        self.black_bishops.clear();
        self.black_queens.clear();
        self.black_knights.clear();
        self.black_king.clear();
    }

    pub fn default_board(&mut self) {
        // set pawns
        for pos in 8..16 {
            self.set_piece_by_bit_pos(Piece::Pawn(Player::White), pos);
        }
        for pos in 48..56 {
            self.set_piece_by_bit_pos(Piece::Pawn(Player::Black), pos);
        }

        // set rooks
        self.set_piece_by_bit_pos(Piece::Rook(Player::White), 0);
        self.set_piece_by_bit_pos(Piece::Rook(Player::White), 7);
        self.set_piece_by_bit_pos(Piece::Rook(Player::Black), 56);
        self.set_piece_by_bit_pos(Piece::Rook(Player::Black), 63);

        // set knights
        self.set_piece_by_bit_pos(Piece::Knight(Player::White), 1);
        self.set_piece_by_bit_pos(Piece::Knight(Player::White), 6);
        self.set_piece_by_bit_pos(Piece::Knight(Player::Black), 57);
        self.set_piece_by_bit_pos(Piece::Knight(Player::Black), 62);

        // set bishops
        self.set_piece_by_bit_pos(Piece::Bishop(Player::White), 2);
        self.set_piece_by_bit_pos(Piece::Bishop(Player::White), 5);
        self.set_piece_by_bit_pos(Piece::Bishop(Player::Black), 58);
        self.set_piece_by_bit_pos(Piece::Bishop(Player::Black), 61);

        // set queens
        self.set_piece_by_bit_pos(Piece::Queen(Player::White), 3);
        self.set_piece_by_bit_pos(Piece::Queen(Player::Black), 59);

        // set king
        self.set_piece_by_bit_pos(Piece::King(Player::White), 4);
        self.set_piece_by_bit_pos(Piece::King(Player::Black), 60);
    }

    pub fn get_piece_by_bit_pos(&self, pos: i8) -> Piece {
        // check if empty piece
        if self.occupied.get(pos) {
            let boards_and_piece = [
                // white
                (&self.white_pawns, Piece::Pawn(Player::White)),
                (&self.white_bishops, Piece::Bishop(Player::White)),
                (&self.white_knights, Piece::Knight(Player::White)),
                (&self.white_rooks, Piece::Rook(Player::White)),
                (&self.white_queens, Piece::Queen(Player::White)),
                (&self.white_king, Piece::King(Player::White)),
                // black
                (&self.black_pawns, Piece::Pawn(Player::Black)),
                (&self.black_bishops, Piece::Bishop(Player::Black)),
                (&self.black_knights, Piece::Knight(Player::Black)),
                (&self.black_rooks, Piece::Rook(Player::Black)),
                (&self.black_queens, Piece::Queen(Player::Black)),
                (&self.black_king, Piece::King(Player::Black)),
            ];

            for (board, piece) in boards_and_piece.into_iter() {
                if board.get(pos) {
                    return piece;
                }
            }
        }

        return Piece::Empty;
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

        let boards_and_piece = [
            // white
            (&mut self.white_pawns, Piece::Pawn(Player::White)),
            (&mut self.white_bishops, Piece::Bishop(Player::White)),
            (&mut self.white_knights, Piece::Knight(Player::White)),
            (&mut self.white_rooks, Piece::Rook(Player::White)),
            (&mut self.white_queens, Piece::Queen(Player::White)),
            (&mut self.white_king, Piece::King(Player::White)),
            // black
            (&mut self.black_pawns, Piece::Pawn(Player::Black)),
            (&mut self.black_bishops, Piece::Bishop(Player::Black)),
            (&mut self.black_knights, Piece::Knight(Player::Black)),
            (&mut self.black_rooks, Piece::Rook(Player::Black)),
            (&mut self.black_queens, Piece::Queen(Player::Black)),
            (&mut self.black_king, Piece::King(Player::Black)),
        ];

        for (board, p) in boards_and_piece.into_iter() {
            if p == piece {
                board.set(pos);
                break;
            }
        }

        self.occupied.set(pos);

        return true;
    }

    pub fn set_or_replace_piece_by_bit_pos(&mut self, piece: Piece, pos: i8) -> Piece {
        let mut removed = Piece::Empty;
        let boards_and_piece = [
            // white
            (&mut self.white_pawns, Piece::Pawn(Player::White)),
            (&mut self.white_bishops, Piece::Bishop(Player::White)),
            (&mut self.white_knights, Piece::Knight(Player::White)),
            (&mut self.white_rooks, Piece::Rook(Player::White)),
            (&mut self.white_queens, Piece::Queen(Player::White)),
            (&mut self.white_king, Piece::King(Player::White)),
            // black
            (&mut self.black_pawns, Piece::Pawn(Player::Black)),
            (&mut self.black_bishops, Piece::Bishop(Player::Black)),
            (&mut self.black_knights, Piece::Knight(Player::Black)),
            (&mut self.black_rooks, Piece::Rook(Player::Black)),
            (&mut self.black_queens, Piece::Queen(Player::Black)),
            (&mut self.black_king, Piece::King(Player::Black)),
        ];

        for (board, p) in boards_and_piece.into_iter() {
            if p == piece {
                board.set(pos);
            } else if board.get(pos) {
                board.unset(pos);
                removed = p;
            }
        }

        self.occupied.set(pos);
        return removed;
    }

    pub fn unset_by_bit_pos(&mut self, pos: i8) -> Piece {
        let mut removed = Piece::Empty;
        let boards_and_piece = [
            // white
            (&mut self.white_pawns, Piece::Pawn(Player::White)),
            (&mut self.white_bishops, Piece::Bishop(Player::White)),
            (&mut self.white_knights, Piece::Knight(Player::White)),
            (&mut self.white_rooks, Piece::Rook(Player::White)),
            (&mut self.white_queens, Piece::Queen(Player::White)),
            (&mut self.white_king, Piece::King(Player::White)),
            // black
            (&mut self.black_pawns, Piece::Pawn(Player::Black)),
            (&mut self.black_bishops, Piece::Bishop(Player::Black)),
            (&mut self.black_knights, Piece::Knight(Player::Black)),
            (&mut self.black_rooks, Piece::Rook(Player::Black)),
            (&mut self.black_queens, Piece::Queen(Player::Black)),
            (&mut self.black_king, Piece::King(Player::Black)),
        ];

        for (board, p) in boards_and_piece.into_iter() {
            if board.get(pos) {
                board.unset(pos);
                removed = p;
                break;
            }
        }

        self.occupied.unset(pos);
        return removed;
    }

    pub fn set_piece(&mut self, piece: Piece, rank: i8, file: i8) -> bool {
        let pos: i8 = rank * 8 + file;
        return self.set_piece_by_bit_pos(piece, pos);
    }
}
