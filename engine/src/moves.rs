use crate::boards::BitBoard;
use crate::movegen::MoveGenerator;
use crate::movescoring::{score_see, score_tt_mmv_lva_killer};
use crate::pieces::Piece;
use crate::position::PositionState;
use crate::searchinfo::SearchInfo;
use crate::square::Square;
use std::fmt::Debug;
use std::mem::MaybeUninit;

/**
 * Move
 * rough draft, we will use more than we might needed
 * TODO: evaluate whether to use bit encoding to make more compact
 */

#[derive(Debug, Clone)]
pub struct Move {
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
    pub score: i16,
}

impl Move {
    /*
        <move descriptor> ::= <from square><to square>[<promoted to>]
        <square>        ::= <file letter><rank number>
        <file letter>   ::= 'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'
        <rank number>   ::= '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'
        <promoted to>   ::= 'q'|'r'|'b'|'n'
    */
    pub const NULL: Move = Move {
        from_pos: 0,
        to_pos: 0,
        piece: Piece::Empty,
        promotion_piece: Piece::Empty,
        captured_piece: Piece::Empty,
        promoting: false,
        capturing: false,
        double: false,
        enpassant: false,
        castling: false,
        score: 0,
    };
}
impl ToString for Move {
    fn to_string(&self) -> String {
        let from_square = Square::from(self.from_pos).stringify();
        let to_square = Square::from(self.to_pos).stringify();
        let promotion = {
            if self.promoting {
                self.promotion_piece.to_string().to_lowercase()
            } else {
                "".to_string()
            }
        };
        return format!("{from_square}{to_square}{promotion}");
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnmakeMoveMetadata {
    pub prev_castle_permissions: u8,
    // 0-7 maps to columns A-H, 8 is none
    pub prev_enpassant_square: u64,
    // It marks the number of moves since the last pawn push or piece capture.
    pub prev_half_move_clock: u32,
    pub captured_piece: Piece,
}

/**
 * SimpleMove
 * A simpler move struct to use for storing.
 * TODO: evaluate whether to use bit encoding to make more compact
 */

#[derive(Clone, Copy, PartialEq)]
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
    pub fn is_similar(&self, move_item: &Move) -> bool {
        self.from == move_item.from_pos
            && self.to == move_item.to_pos
            && self.promotion == move_item.promotion_piece
    }
}

impl Into<SimpleMove> for &Move {
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

/**
 * MoveList
 * Stores moves in uninitialized list
 */

pub struct MoveList {
    pub moves: [Move; 256],
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
    pub fn push(&mut self, move_item: Move) {
        self.moves[self.end] = move_item;
        self.end += 1;
    }
    #[inline(always)]
    pub fn push_multiple_moves(
        &mut self,
        position: &PositionState,
        from: i8,
        mut tos: u64,
        piece: Piece,
        promotion_piece: Piece,
        promoting: bool,
        capturing: bool,
        double: bool,
        enpassant: bool,
        castling: bool,
    ) {
        while tos != 0 {
            let to = tos.pop_mut();
            self.push(Move {
                from_pos: from,
                to_pos: to,
                piece,
                promotion_piece,
                captured_piece: if capturing {
                    position.boards.pos_to_piece[to as usize]
                } else {
                    Piece::Empty
                },
                promoting,
                capturing,
                double,
                enpassant,
                castling,
                score: 0,
            })
        }
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
    pub fn score_moves(&mut self, search_cache: &mut SearchInfo, ply: usize, tt_move: &SimpleMove) {
        for i in 0..self.len() {
            score_tt_mmv_lva_killer(&mut self.moves[i], search_cache, ply, tt_move);
        }
    }
    pub fn score_captures(&mut self, position: &PositionState, generator: &MoveGenerator) {
        for i in 0..self.len() {
            // score_mmv_lva(&mut self.moves[i]);
            score_see(&mut self.moves[i], position, generator);
        }
    }
}
