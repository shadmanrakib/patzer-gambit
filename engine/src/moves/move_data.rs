use super::super::state::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Move {
    pub from_rank: i8,
    pub from_file: i8,
    pub to_rank: i8,
    pub to_file: i8,
    pub promotion: Piece,
}