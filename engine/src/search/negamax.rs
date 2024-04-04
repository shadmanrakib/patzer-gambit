use crate::evaluation::moves::score_moves;
use crate::evaluation::{self, psqt_tapered};
use crate::moves::attacked::in_check::{self, is_in_check};
use crate::moves::move_data::MoveItem;
use crate::moves::precalculate::cache::PrecalculatedCache;
use crate::moves::pseudolegal::all::generate_pseudolegal_moves;
use crate::search::cache::SearchCache;
use crate::state::game::GameState;
use crate::state::movelist::MoveList;
use crate::state::player::Player;

use super::killer::store_killer_move;
use super::quiescence::quiescence;

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;
const STALEMATE: i32 = 0;
const CHECKMATE: i32 = NEG_INF;

pub struct NegamaxResult {
    pub move_item: Option<MoveItem>,
    pub score: i32,
}

pub fn negamax2(
    game: &mut GameState,
    mut depth: u8,
    ply: u8,
    max_ply: u8,
    mut alpha: i32,
    beta: i32,
    cache: &PrecalculatedCache,
    search_cache: &mut SearchCache,
) -> NegamaxResult {
    let player = game.side_to_move;

    // handle base case
    if ply == max_ply {
        let color = if player == Player::White { 1 } else { -1 };
        return NegamaxResult {
            move_item: None,
            score: color * psqt_tapered::eval(&game),
        };
    }
    if depth == 0 {
        return NegamaxResult {
            move_item: None,
            score: quiescence(game, ply, max_ply, alpha, beta, cache, search_cache),
        };
        // return quiescence(game, ply, max_ply, alpha, beta, cache, search_cache);
    }

    let in_check = is_in_check(player, game, cache);

    // check extension
    if in_check {
        depth += 1;
    }

    let mut best_move: Option<MoveItem> = None;
    let mut best_move_idx: i32 = -1;
    let mut best_score: i32 = -INF;

    let mut moveslist = MoveList::new();
    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    score_moves(&mut moveslist, search_cache, ply as usize, player);
    let mut legal_moves_count: u8 = 0;

    for index in 0..moveslist.len() {
        moveslist.sort_move(index);
        let move_item = &moveslist.moves[index];
        let unmake_metadata = game.make_move(&move_item);

        // illegal move
        if is_in_check(player, game, cache) {
            game.unmake_move(&move_item, unmake_metadata);
            continue;
        };

        // wooo, we have a legal move
        legal_moves_count += 1;

        let score = -negamax2(
            game,
            depth - 1,
            ply + 1,
            max_ply,
            -beta,
            -alpha,
            cache,
            search_cache,
        )
        .score;

        game.unmake_move(&move_item, unmake_metadata);

        // if ply == 0 {
        //     println!(
        //         "{} {}",
        //         move_item.pure_algebraic_coordinate_notation(),
        //         score
        //     );
        // }

        // if score >= beta {
        //     return NegamaxResult {
        //         score,
        //         move_item: Some(move_item.clone()),
        //     }
        // }

        if score > best_score {
            best_score = score;
            best_move_idx = index as i32;
        }

        alpha = std::cmp::max(alpha, score);

        if alpha >= beta {
            search_cache.history_moves[player as usize][move_item.piece as usize]
                [move_item.to_pos as usize] += depth as i32;
            if !move_item.capturing {
                store_killer_move(&mut search_cache.killer_moves, move_item, ply as usize);
            }
            break;
        }
    }

    if best_move_idx >= 0 {
        best_move = Some(moveslist.moves[best_move_idx as usize].clone());
    }

    if legal_moves_count == 0 {
        return NegamaxResult {
            score: if in_check { CHECKMATE } else { STALEMATE },
            move_item: best_move,
        };
    }

    NegamaxResult {
        move_item: best_move,
        score: best_score,
    }
}
