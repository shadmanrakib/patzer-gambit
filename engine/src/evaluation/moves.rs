use std::mem::MaybeUninit;

use super::piece::STATIC_PIECE_POINTS;
use crate::{
    constants::search::MAX_KILLER_MOVES,
    moves::{
        attacked::smallest_attacker_see::smallest_attacker_for_see,
        move_data::{MoveItem, UnmakeMoveMetadata},
        precalculate::cache::PrecalculatedCache,
    },
    search::{cache::SearchCache, killer::is_similar},
    state::{
        game::GameState,
        movelist::MoveList,
        player::{self, Player},
    },
};

// MVV_VLA[attacker][victem]
pub const MVV_LVA_SCORE: [[i16; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0], // attacker None, victim None, P, N, B, R, Q, K
    [0, 960, 970, 980, 990, 1000, 10000], // attacker P, victim None, P, N, B, R, Q, K
    [0, 860, 870, 880, 890, 900, 10000], // attacker K, victim None, P, N, B, R, Q, K
    [0, 760, 770, 780, 790, 800, 10000], // attacker B, victim None, P, N, B, R, Q, K
    [0, 660, 670, 680, 690, 700, 10000], // attacker R, victim None, P, N, B, R, Q, K
    [0, 560, 570, 580, 590, 600, 10000], // attacker Q, victim None, P, N, B, R, Q, K
    [0, 70, 70, 70, 70, 70, 0], // attacker K, victim None, P, N, B, R, Q, K
];

pub const PROMOTION_POINTS: [i16; 7] = [
    0,  // Empty
    0,  // Pawn
    40, // Knight
    0,  // Bishop
    0,  // Rook
    60, // Queen
    0,  // King
];

pub fn score_mmv_lva(moveslist: &mut MoveList, search_cache: &mut SearchCache, ply: usize) {
    for i in 0..moveslist.len() {
        let move_item = &mut moveslist.moves[i];
        // move_item.score += PROMOTION_POINTS[move_item.promotion_piece as usize];
        // if move_item.capturing {
        move_item.score =
            MVV_LVA_SCORE[move_item.piece as usize][move_item.captured_piece as usize];
        // }
        // else {
        //     for i in 0..MAX_KILLER_MOVES {
        //         if is_similar(&search_cache.killer_moves[ply][i], move_item) {
        //             move_item.score += 40 - ((i * 10) as i16);
        //             break;
        //         }
        //     }
        // }
    }
}

pub fn score_moves(moveslist: &mut MoveList, search_cache: &mut SearchCache, ply: usize) {
    for i in 0..moveslist.len() {
        let move_item = &mut moveslist.moves[i];
        // move_item.score += PROMOTION_POINTS[move_item.promotion_piece as usize];
        if move_item.capturing {
            move_item.score =
                MVV_LVA_SCORE[move_item.piece as usize][move_item.captured_piece as usize];
        } else {
            for i in 0..MAX_KILLER_MOVES {
                if is_similar(&search_cache.killer_moves[ply][i], move_item) {
                    move_item.score += 40 - ((i * 10) as i16);
                    break;
                }
            }
        }
    }
}

pub fn score_moves_see(moveslist: &mut MoveList, game: &mut GameState, cache: &PrecalculatedCache) {
    for i in 0..moveslist.len() {
        moveslist.moves[i].score = see_move_ordering(&moveslist.moves[i], game, cache);
    }
}

/*
int see(int square, int side)
{
   value = 0;
   piece = get_smallest_attacker(square, side);
   /* skip if the square isn't attacked anymore by this side */
   if ( piece )
   {
      make_capture(piece, square);
      /* Do not consider captures if they lose material, therefor max zero */
      value = max (0, piece_just_captured() -see(square, other(side)) );
      undo_capture(piece, square);
   }
   return value;
}
*/

// for move ordering only
// super unperformant
pub fn see_move_ordering(
    move_item: &MoveItem,
    game: &mut GameState,
    cache: &PrecalculatedCache,
) -> i16 {
    let square = move_item.to_pos;

    let original = game.clone();
    let mut value = STATIC_PIECE_POINTS[move_item.captured_piece as usize] as i16;
    game.make_move(move_item);

    let mut color = -1;
    while let Some(response_move) =
        smallest_attacker_for_see(square, game.side_to_move, game, cache)
    {
        value += color * STATIC_PIECE_POINTS[response_move.captured_piece as usize] as i16;
        game.make_move(&response_move);
        color *= -1;
    }
    game.set(original);

    return value;
}
