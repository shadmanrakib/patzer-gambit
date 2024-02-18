use enum_map::Enum;

use super::player::Player;

#[derive(PartialEq, Debug, Clone, Copy, Enum)]
pub enum Piece {
    Empty,
    Pawn(Player),
    Knight(Player),
    Bishop(Player),
    Rook(Player),
    Queen(Player),
    King(Player),
}

impl Piece {
    fn get_type(&self) -> Option<&Player> {
        match self {
            Piece::Empty => None,
            Piece::Pawn(t) => Some(t),
            Piece::Knight(t) => Some(t),
            Piece::Bishop(t) => Some(t),
            Piece::Rook(t) => Some(t),
            Piece::Queen(t) => Some(t),
            Piece::King(t) => Some(t),
        }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self {
            Piece::Empty => ".".into(),
            Piece::Pawn(Player::Black) => "p".into(),
            Piece::Knight(Player::Black) => "n".into(),
            Piece::Bishop(Player::Black) => "b".into(),
            Piece::Rook(Player::Black) => "r".into(),
            Piece::Queen(Player::Black) => "q".into(),
            Piece::King(Player::Black) => "k".into(),
            Piece::Pawn(Player::White) => "P".into(),
            Piece::Knight(Player::White) => "N".into(),
            Piece::Bishop(Player::White) => "B".into(),
            Piece::Rook(Player::White) => "R".into(),
            Piece::Queen(Player::White) => "Q".into(),
            Piece::King(Player::White) => "K".into(),
        }
    }
}
