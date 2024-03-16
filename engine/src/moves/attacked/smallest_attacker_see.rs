use crate::{
    moves::{
        move_data::MoveItem,
        precalculate::{cache::PrecalculatedCache, magic_bitboards::hash_with_magic},
        pseudolegal::queen,
    },
    state::{boards::BitBoard, game::GameState, pieces::Piece, player::Player, square::Square},
};

pub fn smallest_attacker_for_see(
    pos: i8,
    attacker: Player,
    game: &GameState,
    cache: &PrecalculatedCache,
) -> Option<MoveItem> {
    let square_occupied = game.bitboards.occupied.get(pos);
    let occupied = game.bitboards.occupied;

    let (rank, file) = Square::rank_and_file(pos);

    // regular pawn attack
    let opponent_pawns = game.bitboards.get_board_by_piece(attacker, Piece::Pawn);
    let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
    let mut attacking_pawns = opponent_pawns & attacking_mask;
    if attacking_pawns != 0 {
        // if promotion possible, just assume queen so we can return to pos square.
        // enpassant is not possible in static exchange response, so not handled
        return Some(MoveItem {
            from_pos: attacking_pawns.pop_mut(),
            to_pos: pos,
            piece: Piece::Pawn,
            promotion_piece: if rank == 0 || rank == 7 {
                Piece::Queen
            } else {
                Piece::Empty
            },
            captured_piece: game.bitboards.pos_to_piece[pos as usize],
            promoting: rank == 0 || rank == 7,
            capturing: square_occupied,
            double: false,
            enpassant: false,
            castling: false,
            score: 0,
        });
    }

    // knight
    let knight_move_mask = cache.knight_moves_masks[pos as usize];
    let mut attacking_knights =
        knight_move_mask & game.bitboards.get_board_by_piece(attacker, Piece::Knight);
    if attacking_knights != 0 {
        return Some(MoveItem {
            from_pos: attacking_knights.pop_mut(),
            to_pos: pos,
            piece: Piece::Knight,
            promotion_piece: Piece::Empty,
            captured_piece: game.bitboards.pos_to_piece[pos as usize],
            promoting: false,
            capturing: square_occupied,
            double: false,
            enpassant: false,
            castling: false,
            score: 0,
        });
    }

    let mut attacking_queens = 0;

    // bishop and queen diagonal
    let bishop_magic_index = hash_with_magic(cache.bishop_magics[pos as usize], occupied);
    let bishop_moves_mask = cache.bishop_magic_attack_tables[bishop_magic_index];
    let mut attacking_bishops =
        bishop_moves_mask & game.bitboards.get_board_by_piece(attacker, Piece::Bishop);

    attacking_queens |=
        bishop_moves_mask & game.bitboards.get_board_by_piece(attacker, Piece::Queen);

    if attacking_bishops != 0 {
        return Some(MoveItem {
            from_pos: attacking_bishops.pop_mut(),
            to_pos: pos,
            piece: Piece::Bishop,
            promotion_piece: Piece::Empty,
            captured_piece: game.bitboards.pos_to_piece[pos as usize],
            promoting: false,
            capturing: square_occupied,
            double: false,
            enpassant: false,
            castling: false,
            score: 0,
        });
    }

    // rook and queen vertical and horizontal
    let rook_magic_index = hash_with_magic(cache.rook_magics[pos as usize], occupied);
    let rook_moves_mask = cache.rook_magic_attack_tables[rook_magic_index];
    let mut attacking_rooks =
        rook_moves_mask & game.bitboards.get_board_by_piece(attacker, Piece::Rook);
    attacking_queens |= rook_moves_mask & game.bitboards.get_board_by_piece(attacker, Piece::Queen);

    if attacking_rooks != 0 {
        return Some(MoveItem {
            from_pos: attacking_rooks.pop_mut(),
            to_pos: pos,
            piece: Piece::Rook,
            promotion_piece: Piece::Empty,
            captured_piece: game.bitboards.pos_to_piece[pos as usize],
            promoting: false,
            capturing: square_occupied,
            double: false,
            enpassant: false,
            castling: false,
            score: 0,
        });
    }

    if attacking_queens != 0 {
        return Some(MoveItem {
            from_pos: attacking_queens.pop_mut(),
            to_pos: pos,
            piece: Piece::Queen,
            promotion_piece: Piece::Empty,
            captured_piece: game.bitboards.pos_to_piece[pos as usize],
            promoting: false,
            capturing: square_occupied,
            double: false,
            enpassant: false,
            castling: false,
            score: 0,
        });
    }

    // king attack
    let king_move_mask = cache.king_moves_masks[pos as usize];
    let mut attacking_king =
        king_move_mask & game.bitboards.get_board_by_piece(attacker, Piece::King);

    if attacking_king != 0 {
        return Some(MoveItem {
            from_pos: attacking_king.pop_mut(),
            to_pos: pos,
            piece: Piece::King,
            promotion_piece: Piece::Empty,
            captured_piece: game.bitboards.pos_to_piece[pos as usize],
            promoting: false,
            capturing: square_occupied,
            double: false,
            enpassant: false,
            castling: false,
            score: 0,
        });
    }

    return None;
}
