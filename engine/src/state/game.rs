use regex::Regex;

use super::{boards::Boards, pieces::Piece, player::Player, square::Square};
use crate::{
    fen,
    moves::move_data::{MoveItem, SimpleMoveItem, UnmakeMoveMetadata},
};

#[derive(Default, Debug, Clone)]
pub struct CastlePermissions {
    pub white_queen_side: bool,
    pub white_king_side: bool,
    pub black_queen_side: bool,
    pub black_king_side: bool,
}

impl CastlePermissions {
    pub fn revoke_white(&mut self) {
        self.white_queen_side = false;
        self.white_king_side = false;
    }
    pub fn revoke_black(&mut self) {
        self.black_queen_side = false;
        self.black_king_side = false;
    }
    pub fn none() -> CastlePermissions {
        CastlePermissions {
            white_queen_side: false,
            white_king_side: false,
            black_queen_side: false,
            black_king_side: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnpassantSquare {
    pub exists: bool,
    pub pos: Square,
}

// inspired by FEN notation
#[derive(Debug, Clone)]
pub struct GameState {
    pub bitboards: Boards,
    pub side_to_move: Player,
    pub castle_permissions: CastlePermissions,
    // 0-7 maps to columns A-H, 8 is none
    pub enpassant_square: EnpassantSquare,
    // It marks the number of moves since the last pawn push or piece capture.
    pub half_move_clock: u32,
    // It marks the number of full moves. It starts at 1 and is incremented after black's move.
    pub full_move_number: u32,
}

impl GameState {
    pub fn set(&mut self, other: GameState) {
        self.bitboards = other.bitboards;
        self.side_to_move = other.side_to_move;
        self.castle_permissions = other.castle_permissions;
        self.enpassant_square = other.enpassant_square;
        self.half_move_clock = other.half_move_clock;
        self.full_move_number = other.full_move_number;
    }
    // TODO: will implement later
    pub fn notation_to_simple_move(&self, notation: String) -> Result<SimpleMoveItem, String> {
        let re = Regex::new(r"([a-h][1-8])([a-h][1-8])([nbrq])?").unwrap();

        if let Some(captures) = re.captures(&notation) {
            println!("{:?}", captures);
            if let (Some(from_match), Some(to_match), promotion_match) =
                (captures.get(1), captures.get(2), captures.get(3))
            {
                let from = Square::parse_string(from_match.as_str().into()).unwrap();
                let to = Square::parse_string(to_match.as_str().into()).unwrap();
                let promotion_piece = promotion_match.map_or(Piece::Empty, |promotion| {
                    match promotion.as_str() {
                        "n" => Piece::Knight(self.side_to_move),
                        "b" => Piece::Bishop(self.side_to_move),
                        "r" => Piece::Rook(self.side_to_move),
                        "q" => Piece::Queen(self.side_to_move),
                        _ => unreachable!(), // shouldnt happen,
                    }
                });

                let piece = self.bitboards.get_piece(from.rank, from.file);
                let capturing_piece = self.bitboards.get_piece(to.rank, to.file);

                // we need to consider if this is a valid pattern
                // we need to consider if we are going to castle
                // we need to consider enpassant
                // we need to consider whether this is double move
                // we need to consider whether we are capturing our own piece
                // or the opponents

                return Ok(SimpleMoveItem {
                    from,
                    to,
                    promotion_piece,
                });
            }
        }

        return Err("Invalid long algrebraic notation.".into());
        // MoveItem {
        //     from_pos: (),
        //     to_pos: (),
        //     piece: (),
        //     promotion_piece: (),
        //     captured_piece: (),
        //     promoting: (),
        //     capturing: (),
        //     double: (),
        //     enpassant: (),
        //     castling: (),
        //     score: (),
        // };
    }
    pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
        let prev_castle_permissions = self.castle_permissions.clone();
        let prev_enpassant_square = self.enpassant_square.clone();
        let prev_half_move_clock = self.half_move_clock;

        // ==============================================

        // lets now make the move
        // all other moves get handled
        let final_piece = if move_item.promoting {
            move_item.promotion_piece.clone()
        } else {
            move_item.piece.clone()
        };

        self.bitboards.unset_by_bit_pos(move_item.from_pos);
        self.bitboards
            .set_or_replace_piece_by_bit_pos(final_piece, move_item.to_pos);

        // handle pawn left from enpassant capture
        if move_item.enpassant {
            let from = Square::from(move_item.from_pos);
            let to = Square::from(move_item.to_pos);

            let rank = from.rank;
            let file = to.file;

            let leftover_square = Square { rank, file };

            self.bitboards.unset_by_bit_pos(leftover_square.into());
        }

        if move_item.castling {
            // move rook to place
            match (self.side_to_move, move_item.to_pos) {
                (Player::White, 2) => {
                    self.bitboards.unset_by_bit_pos(0);
                    self.bitboards
                        .set_or_replace_piece_by_bit_pos(Piece::Rook(self.side_to_move), 3);
                }
                (Player::White, 6) => {
                    self.bitboards.unset_by_bit_pos(7);
                    self.bitboards
                        .set_or_replace_piece_by_bit_pos(Piece::Rook(self.side_to_move), 5);
                }
                (Player::Black, 58) => {
                    self.bitboards.unset_by_bit_pos(56);
                    self.bitboards
                        .set_or_replace_piece_by_bit_pos(Piece::Rook(self.side_to_move), 59);
                }
                (Player::Black, 62) => {
                    self.bitboards.unset_by_bit_pos(63);
                    self.bitboards
                        .set_or_replace_piece_by_bit_pos(Piece::Rook(self.side_to_move), 61);
                }
                (_, _) => {
                    // TODO: handle error
                }
            }
        }

        // ==============================================

        // half move clock needs to be incremented if no capture, castle, or pawn move
        // to enforce draw by 50 moves rule, else set to 0 to reset
        if !move_item.capturing
            && !move_item.castling
            && move_item.piece != Piece::Pawn(self.side_to_move)
        {
            self.half_move_clock += 1;
        } else {
            self.half_move_clock = 0;
        }

        // we want to create new enpassant square if needed, or revoke old
        if move_item.double {
            let Square {
                rank: to_rank,
                file,
            } = Square::from(move_item.to_pos);
            let Square {
                rank: from_rank,
                file: _,
            } = Square::from(move_item.from_pos);
            let enpassant_rank: i8 = {
                if to_rank > from_rank {
                    to_rank - 1
                } else {
                    to_rank + 1
                }
            };
            self.enpassant_square = EnpassantSquare {
                exists: true,
                pos: Square {
                    rank: enpassant_rank,
                    file,
                },
            }
        } else {
            // no enpassant square
            self.enpassant_square = EnpassantSquare {
                exists: false,
                pos: Square { rank: 8, file: 8 },
            }
        }

        // full move number needs to be incremented if side to play is black
        if self.side_to_move == Player::Black {
            self.full_move_number += 1;
        }

        // revoke necessary castling permissions
        // || move_item.piece == Piece::King(self.side_to_move)
        if move_item.castling || move_item.piece == Piece::King(self.side_to_move) {
            match self.side_to_move {
                Player::White => {
                    self.castle_permissions.revoke_white();
                }
                Player::Black => {
                    self.castle_permissions.revoke_black();
                }
            }
        }
        if move_item.piece == Piece::Rook(self.side_to_move) {
            match (self.side_to_move, move_item.from_pos) {
                (Player::White, 0) => {
                    self.castle_permissions.white_queen_side = false;
                }
                (Player::White, 7) => {
                    self.castle_permissions.white_king_side = false;
                }
                (Player::Black, 56) => {
                    self.castle_permissions.black_queen_side = false;
                }
                (Player::Black, 63) => {
                    self.castle_permissions.black_king_side = false;
                }
                (_, _) => {}
            }
        }
        if move_item.captured_piece == Piece::Rook(self.side_to_move.opponent()) {
            match (self.side_to_move.opponent(), move_item.to_pos) {
                (Player::White, 0) => {
                    self.castle_permissions.white_queen_side = false;
                }
                (Player::White, 7) => {
                    self.castle_permissions.white_king_side = false;
                }
                (Player::Black, 56) => {
                    self.castle_permissions.black_queen_side = false;
                }
                (Player::Black, 63) => {
                    self.castle_permissions.black_king_side = false;
                }
                (_, _) => {}
            }
        }

        // side to play needs to change to opposite
        self.side_to_move = self.side_to_move.opponent();

        UnmakeMoveMetadata {
            captured_piece: move_item.captured_piece.clone(),
            prev_castle_permissions,
            prev_enpassant_square,
            prev_half_move_clock,
        }
    }

    pub fn unmake_move(&mut self, move_item: MoveItem, unmake_metadata: UnmakeMoveMetadata) {}

    pub fn new() -> GameState {
        let start_board_fen =
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        return Self::from_fen(start_board_fen).unwrap();
    }

    pub fn from_fen(fen: String) -> Result<GameState, String> {
        let parts: Vec<&str> = fen.split(" ").collect();

        if parts.len() != 6 {
            return Err("Fen missing parts".to_string());
        }

        // need to make this cleaner
        return Ok(GameState {
            bitboards: fen::parse::parse_fen_board(parts[0]).unwrap(),
            side_to_move: fen::parse::parse_fen_side(parts[1]).unwrap(),
            castle_permissions: fen::parse::parse_fen_castle(parts[2]).unwrap(),
            enpassant_square: fen::parse::parse_fen_enpassant(parts[3]).unwrap(),
            half_move_clock: parts[4].parse::<u32>().unwrap(),
            full_move_number: parts[5].parse::<u32>().unwrap(),
        });
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
        for rank in (0..8).rev() {
            print!("{} | ", rank + 1);
            for file in 0..8 {
                let pos = 8 * rank + file;
                print!("{} ", self.bitboards.get_piece_by_bit_pos(pos).to_string());
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
        println!("Player to move: {}", self.side_to_move.to_string());
        println!("Castling: {:?}", self.castle_permissions);
        println!("Enpassant: {:?}", self.enpassant_square);
        println!(
            "Half move: {} \t Full move: {}",
            self.half_move_clock, self.full_move_number
        );
    }
}
