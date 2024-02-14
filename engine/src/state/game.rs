use std::fs::Permissions;

use regex::Regex;
use super::{bitboards::BitBoards, fen, pieces::Piece, player::Player, square::Square};

#[derive(Default, Debug)]
pub struct CastlePermissions {
    pub white_queen_side: bool,
    pub white_king_side: bool,
    pub black_queen_side: bool,
    pub black_king_side: bool,
}

#[derive(Debug)]
pub struct EnpassantSquare {
    pub exists: bool,
    pub pos: Square,
}

// inspired by FEN notation
#[derive(Debug)]
pub struct GameState {
    pub bitboards: BitBoards,
    pub side_to_move: Player,
    pub castle_permissions: CastlePermissions,
    // 0-7 maps to columns A-H, 8 is none
    pub enpassant_square: EnpassantSquare,
    // It marks the number of moves since the last pawn push or piece capture.
    pub half_move_clock: u32,
    // It marks the number of full moves. It starts at 1 and is incremented after black's move.
    pub full_move_clock: u32,
}

impl GameState {
    pub fn new() -> GameState {
        let start_board_fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        return Self::from_fen(start_board_fen).unwrap();
    }

    pub fn from_fen(fen: String) -> Result<GameState, String> {
        let parts: Vec<&str> = fen.split(" ").collect();

        if parts.len() != 6 {
            return Err("Fen missing parts".to_string());
        }

        // need to make this cleaner
        return Ok(GameState {
            bitboards: fen::parse_fen_board(parts[0]).unwrap(),
            side_to_move: fen::parse_fen_side(parts[1]).unwrap(),
            castle_permissions: fen::parse_fen_castle(parts[2]).unwrap(),
            enpassant_square: fen::parse_fen_enpassant(parts[3]).unwrap(),
            half_move_clock: parts[4].parse::<u32>().unwrap(),
            full_move_clock: parts[5].parse::<u32>().unwrap(),
        });
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
    }
    
    pub fn print_state(&self) {
        self.print_board();
        println!("Player to move: {}", self.side_to_move.to_string());
        println!("Castling: {:?}", self.castle_permissions);
        println!("Enpassant: {:?}", self.enpassant_square);
        println!("Half move: {} \t Full move: {}", self.half_move_clock, self.full_move_clock);

    }
}
