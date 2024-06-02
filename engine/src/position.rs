use regex::Regex;

use super::{boards::Boards, pieces::Piece, player::Player};
use crate::{
    boards::BitBoard,
    evaluation::{
        self, ENDGAME_PSQT_TABLES, OPENING_PSQT_TABLES, PHASE_INCREMENT_BY_PIECE, PSQT_INDEX,
        TOTAL_PHASE,
    },
    fen,
    magics::hash_with_magic,
    movegen::MoveGenerator,
    moves::{Move, UnmakeMoveMetadata},
    square::Square,
    zobrist::ZobristHasher,
};

// we will make this a bitmap
// abcd, a - white_queen_side, b - white_king_side, c - black_queen_side, d - black_king_side
// ex 1100 -> KQ
pub trait CastlePermissions {
    const WHITE_KING_SIDE: u8 = (1 << 0);
    const WHITE_QUEEN_SIDE: u8 = (1 << 1);
    const BLACK_KING_SIDE: u8 = (1 << 2);
    const BLACK_QUEEN_SIDE: u8 = (1 << 3);
    fn is_allowed(&self, perm: u8) -> bool;
    fn grant(&mut self, perm: u8);
    fn revoke(&mut self, perm: u8);
    fn revoke_white(&mut self);
    fn revoke_black(&mut self);
    fn revoke_all(&mut self);
}
impl CastlePermissions for u8 {
    fn is_allowed(&self, perm: u8) -> bool {
        self & perm != 0
    }
    fn grant(&mut self, perm: u8) {
        *self |= perm;
    }
    fn revoke(&mut self, perm: u8) {
        *self &= !perm;
    }
    fn revoke_white(&mut self) {
        *self &= Self::BLACK_KING_SIDE | Self::BLACK_QUEEN_SIDE;
    }
    fn revoke_black(&mut self) {
        *self &= Self::WHITE_KING_SIDE | Self::WHITE_QUEEN_SIDE;
    }
    fn revoke_all(&mut self) {
        *self = 0;
    }
}

#[derive(Debug, Clone)]
pub struct EnpassantSquare {
    pub exists: bool,
    pub pos: Square,
}

// inspired by FEN notation
#[derive(Debug, Clone)]
pub struct PositionState {
    pub boards: Boards,
    pub side_to_move: Player,
    pub castle_permissions: u8,
    // 0-7 maps to columns A-H, 8 is none
    pub enpassant_square: u64,
    // It marks the number of moves since the last pawn push or piece capture.
    pub half_move_clock: u32,
    // It marks the number of full moves. It starts at 1 and is incremented after black's move.
    pub full_move_number: u32,

    // for 3 fold repetion and undoing
    pub history: Vec<(Move, UnmakeMoveMetadata, u64)>,

    // for pqst
    pub phase: i32,
    pub opening: [i32; 2],
    pub endposition: [i32; 2],
    pub color: i32,
    pub hash: u64,
}

impl PositionState {
    #[allow(dead_code)]
    pub fn set(&mut self, other: PositionState) {
        self.boards = other.boards;
        self.side_to_move = other.side_to_move;
        self.castle_permissions = other.castle_permissions;
        self.enpassant_square = other.enpassant_square;
        self.half_move_clock = other.half_move_clock;
        self.full_move_number = other.full_move_number;
        self.phase = other.phase;
        self.color = other.color;
        self.opening = other.opening;
        self.endposition = other.endposition;
        self.history = other.history;
    }
    // TODO: will implement later
    pub fn notation_to_move(
        &self,
        notation: String,
        generator: &MoveGenerator,
    ) -> Result<Move, String> {
        let re = Regex::new(r"([a-h][1-8])([a-h][1-8])([nbrq])?").unwrap();

        if re.is_match(&notation) {
            let moves_list = generator.generate_moves(self);
            for index in 0..moves_list.len() {
                let move_item = &moves_list.moves[index];

                if move_item.to_string() == notation {
                    return Ok(move_item.clone());
                }
            }

            return Err("Move not found".into());
        }

        return Err("Invalid long algrebraic notation.".into());
    }

    pub fn has_three_fold_rep(&self) -> bool {
        let size = self.history.len();
        let mut count = 1;
        for i in (0..size - 1).rev().step_by(2) {
            // a piece was captured so the piece count got altered, so no way for same position to arise
            // only way for that piece to reappear is potentially pawn promotion
            // but that means we lose a pawn, so still unequal positions
            if self.history[i].0.capturing {
                return false;
            }
            if self.history[i].2 == self.hash {
                count += 1;

                if count >= 3 {
                    return true;
                }
            }
        }

        false
    }

    pub fn has_insufficient_material(&self) -> bool {
        // there is no chance for there to be insufficient material if we still have 4 bishops
        // worth of materials on the board
        if self.phase < TOTAL_PHASE - PHASE_INCREMENT_BY_PIECE[Piece::Bishop as usize] * 4 {
            return false;
        }

        let num_wp =
            self.boards.bitboards[Player::White as usize][Piece::Pawn as usize].count_ones();
        let num_bp =
            self.boards.bitboards[Player::Black as usize][Piece::Pawn as usize].count_ones();

        let num_wq =
            self.boards.bitboards[Player::White as usize][Piece::Queen as usize].count_ones();
        let num_bq =
            self.boards.bitboards[Player::Black as usize][Piece::Queen as usize].count_ones();

        let num_wr =
            self.boards.bitboards[Player::White as usize][Piece::Rook as usize].count_ones();
        let num_br =
            self.boards.bitboards[Player::Black as usize][Piece::Rook as usize].count_ones();

        let num_wn =
            self.boards.bitboards[Player::White as usize][Piece::Knight as usize].count_ones();
        let num_bn =
            self.boards.bitboards[Player::Black as usize][Piece::Knight as usize].count_ones();

        let num_wb =
            self.boards.bitboards[Player::White as usize][Piece::Bishop as usize].count_ones();
        let num_bb =
            self.boards.bitboards[Player::Black as usize][Piece::Bishop as usize].count_ones();

        // its possible to mate if you have a pawn, a rook, a queen, or bishop + knight
        // assuming pawn promotes
        if num_wp > 0
            || num_bp > 0
            || num_wq > 0
            || num_bq > 0
            || num_wr > 0
            || num_br > 0
            || (num_wb > 0 && num_wn > 0)
            || (num_bb > 0 && num_bn > 0)
        {
            return false;
        }

        // its possible to mate with two bishops of diff colors.
        let white_squares_mask = 0x55aa55aa55aa552a;
        let black_squares_mask = 0xaa55aa55aa55aa55;
        let num_wwb = (self.boards.bitboards[Player::White as usize][Piece::Bishop as usize]
            & white_squares_mask)
            .count_ones();
        let num_wbb = (self.boards.bitboards[Player::White as usize][Piece::Bishop as usize]
            & black_squares_mask)
            .count_ones();
        let num_bwb = (self.boards.bitboards[Player::Black as usize][Piece::Bishop as usize]
            & white_squares_mask)
            .count_ones();
        let num_bbb = (self.boards.bitboards[Player::Black as usize][Piece::Bishop as usize]
            & black_squares_mask)
            .count_ones();

        if (num_wwb > 0 && num_wbb > 0) || (num_bwb > 0 && num_bbb > 0) {
            return false;
        }

        return true;
    }

    #[inline(always)]
    pub fn score(&self) -> i32 {
        if self.has_insufficient_material()
            || self.has_three_fold_rep()
            || self.half_move_clock >= 50
        {
            return 0;
        }
        self.color * evaluation::eval(self)
    }

    #[inline(always)]
    pub fn place_piece(&mut self, player: Player, piece: Piece, pos: i8, zobrist: &ZobristHasher) {
        self.boards.place_piece(player, piece, pos);

        // make incremental updates
        // places piece updates
        let pqst_pos = PSQT_INDEX[player as usize][pos as usize];
        self.opening[player as usize] += OPENING_PSQT_TABLES[piece as usize][pqst_pos];
        self.endposition[player as usize] += ENDGAME_PSQT_TABLES[piece as usize][pqst_pos];
        self.phase -= PHASE_INCREMENT_BY_PIECE[piece as usize];
        self.hash ^= zobrist.pieces[player as usize][piece as usize][pos as usize];
    }

    #[inline(always)]
    pub fn remove_piece(&mut self, player: Player, pos: i8, zobrist: &ZobristHasher) -> Piece {
        let removed = self.boards.remove_piece(player, pos);

        let pqst_pos = PSQT_INDEX[player as usize][pos as usize];
        self.opening[player as usize] -= OPENING_PSQT_TABLES[removed as usize][pqst_pos];
        self.endposition[player as usize] -= ENDGAME_PSQT_TABLES[removed as usize][pqst_pos];
        self.phase += PHASE_INCREMENT_BY_PIECE[removed as usize];
        self.hash ^= zobrist.pieces[player as usize][removed as usize][pos as usize];

        return removed;
    }

    pub fn make_move(
        &mut self,
        move_item: Move,
        generator: &MoveGenerator,
        zobrist: &ZobristHasher,
    ) -> bool {
        // keep copies for undoing metadata
        let prev_hash = self.hash;
        let prev_castle_permissions = self.castle_permissions.clone();
        let prev_enpassant_square = self.enpassant_square;
        let prev_half_move_clock = self.half_move_clock;

        // remove prev from hash
        self.hash ^= zobrist.castling[self.castle_permissions as usize];
        self.hash ^= zobrist.enpassant[self.enpassant_square.trailing_zeros() as usize];

        // start making the move
        let side_moving = self.side_to_move;
        let opponent = side_moving.opponent();

        // lets now make this optimistic removal
        // all other moves get handled individually
        self.remove_piece(side_moving, move_item.from_pos, zobrist);

        if move_item.piece == Piece::Pawn {
            let final_piece = if !move_item.promoting {
                move_item.piece
            } else {
                move_item.promotion_piece
            };

            // handle pawn left from enpassant capture
            if move_item.enpassant {
                let rank = Square::rank(move_item.from_pos);
                let file = Square::file(move_item.to_pos);

                let leftover = Square::index(rank, file);

                // capture enpassant
                self.remove_piece(opponent, leftover, zobrist);
            } else {
                // capture regular if present
                self.remove_piece(opponent, move_item.to_pos, zobrist);
            }

            self.place_piece(side_moving, final_piece, move_item.to_pos, zobrist);
        } else {
            // capture if present
            self.remove_piece(opponent, move_item.to_pos, zobrist);
            // place piece if present
            self.place_piece(side_moving, move_item.piece, move_item.to_pos, zobrist);

            // we need to move the rooks too for castling
            if move_item.castling {
                match move_item.to_pos {
                    2 => {
                        self.remove_piece(side_moving, 0, zobrist);
                        self.place_piece(side_moving, Piece::Rook, 3, zobrist);
                    }
                    6 => {
                        self.remove_piece(side_moving, 7, zobrist);
                        self.place_piece(side_moving, Piece::Rook, 5, zobrist);
                    }
                    58 => {
                        self.remove_piece(side_moving, 56, zobrist);
                        self.place_piece(side_moving, Piece::Rook, 59, zobrist);
                    }
                    62 => {
                        self.remove_piece(side_moving, 63, zobrist);
                        self.place_piece(side_moving, Piece::Rook, 61, zobrist);
                    }
                    _ => {}
                }
            }

            // half move clock needs to be incremented if no capture, castle, or pawn move
            // to enforce draw by 50 moves rule, else set to 0 to reset
            if !move_item.capturing && !move_item.castling {
                self.half_move_clock += 1;
            } else {
                self.half_move_clock = 0;
            }
        }

        // ==============================================

        // we want to create new enpassant square if needed or revoke old
        if move_item.double {
            let (to_rank, file) = Square::rank_and_file(move_item.to_pos);
            let from_rank = Square::rank(move_item.from_pos);
            let enpassant_rank: i8 = {
                if to_rank > from_rank {
                    to_rank - 1
                } else {
                    to_rank + 1
                }
            };
            self.enpassant_square = 1 << Square::index(enpassant_rank, file);
        } else {
            self.enpassant_square = 0;
        }

        // full move number needs to be incremented if side to play is black
        if side_moving == Player::Black {
            self.full_move_number += 1;
        }

        // revoke necessary castling permissions
        if move_item.castling || move_item.piece == Piece::King {
            match side_moving {
                Player::White => {
                    self.castle_permissions.revoke_white();
                }
                Player::Black => {
                    self.castle_permissions.revoke_black();
                }
            }
        } else if move_item.piece == Piece::Rook {
            match move_item.from_pos {
                0 => {
                    self.castle_permissions.revoke(u8::WHITE_QUEEN_SIDE);
                }
                7 => {
                    self.castle_permissions.revoke(u8::WHITE_KING_SIDE);
                }
                56 => {
                    self.castle_permissions.revoke(u8::BLACK_QUEEN_SIDE);
                }
                63 => {
                    self.castle_permissions.revoke(u8::BLACK_KING_SIDE);
                }
                _ => {}
            }
        }
        if move_item.captured_piece == Piece::Rook {
            match move_item.to_pos {
                0 => {
                    self.castle_permissions.revoke(u8::WHITE_QUEEN_SIDE);
                }
                7 => {
                    self.castle_permissions.revoke(u8::WHITE_KING_SIDE);
                }
                56 => {
                    self.castle_permissions.revoke(u8::BLACK_QUEEN_SIDE);
                }
                63 => {
                    self.castle_permissions.revoke(u8::BLACK_KING_SIDE);
                }
                _ => {}
            }
        }

        // side to play needs to change to opposite
        self.side_to_move = opponent;
        self.color *= -1;

        // new values to hash
        self.hash ^= zobrist.castling[self.castle_permissions as usize];
        self.hash ^= zobrist.enpassant[self.enpassant_square.trailing_zeros() as usize];
        self.hash ^= zobrist.side_to_move;

        let unmake_metadata = UnmakeMoveMetadata {
            captured_piece: move_item.captured_piece,
            prev_castle_permissions,
            prev_enpassant_square,
            prev_half_move_clock,
        };
        self.history.push((move_item, unmake_metadata, prev_hash));

        if self.in_check(self.side_to_move.opponent(), generator) {
            self.unmake_move(zobrist);
            return false;
        }

        true
    }

    pub fn unmake_move(&mut self, zobrist: &ZobristHasher) {
        if let Some((move_item, unmake_metadata, prev_hash)) = self.history.pop() {
            // lets handle the easy undos
            self.castle_permissions = unmake_metadata.prev_castle_permissions;

            self.half_move_clock = unmake_metadata.prev_half_move_clock;
            self.enpassant_square = unmake_metadata.prev_enpassant_square;
            if self.side_to_move == Player::White {
                self.full_move_number -= 1;
            }
            self.side_to_move = self.side_to_move.opponent();
            self.color *= -1;

            // lets move the original piece to its position
            self.place_piece(
                self.side_to_move,
                move_item.piece,
                move_item.from_pos,
                zobrist,
            );

            // lets remove the to piece preliminarly
            self.remove_piece(self.side_to_move, move_item.to_pos, zobrist);

            // lets place back the captured piece, if not enpassant
            if move_item.capturing {
                if !move_item.enpassant {
                    self.place_piece(
                        self.side_to_move.opponent(),
                        unmake_metadata.captured_piece,
                        move_item.to_pos,
                        zobrist,
                    );
                } else {
                    // we have an enpassant so we need to do a bit more calculation
                    // for where to place the captured piece
                    let from = Square::from(move_item.from_pos);
                    let to = Square::from(move_item.to_pos);

                    let rank = from.rank;
                    let file = to.file;

                    let captured_square = Square::index(rank, file);

                    self.place_piece(
                        self.side_to_move.opponent(),
                        unmake_metadata.captured_piece,
                        captured_square.into(),
                        zobrist,
                    );
                }
            }

            if move_item.castling {
                match move_item.to_pos {
                    2 => {
                        self.place_piece(self.side_to_move, Piece::Rook, 0, zobrist);
                        self.remove_piece(self.side_to_move, 3, zobrist);
                    }
                    6 => {
                        self.place_piece(self.side_to_move, Piece::Rook, 7, zobrist);
                        self.remove_piece(self.side_to_move, 5, zobrist);
                    }
                    58 => {
                        self.place_piece(self.side_to_move, Piece::Rook, 56, zobrist);
                        self.remove_piece(self.side_to_move, 59, zobrist);
                    }
                    62 => {
                        self.place_piece(self.side_to_move, Piece::Rook, 63, zobrist);
                        self.remove_piece(self.side_to_move, 61, zobrist);
                    }
                    _ => {}
                }
            }

            self.hash = prev_hash;
        }
    }

    pub fn make_null_move(&mut self, zobrist: &ZobristHasher) -> u64 {
        let null_unmake = UnmakeMoveMetadata {
            prev_castle_permissions: self.castle_permissions,
            prev_enpassant_square: self.enpassant_square,
            prev_half_move_clock: self.half_move_clock,
            captured_piece: Piece::Empty,
        };
        self.history.push((Move::NULL, null_unmake, self.hash));

        // incremental updates
        self.hash ^= zobrist.side_to_move;
        self.color *= -1;
        self.hash ^= zobrist.enpassant[self.enpassant_square.trailing_zeros() as usize]; // remove enpassant from hash

        // make null move
        self.side_to_move = self.side_to_move.opponent();
        let enpassant = self.enpassant_square;
        self.enpassant_square = 0;

        return enpassant;
    }
    pub fn unmake_null_move(&mut self, enpassant: u64) {
        if let Some((_, _, prev_hash)) = self.history.pop() {
            // restore values
            self.side_to_move = self.side_to_move.opponent();
            self.enpassant_square = enpassant;
            self.color *= -1;
            self.hash = prev_hash;
        }
    }

    #[inline(always)]
    pub fn is_square_attacked(&self, pos: i8, attacker: Player, generator: &MoveGenerator) -> bool {
        let occupied = &self.boards.occupied;

        // knight
        let knight_move_mask = generator.knight_moves_masks[pos as usize];
        let attacking_knights =
            knight_move_mask & self.boards.get_board_by_piece(attacker, Piece::Knight);
        if attacking_knights != 0 {
            return true;
        }

        // rook and queen vertical and horizontal
        let rook_magic_index = hash_with_magic(generator.rook_magics[pos as usize], occupied);
        let rook_moves_mask = generator.rook_magic_attack_tables[rook_magic_index];
        let attacking_rooks =
            rook_moves_mask & self.boards.get_board_by_piece(attacker, Piece::Rook);
        let attacking_queens_straight =
            rook_moves_mask & self.boards.get_board_by_piece(attacker, Piece::Queen);

        if attacking_rooks != 0 || attacking_queens_straight != 0 {
            return true;
        }

        // bishop and queen diagonal
        let bishop_magic_index = hash_with_magic(generator.bishop_magics[pos as usize], occupied);
        let bishop_moves_mask = generator.bishop_magic_attack_tables[bishop_magic_index];

        let attacking_bishops =
            bishop_moves_mask & self.boards.get_board_by_piece(attacker, Piece::Bishop);
        let attacking_queens_diagonal =
            bishop_moves_mask & self.boards.get_board_by_piece(attacker, Piece::Queen);

        if attacking_bishops != 0 || attacking_queens_diagonal != 0 {
            return true;
        }

        // pawn attack
        let opponent_pawns = self.boards.get_board_by_piece(attacker, Piece::Pawn);
        let attacking_mask =
            generator.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
        let attacking_pawns = opponent_pawns & attacking_mask;
        if attacking_pawns != 0 {
            return true;
        }

        // king attack
        let king_move_mask = generator.king_moves_masks[pos as usize];
        let attacking_king = king_move_mask & self.boards.get_board_by_piece(attacker, Piece::King);
        if attacking_king != 0 {
            return true;
        }

        return false;
    }

    // #[inline(always)]
    pub fn in_check(&self, player: Player, generator: &MoveGenerator) -> bool {
        let king = self
            .boards
            .get_board_by_piece(player, Piece::King)
            .trailing_zeros() as i8;

        return self.is_square_attacked(king, player.opponent(), generator);
    }

    pub fn new(key: &ZobristHasher) -> PositionState {
        let start_board_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into();
        return Self::from_fen(start_board_fen, key).unwrap();
    }

    pub fn from_fen(fen: String, zobrist: &ZobristHasher) -> Result<PositionState, String> {
        let parts: Vec<&str> = fen.split(" ").collect();

        if parts.len() != 6 {
            return Err("Fen missing parts".to_string());
        }

        let mut position = PositionState {
            boards: fen::parse_fen_board(parts[0]).unwrap(),
            side_to_move: fen::parse_fen_side(parts[1]).unwrap(),
            castle_permissions: fen::parse_fen_castle(parts[2]).unwrap(),
            enpassant_square: fen::parse_fen_enpassant(parts[3]).unwrap(),
            half_move_clock: parts[4].parse::<u32>().unwrap(),
            full_move_number: parts[5].parse::<u32>().unwrap(),
            // temp values
            history: Vec::new(),
            phase: 0,
            color: 0,
            opening: [0, 0],
            endposition: [0, 0],
            hash: 0,
        };

        // initialize values needed for scoring
        let (phase, opening, endposition) = evaluation::init(&position);
        position.phase = phase;
        position.opening = opening;
        position.endposition = endposition;
        position.color = if position.side_to_move == Player::White {
            1
        } else {
            -1
        };
        position.hash = zobrist.hash(&position);
        position.history = vec![];

        return Ok(position);
    }

    pub fn to_fen(&self) -> String {
        let board = fen::stringify_board(self);
        let side = fen::stringify_side(self);
        let castling = fen::stringify_castling(self);
        let enpassant = fen::stringify_enpassant(self);
        let half_move: u32 = self.half_move_clock;
        let full_move: u32 = self.full_move_number;

        return format!("{board} {side} {castling} {enpassant} {half_move} {full_move}");
    }

    pub fn print_board(&self) {
        const BLACK_PIECES: [&str; 7] = [".", "♟︎", "♞", "♝", "♜", "♛", "♚"];
        const WHITE_PIECES: [&str; 7] = [".", "♙", "♘", "♗", "♖", "♕", "♔"];

        for rank in (0..8).rev() {
            print!("{} | ", rank + 1);
            for file in 0..8 {
                let pos = 8 * rank + file;

                let piece = self.boards.pos_to_piece[pos];
                let colored = if self.boards.pos_to_player[Player::White as usize].get(pos as i8) {
                    BLACK_PIECES[piece as usize]
                } else {
                    WHITE_PIECES[piece as usize]
                };

                print!("{} ", colored);
            }
            println!();
        }
        println!("  -----------------");
        println!("    A B C D E F G H");
    }

    pub fn print_state(&self) {
        self.print_board();
        println!();
        println!("fen: {}", self.to_fen());
        println!("zobrist: {}", self.hash);
        println!("score: {}", self.score());
    }
}
