use enum_map::Enum;

use super::player::Player;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    Pawn(Player),
    Knight(Player),
    Bishop(Player),
    Rook(Player),
    Queen(Player),
    King(Player),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Piece {
    Empty = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook= 4,
    Queen= 5,
    King = 6,
}

// impl Piece {
//     fn get_type(&self) -> Option<&Player> {
//         match self {
//             Piece::Empty => None,
//             Piece::Pawn(t) => Some(t),
//             Piece::Knight(t) => Some(t),
//             Piece::Bishop(t) => Some(t),
//             Piece::Rook(t) => Some(t),
//             Piece::Queen(t) => Some(t),
//             Piece::King(t) => Some(t),
//         }
//     }
// }

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self {
            Piece::Empty => ".".into(),
            Piece::Pawn => "p".into(),
            Piece::Knight => "n".into(),
            Piece::Bishop => "b".into(),
            Piece::Rook => "r".into(),
            Piece::Queen => "q".into(),
            Piece::King => "k".into(),
        }
    }
}
