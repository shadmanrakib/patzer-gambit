use regex::Regex;

use super::{boards::Boards, pieces::Piece, player::Player, square::Square};
use crate::{
    constants::masks::SQUARE_MASKS,
    fen,
    moves::{
        move_data::{MoveItem, UnmakeMoveMetadata},
        precalculate::cache::PrecalculatedCache,
        pseudolegal::all::generate_pseudolegal_moves,
    },
    search::zobrist::{calculate_zobrist_hash, ZobristRandomKeys},
    state::{boards::BitBoard, movelist::MoveList},
};

// we will make this a bitmap
// abcd, a - white_queen_side, b - white_king_side, c - black_queen_side, d - black_king_side
// ex 1100 -> KQ
pub trait CastlePermissions {
    const WHITE_KING_SIDE: u8 = 1 << 3;
    const WHITE_QUEEN_SIDE: u8 = 1 << 2;
    const BLACK_KING_SIDE: u8 = 1 << 1;
    const BLACK_QUEEN_SIDE: u8 = 1 << 0;

    fn is_allowed(&self, i: u8) -> bool;
    fn grant(&mut self, i: u8);
    fn revoke(&mut self, i: u8);
    fn revoke_all(&mut self);
    fn revoke_white(&mut self);
    fn revoke_black(&mut self);
}
impl CastlePermissions for u8 {
    // const WHITE_KING_SIDE: u8 = 1 << 3;
    // const WHITE_QUEEN_SIDE: u8 = 1 << 2;
    // const BLACK_KING_SIDE: u8 = 1 << 1;
    // const BLACK_QUEEN_SIDE: u8 = 1 << 0;

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
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub bitboards: Boards,
    pub side_to_move: Player,
    pub castle_permissions: u8,
    // 0-7 maps to columns A-H, 8 is none
    pub enpassant_square: u64,
    // It marks the number of moves since the last pawn push or piece capture.
    pub half_move_clock: u32,
    // It marks the number of full moves. It starts at 1 and is incremented after black's move.
    pub full_move_number: u32,

    // for pqst
    pub phase: i32,
    pub opening: [i32; 2],
    pub endgame: [i32; 2],
    pub opening_pqst_score: i32,
    pub endgame_pqst_score: i32,
    pub color: i32,
    pub hash: u64,
}

impl GameState {
    #[allow(dead_code)]
    pub fn set(&mut self, other: GameState) {
        self.bitboards = other.bitboards;
        self.side_to_move = other.side_to_move;
        self.castle_permissions = other.castle_permissions;
        self.enpassant_square = other.enpassant_square;
        self.half_move_clock = other.half_move_clock;
        self.full_move_number = other.full_move_number;
        self.phase = other.phase;
        self.opening_pqst_score = other.opening_pqst_score;
        self.endgame_pqst_score = other.endgame_pqst_score;
        self.color = other.color;
        self.opening = other.opening;
        self.endgame = other.endgame;
    }
    // TODO: will implement later
    pub fn notation_to_move(
        &self,
        notation: String,
        cache: &PrecalculatedCache,
    ) -> Result<MoveItem, String> {
        let re = Regex::new(r"([a-h][1-8])([a-h][1-8])([nbrq])?").unwrap();

        if re.is_match(&notation) {
            let mut moveslist = MoveList::new();
            generate_pseudolegal_moves(&mut moveslist, self, self.side_to_move, cache);

            for index in 0..moveslist.len() {
                let move_item = &moveslist.moves[index];

                if move_item.pure_algebraic_coordinate_notation() == notation {
                    return Ok(move_item.clone());
                }
            }

            return Err("Move not found".into());
        }

        return Err("Invalid long algrebraic notation.".into());
    }
    pub fn make_move(
        &mut self,
        move_item: &MoveItem,
        keys: &ZobristRandomKeys,
    ) -> UnmakeMoveMetadata {
        self.hash ^= keys.castling[self.castle_permissions as usize];
        let hash_enpassant_sq = std::cmp::min(self.enpassant_square.trailing_zeros(), 63) as usize;
        self.hash ^= keys.enpassant[hash_enpassant_sq as usize];
        self.hash ^= keys.side_to_move[self.side_to_move as usize];

        let prev_castle_permissions = self.castle_permissions.clone();
        let prev_enpassant_square = self.enpassant_square;
        let prev_half_move_clock = self.half_move_clock;

        // ==============================================

        let side_moving = self.side_to_move;
        let opponent = side_moving.opponent();

        // lets now make the move
        // all other moves get handled
        self.bitboards.remove_piece(
            side_moving,
            move_item.from_pos,
            &mut self.phase,
            &mut self.opening,
            &mut self.endgame,
            &mut self.hash,
            keys,
        );

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

                self.bitboards.remove_piece(
                    opponent,
                    leftover,
                    &mut self.phase,
                    &mut self.opening,
                    &mut self.endgame,
                    &mut self.hash,
                    keys,
                );
            } else {
                self.bitboards.remove_piece(
                    opponent,
                    move_item.to_pos,
                    &mut self.phase,
                    &mut self.opening,
                    &mut self.endgame,
                    &mut self.hash,
                    keys,
                );
            }

            self.bitboards.place_piece(
                side_moving,
                final_piece,
                move_item.to_pos,
                &mut self.phase,
                &mut self.opening,
                &mut self.endgame,
                &mut self.hash,
                keys,
            );
        } else {
            self.bitboards.remove_piece(
                opponent,
                move_item.to_pos,
                &mut self.phase,
                &mut self.opening,
                &mut self.endgame,
                &mut self.hash,
                keys,
            );

            self.bitboards.place_piece(
                side_moving,
                move_item.piece,
                move_item.to_pos,
                &mut self.phase,
                &mut self.opening,
                &mut self.endgame,
                &mut self.hash,
                keys,
            );

            if move_item.castling {
                // move rook to place
                match (self.side_to_move, move_item.to_pos) {
                    (Player::White, 2) => {
                        self.bitboards.remove_piece(
                            side_moving,
                            0,
                            &mut self.phase,
                            &mut self.opening,
                            &mut self.endgame,
                            &mut self.hash,
                            keys,
                        );
                        self.bitboards.place_piece(
                            side_moving,
                            Piece::Rook,
                            3,
                            &mut self.phase,
                            &mut self.opening,
                            &mut self.endgame,
                            &mut self.hash,
                            keys,
                        );
                    }
                    (Player::White, 6) => {
                        self.bitboards.remove_piece(
                            side_moving,
                            7,
                            &mut self.phase,
                            &mut self.opening,
                            &mut self.endgame,
                            &mut self.hash,
                            keys,
                        );
                        self.bitboards.place_piece(
                            side_moving,
                            Piece::Rook,
                            5,
                            &mut self.phase,
                            &mut self.opening,
                            &mut self.endgame,
                            &mut self.hash,
                            keys,
                        );
                    }
                    (Player::Black, 58) => {
                        self.bitboards.remove_piece(
                            side_moving,
                            56,
                            &mut self.phase,
                            &mut self.opening,
                            &mut self.endgame,
                            &mut self.hash,
                            keys,
                        );
                        self.bitboards.place_piece(
                            side_moving,
                            Piece::Rook,
                            59,
                            &mut self.phase,
                            &mut self.opening,
                            &mut self.endgame,
                            &mut self.hash,
                            keys,
                        );
                    }
                    (Player::Black, 62) => {
                        self.bitboards.remove_piece(
                            side_moving,
                            63,
                            &mut self.phase,
                            &mut self.opening,
                            &mut self.endgame,
                            &mut self.hash,
                            keys,
                        );
                        self.bitboards.place_piece(
                            side_moving,
                            Piece::Rook,
                            61,
                            &mut self.phase,
                            &mut self.opening,
                            &mut self.endgame,
                            &mut self.hash,
                            keys,
                        );
                    }
                    (_, _) => {
                        // TODO: handle error
                    }
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

        // we want to create new enpassant square if needed, or revoke old
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
            self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
        } else {
            self.enpassant_square = 0;
        }

        // full move number needs to be incremented if side to play is black
        if side_moving == Player::Black {
            self.full_move_number += 1;
        }

        // revoke necessary castling permissions
        // || move_item.piece == Piece::King(self.side_to_move)
        if move_item.castling || move_item.piece == Piece::King {
            match side_moving {
                Player::White => {
                    self.castle_permissions.revoke_white();
                }
                Player::Black => {
                    self.castle_permissions.revoke_black();
                }
            }
        }
        if move_item.piece == Piece::Rook {
            match (side_moving, move_item.from_pos) {
                (Player::White, 0) => {
                    self.castle_permissions.revoke(u8::WHITE_QUEEN_SIDE);
                }
                (Player::White, 7) => {
                    self.castle_permissions.revoke(u8::WHITE_KING_SIDE);
                }
                (Player::Black, 56) => {
                    self.castle_permissions.revoke(u8::BLACK_QUEEN_SIDE);
                }
                (Player::Black, 63) => {
                    self.castle_permissions.revoke(u8::BLACK_KING_SIDE);
                }
                (_, _) => {}
            }
        }
        if move_item.captured_piece == Piece::Rook {
            match (opponent, move_item.to_pos) {
                (Player::White, 0) => {
                    self.castle_permissions.revoke(u8::WHITE_QUEEN_SIDE);
                }
                (Player::White, 7) => {
                    self.castle_permissions.revoke(u8::WHITE_KING_SIDE);
                }
                (Player::Black, 56) => {
                    self.castle_permissions.revoke(u8::BLACK_QUEEN_SIDE);
                }
                (Player::Black, 63) => {
                    self.castle_permissions.revoke(u8::BLACK_KING_SIDE);
                }
                (_, _) => {}
            }
        }

        // side to play needs to change to opposite
        self.side_to_move = opponent;
        self.color *= -1;

        self.hash ^= keys.castling[self.castle_permissions as usize];
        let hash_enpassant_sq = std::cmp::min(self.enpassant_square.trailing_zeros(), 63) as usize;
        self.hash ^= keys.enpassant[hash_enpassant_sq as usize];
        self.hash ^= keys.side_to_move[self.side_to_move as usize];

        UnmakeMoveMetadata {
            captured_piece: move_item.captured_piece,
            prev_castle_permissions,
            prev_enpassant_square,
            prev_half_move_clock,
        }
    }

    pub fn unmake_move(
        &mut self,
        move_item: &MoveItem,
        unmake_metadata: UnmakeMoveMetadata,
        keys: &ZobristRandomKeys,
    ) {
        self.hash ^= keys.castling[self.castle_permissions as usize];
        let hash_enpassant_sq = std::cmp::min(self.enpassant_square.trailing_zeros(), 63) as usize;
        self.hash ^= keys.enpassant[hash_enpassant_sq as usize];
        self.hash ^= keys.side_to_move[self.side_to_move as usize];

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
        self.bitboards.place_piece(
            self.side_to_move,
            move_item.piece,
            move_item.from_pos,
            &mut self.phase,
            &mut self.opening,
            &mut self.endgame,
            &mut self.hash,
            keys,
        );

        // lets remove the to piece preliminarly
        self.bitboards.remove_piece(
            self.side_to_move,
            move_item.to_pos,
            &mut self.phase,
            &mut self.opening,
            &mut self.endgame,
            &mut self.hash,
            keys,
        );

        // lets place back the captured piece, if not enpassant
        if move_item.capturing {
            if !move_item.enpassant {
                self.bitboards.place_piece(
                    self.side_to_move.opponent(),
                    unmake_metadata.captured_piece,
                    move_item.to_pos,
                    &mut self.phase,
                    &mut self.opening,
                    &mut self.endgame,
                    &mut self.hash,
                    keys,
                );
            } else {
                // we have an enpassant so we need to do a bit more calculation
                // for where to place the captured piece
                let from = Square::from(move_item.from_pos);
                let to = Square::from(move_item.to_pos);

                let rank = from.rank;
                let file = to.file;

                let captured_square = Square::index(rank, file);

                self.bitboards.place_piece(
                    self.side_to_move.opponent(),
                    unmake_metadata.captured_piece,
                    captured_square.into(),
                    &mut self.phase,
                    &mut self.opening,
                    &mut self.endgame,
                    &mut self.hash,
                    keys,
                );
            }
        }

        if move_item.castling {
            match (self.side_to_move, move_item.to_pos) {
                (Player::White, 2) => {
                    self.bitboards.place_piece(
                        self.side_to_move,
                        Piece::Rook,
                        0,
                        &mut self.phase,
                        &mut self.opening,
                        &mut self.endgame,
                        &mut self.hash,
                        keys,
                    );
                    self.bitboards.remove_piece(
                        self.side_to_move,
                        3,
                        &mut self.phase,
                        &mut self.opening,
                        &mut self.endgame,
                        &mut self.hash,
                        keys,
                    );
                }
                (Player::White, 6) => {
                    self.bitboards.place_piece(
                        self.side_to_move,
                        Piece::Rook,
                        7,
                        &mut self.phase,
                        &mut self.opening,
                        &mut self.endgame,
                        &mut self.hash,
                        keys,
                    );
                    self.bitboards.remove_piece(
                        self.side_to_move,
                        5,
                        &mut self.phase,
                        &mut self.opening,
                        &mut self.endgame,
                        &mut self.hash,
                        keys,
                    );
                }
                (Player::Black, 58) => {
                    self.bitboards.place_piece(
                        self.side_to_move,
                        Piece::Rook,
                        56,
                        &mut self.phase,
                        &mut self.opening,
                        &mut self.endgame,
                        &mut self.hash,
                        keys,
                    );
                    self.bitboards.remove_piece(
                        self.side_to_move,
                        59,
                        &mut self.phase,
                        &mut self.opening,
                        &mut self.endgame,
                        &mut self.hash,
                        keys,
                    );
                }
                (Player::Black, 62) => {
                    self.bitboards.place_piece(
                        self.side_to_move,
                        Piece::Rook,
                        63,
                        &mut self.phase,
                        &mut self.opening,
                        &mut self.endgame,
                        &mut self.hash,
                        keys,
                    );
                    self.bitboards.remove_piece(
                        self.side_to_move,
                        61,
                        &mut self.phase,
                        &mut self.opening,
                        &mut self.endgame,
                        &mut self.hash,
                        keys,
                    );
                }
                (_, _) => {
                    println!("{:?} {}", self.side_to_move, move_item.to_pos);
                }
            }
        }

        self.hash ^= keys.castling[self.castle_permissions as usize];
        let hash_enpassant_sq = std::cmp::min(self.enpassant_square.trailing_zeros(), 63) as usize;
        self.hash ^= keys.enpassant[hash_enpassant_sq as usize];
        self.hash ^= keys.side_to_move[self.side_to_move as usize];
    }

    pub fn make_null_move(&mut self, keys: &ZobristRandomKeys) -> u64 {
        self.hash ^= keys.side_to_move[self.side_to_move as usize]; // remove prev side from hash
        let hash_enpassant_sq = std::cmp::min(self.enpassant_square.trailing_zeros(), 63) as usize;
        self.hash ^= keys.enpassant[hash_enpassant_sq as usize]; // remove prev side from hash

        self.side_to_move = self.side_to_move.opponent();
        let enpassant = self.enpassant_square;
        self.enpassant_square = 0;

        self.hash ^= keys.side_to_move[self.side_to_move as usize]; // add new side to hash
        let hash_enpassant_sq = std::cmp::min(self.enpassant_square.trailing_zeros(), 63) as usize;
        self.hash ^= keys.enpassant[hash_enpassant_sq as usize]; // remove prev side from hash

        return enpassant;
    }
    pub fn unmake_null_move(&mut self, enpassant: u64, keys: &ZobristRandomKeys) -> u64 {
        self.hash ^= keys.side_to_move[self.side_to_move as usize]; // remove prev side from hash
        let hash_enpassant_sq = std::cmp::min(self.enpassant_square.trailing_zeros(), 63) as usize;
        self.hash ^= keys.enpassant[hash_enpassant_sq as usize]; // remove prev side from hash
        
        self.side_to_move = self.side_to_move.opponent();
        self.enpassant_square = enpassant;
        
        self.hash ^= keys.side_to_move[self.side_to_move as usize]; // add new side to hash
        let hash_enpassant_sq = std::cmp::min(self.enpassant_square.trailing_zeros(), 63) as usize;
        self.hash ^= keys.enpassant[hash_enpassant_sq as usize]; // remove prev side from hash

        return enpassant;
    }

    #[allow(dead_code)]
    pub fn new(key: &ZobristRandomKeys) -> GameState {
        let start_board_fen =
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        return Self::from_fen(start_board_fen, key).unwrap();
    }

    pub fn from_fen(fen: String, keys: &ZobristRandomKeys) -> Result<GameState, String> {
        let parts: Vec<&str> = fen.split(" ").collect();

        if parts.len() != 6 {
            return Err("Fen missing parts".to_string());
        }

        let mut game = GameState {
            bitboards: fen::parse::parse_fen_board(parts[0], keys).unwrap(),
            side_to_move: fen::parse::parse_fen_side(parts[1]).unwrap(),
            castle_permissions: fen::parse::parse_fen_castle(parts[2]).unwrap(),
            enpassant_square: fen::parse::parse_fen_enpassant(parts[3]).unwrap(),
            half_move_clock: parts[4].parse::<u32>().unwrap(),
            full_move_number: parts[5].parse::<u32>().unwrap(),
            // temp values
            phase: 0,
            opening_pqst_score: 0,
            endgame_pqst_score: 0,
            color: 0,
            opening: [0, 0],
            endgame: [0, 0],
            hash: 0,
        };

        let (phase, opening, endgame) = crate::evaluation::psqt_tapered::init(&game);
        game.phase = phase;
        game.opening = opening;
        game.endgame = endgame;
        game.opening_pqst_score = opening[0] - opening[1];
        game.endgame_pqst_score = endgame[0] - endgame[1];
        game.color = if game.side_to_move == Player::White {
            1
        } else {
            -1
        };

        game.hash = calculate_zobrist_hash(&game, keys);

        // need to make this cleaner
        return Ok(game);
    }

    pub fn to_fen(&self) -> String {
        let board = fen::stringify::stringify_board(self);
        let side = fen::stringify::stringify_side(self);
        let castling = fen::stringify::stringify_castling(self);
        let enpassant = fen::stringify::stringify_enpassant(self);
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

                let piece = self.bitboards.pos_to_piece[pos];
                let colored = if self.bitboards.pos_to_player[Player::White as usize].get(pos as i8)
                {
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
        println!();
        println!("fen: {}", self.to_fen());
    }

    pub fn print_state(&self) {
        self.print_board();
    }
}
