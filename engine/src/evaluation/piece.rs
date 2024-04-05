#[allow(dead_code)]
pub const STATIC_PIECE_POINTS: [i32; 7] = [
    0,     // Empty
    100,   // Pawn
    300,   // Knight
    325,   // Bishop
    500,   // Rook
    925,   // Queen
    100000, // King
];

pub const OPENING_PIECE_POINTS: [i32; 7] = [
    0,     // Empty
    80,   // Pawn
    320,   // Knight
    330,   // Bishop
    500,   // Rook
    900,   // Queen
    1000000, // King
];

pub const ENDGAME_PIECE_POINTS: [i32; 7] = [
    0,     // Empty
    130,   // Pawn
    300,   // Knight
    340,   // Bishop
    490,   // Rook
    1000,   // Queen
    1000000, // King
];
