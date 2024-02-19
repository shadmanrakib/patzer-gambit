use crate::state::{game::GameState, pieces::Piece, player::Player};

// #[inline(always)]
pub fn stringify_board(game: &GameState) -> String {
    let mut stringified = String::new();

    for rank in (0..8).rev() {
        let mut contingious_empty = 0;
        for file in 0..8 {
            let pos: i8 = rank * 8 + file;
            let piece = game.bitboards.get_piece_by_bit_pos(pos);

            match piece {
                Piece::Empty => {
                    contingious_empty += 1;
                },
                _ => {
                    if contingious_empty > 0 {
                        stringified += &contingious_empty.to_string();
                        contingious_empty = 0;
                    }

                    stringified += &piece.to_string();
                },
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
    if !game.enpassant_square.exists {
        return "-".into();
    }
    return game.enpassant_square.pos.stringify();
}
pub fn stringify_castling(game: &GameState) -> String {
    let mut stringified = String::from("");
    if game.castle_permissions.white_king_side {
        stringified += "K";
    }
    if game.castle_permissions.white_queen_side {
        stringified += "Q";
    }
    if game.castle_permissions.black_king_side {
        stringified += "k";
    }
    if game.castle_permissions.black_queen_side {
        stringified += "q";
    }

    if stringified == "" {
        return "-".into();
    }

    return stringified;
}