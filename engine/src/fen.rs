use regex::Regex;

use crate::{
    state::{
        boards::{BitBoard, Boards}, game::{CastlePermissions, GameState}, pieces::Piece, player::Player, square::Square,
    }, zobrist::ZobristHasher
};

pub fn parse_fen_board(part: &str, keys: &ZobristHasher) -> Result<Boards, String> {
    let mut bitboards: Boards = Default::default();
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
                    bitboards.place_piece(
                        Player::Black,
                        Piece::Pawn,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'r' => {
                    bitboards.place_piece(
                        Player::Black,
                        Piece::Rook,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'n' => {
                    bitboards.place_piece(
                        Player::Black,
                        Piece::Knight,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'b' => {
                    bitboards.place_piece(
                        Player::Black,
                        Piece::Bishop,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'q' => {
                    bitboards.place_piece(
                        Player::Black,
                        Piece::Queen,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'k' => {
                    bitboards.place_piece(
                        Player::Black,
                        Piece::King,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'P' => {
                    bitboards.place_piece(
                        Player::White,
                        Piece::Pawn,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'R' => {
                    bitboards.place_piece(
                        Player::White,
                        Piece::Rook,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'N' => {
                    bitboards.place_piece(
                        Player::White,
                        Piece::Knight,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'B' => {
                    bitboards.place_piece(
                        Player::White,
                        Piece::Bishop,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'Q' => {
                    bitboards.place_piece(
                        Player::White,
                        Piece::Queen,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
                    pos += 1;
                }
                'K' => {
                    bitboards.place_piece(
                        Player::White,
                        Piece::King,
                        pos,
                        &mut 0,
                        &mut [0, 0],
                        &mut [0, 0],
                        &mut 0,
                        keys,
                    );
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

pub fn parse_fen_enpassant(part: &str) -> Result<u64, String> {
    let re = Regex::new(r"^[a-h][1-8]$") .unwrap();
    match part {
        "-" => Ok(0),
        part3 if re.is_match(part3) => {
            if let Ok(square) = Square::parse_string(part3.into()) {
                return Ok(1 << <Square as Into<i8>>::into(square));
            }

            return Err("Invalid enpassant square".into());
        }
        _ => Err("Invalid enpassant square".into()),
    }
}
pub fn parse_fen_castle(part: &str) -> Result<u8, String> {
    let mut permission = 0;

    if part == "-" {
        return Ok(permission);
    }

    for c in part.chars() {
        if c == 'K' {
            permission.grant(u8::WHITE_KING_SIDE);
        } else if c == 'k' {
            permission.grant(u8::BLACK_KING_SIDE);
        } else if c == 'Q' {
            permission.grant(u8::WHITE_QUEEN_SIDE);
        } else if c == 'q' {
            permission.grant(u8::BLACK_QUEEN_SIDE);
        } else {
            return Err("Invalid character in castle permission".to_string());
        }
    }

    return Ok(permission);
}

pub fn stringify_board(game: &GameState) -> String {
    let mut stringified = String::new();
    for rank in (0..8).rev() {
        let mut contingious_empty = 0;
        for file in 0..8 {
            let pos: i8 = rank * 8 + file;
            let piece = game.bitboards.pos_to_piece[pos as usize];

            match piece {
                Piece::Empty => {
                    contingious_empty += 1;
                }
                _ => {
                    let p = piece.to_string();
                    let colored =
                        if game.bitboards.pos_to_player[Player::White as usize].get(pos as i8) {
                            p.to_uppercase()
                        } else {
                            p
                        };

                    if contingious_empty > 0 {
                        stringified += &contingious_empty.to_string();
                        contingious_empty = 0;
                    }

                    stringified += &colored.to_string();
                }
            }
        }

        if contingious_empty > 0 {
            stringified += &contingious_empty.to_string();
        }

        if rank != 0 {
            stringified += "/";
        }
    }

    return stringified;
}

// #[inline(always)]
pub fn stringify_side(game: &GameState) -> String {
    match game.side_to_move {
        Player::White => "w".into(),
        Player::Black => "b".into(),
    }
}

// #[inline(always)]
pub fn stringify_enpassant(game: &GameState) -> String {
    if game.enpassant_square == 0 {
        return "-".into();
    }
    let (_, pos) = game.enpassant_square.pop();
    return Square::from(pos).stringify();
}
pub fn stringify_castling(game: &GameState) -> String {
    let mut stringified = String::from("");
    if game.castle_permissions.is_allowed(u8::WHITE_KING_SIDE) {
        stringified += "K";
    }
    if game.castle_permissions.is_allowed(u8::WHITE_QUEEN_SIDE) {
        stringified += "Q";
    }
    if game.castle_permissions.is_allowed(u8::BLACK_KING_SIDE) {
        stringified += "k";
    }
    if game.castle_permissions.is_allowed(u8::BLACK_QUEEN_SIDE) {
        stringified += "q";
    }

    if stringified == "" {
        return "-".into();
    }

    return stringified;
}