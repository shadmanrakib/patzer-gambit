use crate::{
    constants::masks::INVERTED_SQUARE_MASKS,
    evaluation::psqt_tapered::{
        ENDGAME_PSQT_TABLES, OPENING_PSQT_TABLES, PHASE_INCREMENT_BY_PIECE, PSQT_INDEX,
    },
    search::zobrist::ZobristRandomKeys,
};

use super::{pieces::Piece, player::Player};

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
    // #[inline(never)]
    fn set(&mut self, index: i8) {
        *self |= 1 << index; // SQUARE_MASKS[index as usize];
    }

    // #[inline(never)]
    fn get(&self, index: i8) -> bool {
        self & 1 << index != 0
    }

    // #[inline(never)]
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
    pub boards: [[u64; 7]; 2],
    pub pos_to_player: [u64; 2],
    pub pos_to_piece: [Piece; 64],
}

impl Default for Boards {
    fn default() -> Self {
        Self {
            pos_to_piece: [Piece::Empty; 64],
            pos_to_player: [0; 2],
            boards: Default::default(),
            occupied: Default::default(),
        }
    }
}

impl Boards {
    pub fn get_board_by_piece(&self, player: Player, piece: Piece) -> &u64 {
        return &self.boards[player as usize][piece as usize];
    }

    #[inline(always)]
    pub fn place_piece(
        &mut self,
        player: Player,
        piece: Piece,
        pos: i8,
        phase: &mut i32,
        opening: &mut [i32; 2],
        endgame: &mut [i32; 2],
        hash: &mut u64,
        keys: &ZobristRandomKeys,
    ) -> Piece {
        let removed =
            self.remove_piece(player.opponent(), pos, phase, opening, endgame, hash, keys);

        self.boards[player as usize][piece as usize].set(pos);
        self.pos_to_piece[pos as usize] = piece;
        self.pos_to_player[player as usize].set(pos);
        self.occupied.set(pos);

        let pqst_pos = PSQT_INDEX[player as usize][pos as usize];
        opening[player as usize] += OPENING_PSQT_TABLES[piece as usize][pqst_pos];
        endgame[player as usize] += ENDGAME_PSQT_TABLES[piece as usize][pqst_pos];
        *phase += PHASE_INCREMENT_BY_PIECE[piece as usize];

        *hash ^= keys.pieces[player as usize][piece as usize][pos as usize];
        *hash ^= keys.pieces[player as usize][removed as usize][pos as usize];

        return removed;
    }

    #[inline(always)]
    pub fn remove_piece(
        &mut self,
        player: Player,
        pos: i8,
        phase: &mut i32,
        opening: &mut [i32; 2],
        endgame: &mut [i32; 2],
        hash: &mut u64,
        keys: &ZobristRandomKeys,
    ) -> Piece {
        let removed = self.pos_to_piece[pos as usize];
        self.pos_to_piece[pos as usize] = Piece::Empty;
        self.boards[player as usize][removed as usize].unset(pos);
        self.pos_to_player[player as usize].unset(pos);
        self.occupied.unset(pos);

        // let pqst_pos = if player == Player::White {
        //     pos
        // } else {
        //     pos ^ 56
        // } as usize;
        let pqst_pos = PSQT_INDEX[player as usize][pos as usize];
        opening[player as usize] -= OPENING_PSQT_TABLES[removed as usize][pqst_pos];
        endgame[player as usize] -= ENDGAME_PSQT_TABLES[removed as usize][pqst_pos];
        *phase -= PHASE_INCREMENT_BY_PIECE[removed as usize];

        *hash ^= keys.pieces[player as usize][removed as usize][pos as usize];
        *hash ^= keys.pieces[player as usize][Piece::Empty as usize][pos as usize];

        return removed;
    }
}
