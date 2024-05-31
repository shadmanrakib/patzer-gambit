use crate::{pieces::Piece, square::Square};

// rough draft as i figure out how i want to structure things
// we will use more than needed initially

#[derive(Debug, Clone)]
pub struct MoveItem {
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

impl MoveItem {
    /*
        <move descriptor> ::= <from square><to square>[<promoted to>]
        <square>        ::= <file letter><rank number>
        <file letter>   ::= 'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'
        <rank number>   ::= '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'
        <promoted to>   ::= 'q'|'r'|'b'|'n'
    */
    pub const NULL: MoveItem = MoveItem {
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
    pub fn notation(&self) -> String {
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
    // pub prev_zorbist_hash
    pub captured_piece: Piece,
}