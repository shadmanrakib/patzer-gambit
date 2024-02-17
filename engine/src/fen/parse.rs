use regex::Regex;

use crate::state::{bitboards::BitBoards, game::{CastlePermissions, EnpassantSquare}, pieces::Piece, player::Player, square::Square};

pub fn parse_fen_board(part: &str) -> Result<BitBoards, String> {
    let mut bitboards: BitBoards = Default::default();
    // part 1 parsing
    let splitted: Vec<&str> = part.split("/").collect();
    if splitted.len() != 8 {
        return Err("Part 1 of FEN is invalid length".to_string());
    }
    for rank in (0..8).rev() {
        let rank_str = splitted[7 - rank];
        let mut pos: i8 = (rank * 8).try_into().unwrap();
        for c in rank_str.chars() {
            match c {
                'p' => {
                    bitboards.set_piece_by_bit_pos(Piece::Pawn(Player::Black), pos);
                    pos += 1;
                }
                'r' => {
                    bitboards.set_piece_by_bit_pos(Piece::Rook(Player::Black), pos);
                    pos += 1;
                }
                'n' => {
                    bitboards.set_piece_by_bit_pos(Piece::Knight(Player::Black), pos);
                    pos += 1;
                }
                'b' => {
                    bitboards.set_piece_by_bit_pos(Piece::Bishop(Player::Black), pos);
                    pos += 1;
                }
                'q' => {
                    bitboards.set_piece_by_bit_pos(Piece::Queen(Player::Black), pos);
                    pos += 1;
                }
                'k' => {
                    bitboards.set_piece_by_bit_pos(Piece::King(Player::Black), pos);
                    pos += 1;
                }
                'P' => {
                    bitboards.set_piece_by_bit_pos(Piece::Pawn(Player::White), pos);
                    pos += 1;
                }
                'R' => {
                    bitboards.set_piece_by_bit_pos(Piece::Rook(Player::White), pos);
                    pos += 1;
                }
                'N' => {
                    bitboards.set_piece_by_bit_pos(Piece::Knight(Player::White), pos);
                    pos += 1;
                }
                'B' => {
                    bitboards.set_piece_by_bit_pos(Piece::Bishop(Player::White), pos);
                    pos += 1;
                }
                'Q' => {
                    bitboards.set_piece_by_bit_pos(Piece::Queen(Player::White), pos);
                    pos += 1;
                }
                'K' => {
                    bitboards.set_piece_by_bit_pos(Piece::King(Player::White), pos);
                    pos += 1;
                }
                '0'..='8' => {
                    pos += <u32 as TryInto<i8>>::try_into(c.to_digit(10).unwrap()).unwrap();
                }
                _ => return Err(format!("Invalid character in board represenation {c}")),
            };
        }
    }

    return Ok(bitboards);
}
pub fn parse_fen_side(part: &str) -> Result<Player, String> {
    match part {
        "w" => Ok(Player::White),
        "b" => Ok(Player::Black),
        _ => Err("Invalid player character".into()),
    }
}
pub fn parse_fen_enpassant(part: &str) -> Result<EnpassantSquare, String> {
    let re = Regex::new(r"^[a-h][1-8]$").unwrap();
    match part {
        "-" => Ok(EnpassantSquare{exists: false, pos: Square{rank: 8,file: 8}}),
        part3 if re.is_match(part3) => {
            let rank = (part3.chars().nth(1).unwrap() as u32 - '1' as u32) as i8;
            let file = (part3.chars().nth(0).unwrap() as u32 - 'a' as u32) as i8;
            Ok(EnpassantSquare{exists: true, pos: Square{rank, file}})
        }
        _ => Err("Invalid player character".into()),
    }
}
pub fn parse_fen_castle(part: &str) -> Result<CastlePermissions, String> {
    let mut permission = CastlePermissions{
        white_king_side: false,
        white_queen_side: false,
        black_king_side: false,
        black_queen_side: false,
    };
    
    if part == "-" {
        return Ok(permission);
    }

    for c in part.chars() {
        if c == 'K' {
            permission.white_king_side = true;
        }
        else if c == 'k' {
            permission.black_king_side = true;
        }
        else if c == 'Q' {
            permission.white_queen_side = true;
        }
        else if c == 'q' {
            permission.black_queen_side = true;
        } else {
            return Err("Invalid character in castle permission".to_string())
        }
    }

    return Ok(permission)
}