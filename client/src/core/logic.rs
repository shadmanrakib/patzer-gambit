use bevy::prelude::*;

pub enum Piece {
    Empty,
    Pawn(PlayerType),
    Knight(PlayerType),
    Bishop(PlayerType),
    Rook(PlayerType),
    Queen(PlayerType),
    King(PlayerType),
}

struct Board {
    pieces: Vec<BoardPiece>,
    captured_pieces: Vec<BoardPiece>,
    fen_notation: String,

}

#[derive(Component)]
pub struct BoardPiece {
    pub piece: Piece,
    pub x_pos: u8,
    pub y_pos: u8,
    pub player: PlayerType,
}


pub enum PlayerType  {
    White,
    Black,
}


