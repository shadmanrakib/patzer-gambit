#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    White,
    Black,
}

impl Player {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Player::White => "w".into(),
            Player::Black => "b".into(),
        }
    }
    pub fn opponent(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}