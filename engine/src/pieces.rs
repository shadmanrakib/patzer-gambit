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
