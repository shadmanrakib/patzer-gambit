// https://www.chessprogramming.org/Tapered_Eval

use crate::{boards::BitBoard, pieces::Piece, position::PositionState};

pub const OPENING_PIECE_POINTS: [i32; 7] = [
    0,       // Empty
    80,      // Pawn
    320,     // Knight
    330,     // Bishop
    500,     // Rook
    900,     // Queen
    1000000, // King
];

pub const ENDGAME_PIECE_POINTS: [i32; 7] = [
    0,       // Empty
    130,     // Pawn
    300,     // Knight
    340,     // Bishop
    490,     // Rook
    1000,    // Queen
    1000000, // King
];

type PSQT = [i32; 64];

const fn flip_psqt(mut table: PSQT) -> [i32; 64] {
    let mut i = 0;
    while i < 64 / 2 {
        // i ^ 56 gets the flipped index
        let temp = table[i ^ 56];
        table[i ^ 56] = table[i];
        table[i] = temp;
        i += 1;
    }
    return table;
}

const fn add_value(mut table: PSQT, value: i32) -> [i32; 64] {
    let mut i = 0;
    while i < 64 {
        table[i] += value;
        i += 1;
    }
    return table;
}

const EMPTY_TABLE: PSQT = [0; 64];

#[rustfmt::skip]
const ENDGAME_PAWN_TABLE: PSQT = add_value(flip_psqt([
  0, 0, 0, 0, 0, 0, 0, 0,
  60, 60, 60, 60, 70, 60, 60, 60,
  40, 40, 40, 50, 60, 40, 40, 40,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
]), ENDGAME_PIECE_POINTS[Piece::Pawn as usize]);

#[rustfmt::skip]
const OPENING_PAWN_TABLE: PSQT = add_value(flip_psqt([
  0, 0, 0, 0, 0, 0, 0, 0,
  60, 60, 60, 60, 70, 60, 60, 60,
  40, 40, 40, 50, 60, 40, 40, 40,
  20, 20, 20, 40, 50, 20, 20, 20,
  5, 5, 15, 30, 40, 10, 5, 5,
  5, 5, 10, 20, 30, 5, 5, 5,
  5, 5, 5, -30,  -30, 5, 5, 5,
  0, 0, 0, 0, 0, 0, 0, 0
]), OPENING_PIECE_POINTS[Piece::Pawn as usize]);

#[rustfmt::skip]
const ENDGAME_KNIGHT_TABLE: PSQT = add_value(flip_psqt([
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
]), ENDGAME_PIECE_POINTS[Piece::Knight as usize]);

#[rustfmt::skip]
const OPENING_KNIGHT_TABLE: PSQT = add_value(flip_psqt([
  -10, 0, 0, 0, 0, 0, 0, -10,
  -1, 5, 5, 5, 5, 5, 5, -1,
  -1, 5, 25, 25, 25, 25, 5, -1,
  -1, 5, 25, 25, 25, 25, 5, -1,
  -1, 5, 25, 25, 25, 25, 5, -1,
  -1, 5, 20, 25, 25, 25, 5, -1,
  -1, 5, 5, 5, 5, 5, 5, -1,
  -10, 10, 0, 0, 0, 0, 10, -10
]), OPENING_PIECE_POINTS[Piece::Knight as usize]);

#[rustfmt::skip]
const ENDGAME_BISHOP_TABLE: PSQT = add_value(flip_psqt([
  -20,-10,-10,-10,-10,-10,-10,-20,
  -10,  10,  0,  0,  0,  0,  10,-10,
  -10, 0, 0, 0, 0, 0,0,-10,
  -10,  0, 10, 6, 6, 6,  0,-10,
  -10,  5,  20, 6, 6,  20,  5,-10,
  -10,  13,  10, 7, 7,  10,  0,-10,
  -10,  20,  0,  0,  0,  0,  17,-10,
  -20,-10,-10,-10,-10,-10,-10,-20,
]), ENDGAME_PIECE_POINTS[Piece::Bishop as usize]);

#[rustfmt::skip]
const OPENING_BISHOP_TABLE: PSQT = add_value(flip_psqt([
  -20,-10,-10,-10,-10,-10,-10,-20,
  -10,  10,  0,  0,  0,  0,  10,-10,
  -10, 0, 0, 0, 0, 0,0,-10,
  -10,  0, 10, 6, 6, 6,  0,-10,
  -10,  5,  20, 6, 6,  20,  5,-10,
  -10,  13,  10, 7, 7,  10,  0,-10,
  -10,  20,  0,  0,  0,  0,  17,-10,
  -20,-10,-10,-10,-10,-10,-10,-20,
]), OPENING_PIECE_POINTS[Piece::Bishop as usize]);

#[rustfmt::skip]
const ENDGAME_ROOK_TABLE: PSQT = add_value(flip_psqt([
  0, 0, 0, 0, 0, 0, 0, 0,
  15, 15, 15, 20, 20, 15, 15, 15,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 10, 8, 8, 8, 10, 0,
]), ENDGAME_PIECE_POINTS[Piece::Rook as usize]);

#[rustfmt::skip]
const OPENING_ROOK_TABLE: PSQT = add_value(flip_psqt([
  0, 0, 0, 0, 0, 0, 0, 0,
  15, 15, 15, 20, 20, 15, 15, 15,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 0, 0, 0, 0, 0,
  0, 0, 0, 10, 10, 10, 0, 0
]), OPENING_PIECE_POINTS[Piece::Rook as usize]);

#[rustfmt::skip]
const ENDGAME_QUEEN_TABLE: PSQT = add_value(flip_psqt([
  -20,-10,-10, -5, -5,-10,-10,-20,
  -10,  0,  0,  0,  0,  0,  0,-10,
  -10,  0,  5,  5,  5,  5,  0,-10,
  -5,  0,  5,  5,  5,  5,  0, -5,
  0,  0,  5,  5,  5,  5,  0, -5,
  -10,  5,  5,  5,  5,  5,  0,-10,
  -10,  0,  5,  0,  0,  0,  0,-10,
  -20,-10,-10, -5, -5,-10,-10,-20
]), ENDGAME_PIECE_POINTS[Piece::Queen as usize]);

#[rustfmt::skip]
const OPENING_QUEEN_TABLE: PSQT = add_value(flip_psqt([
  -20,-10,-10, -5, -5,-10,-10,-20,
  -10,  -5,  -5,  -5,  -5,  -5,  -5,-10,
  -10,  -5,  -5,  -5,  -5,  -5,  -5,-10,
  -5,  -5,  -5,  -5,  -5,  -5,  -5, -5,
  0,  -5,  -5,  -5,  -5,  -5,  -5, -5,
  -10,  5,  5,  5,  5,  5,  0,-10,
  -10,  0,  5,  0,  0,  0,  0,-10,
  -20,-10,-10, -5, -5,-10,-10,-20
]), OPENING_PIECE_POINTS[Piece::Queen as usize]);

#[rustfmt::skip]
const ENDGAME_KING_TABLE: PSQT = add_value(flip_psqt([
  -5, -5, -5, -5, -5, -5, -5, -5,
  -5, 0, 0, 0, 0, 0, 0, -5,
  -5, 0, 0, 3, 3, 0, 0, -5,
  -5, 0, 3, 5, 5, 3, 0, -5,
  -5, 0, 3, 5, 5, 3, 0, -5,
  -5, 0, 0, 3, 3, 0, 0, -5,
  -5, 0, 0, 0, 0, 0, 0, -5,
  -5, -5, -5, -5, -5, -5, -5, -5,
]), ENDGAME_PIECE_POINTS[Piece::King as usize]);

#[rustfmt::skip]
const OPENING_KING_TABLE: PSQT = add_value(flip_psqt([
  -20,-20,-20,25,-20,-20,20,-20,
  -20,-20, -20, -20, -20,  20,-20,-20,
  -30,-30, -30, -30, -30, -30,-30,-30,
  -40,-40, -40, -40, -40, -40,-40,-40,
  -40,-40, -40, -40, -40, -40,-40,-40,
  -30,-30, -30, -30, -30, -30,-30,-30,
  -20,-20,-20, -20, -20,-20,-20,-20,
  -20,-20, 35,-20,0,-20,40,-20
]), OPENING_PIECE_POINTS[Piece::King as usize]);

#[rustfmt::skip]
pub const PSQT_WHITE_INDEX: [usize; 64] = [
  0,  1,  2,  3,  4,  5,  6,  7,
  8,  9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63,
];

#[rustfmt::skip]
pub const PSQT_BLACK_INDEX: [usize; 64] = [
  56, 57, 58, 59, 60, 61, 62, 63,
  48, 49, 50, 51, 52, 53, 54, 55,
  40, 41, 42, 43, 44, 45, 46, 47,
  32, 33, 34, 35, 36, 37, 38, 39,
  24, 25, 26, 27, 28, 29, 30, 31,
  16, 17, 18, 19, 20, 21, 22, 23,
   8,  9, 10, 11, 12, 13, 14, 15,
   0,  1,  2,  3,  4,  5,  6,  7,
];

pub const PSQT_INDEX: [[usize; 64]; 2] = [PSQT_WHITE_INDEX, PSQT_BLACK_INDEX];

pub const OPENING_PSQT_TABLES: [PSQT; 7] = [
    EMPTY_TABLE,
    OPENING_PAWN_TABLE,
    OPENING_KNIGHT_TABLE,
    OPENING_BISHOP_TABLE,
    OPENING_ROOK_TABLE,
    OPENING_QUEEN_TABLE,
    OPENING_KING_TABLE,
];

pub const ENDGAME_PSQT_TABLES: [PSQT; 7] = [
    EMPTY_TABLE,
    ENDGAME_PAWN_TABLE,
    ENDGAME_KNIGHT_TABLE,
    ENDGAME_BISHOP_TABLE,
    ENDGAME_ROOK_TABLE,
    ENDGAME_QUEEN_TABLE,
    ENDGAME_KING_TABLE,
];

pub const PHASE_INCREMENT_BY_PIECE: [i32; 7] = [0, 0, 1, 1, 2, 4, 0];
pub const TOTAL_PHASE: i32 = {
    PHASE_INCREMENT_BY_PIECE[Piece::Pawn as usize] * 16
        + PHASE_INCREMENT_BY_PIECE[Piece::Knight as usize] * 4
        + PHASE_INCREMENT_BY_PIECE[Piece::Bishop as usize] * 4
        + PHASE_INCREMENT_BY_PIECE[Piece::Rook as usize] * 4
        + PHASE_INCREMENT_BY_PIECE[Piece::Queen as usize] * 2
};

// inspired by Crafty engine
pub fn eval(position: &PositionState) -> i32 {
    let phase = position.phase;
    let opening = position.opening[0] - position.opening[1];
    let endposition = position.endposition[0] - position.endposition[1];
    // return opening;
    return ((opening * (TOTAL_PHASE - phase)) + (endposition * phase)) / TOTAL_PHASE;
    // return position::simple(position);
}

pub fn init(position: &PositionState) -> (i32, [i32; 2], [i32; 2]) {
    let mut opening = [0; 2];
    let mut endposition = [0; 2];

    let mut phase = TOTAL_PHASE;
    let mut occupied = position.boards.occupied;
    while occupied != 0 {
        let pos = occupied.pop_mut() as usize;
        let piece = position.boards.pos_to_piece[pos] as usize;
        phase -= PHASE_INCREMENT_BY_PIECE[piece as usize];

        if position.boards.pos_to_player[0].get(pos as i8) {
            opening[0] += OPENING_PSQT_TABLES[piece][pos];
            endposition[0] += ENDGAME_PSQT_TABLES[piece][pos];
        } else {
            opening[1] += OPENING_PSQT_TABLES[piece][pos ^ 56];
            endposition[1] += ENDGAME_PSQT_TABLES[piece][pos ^ 56];
        }
    }

    return (phase, opening, endposition);
}
