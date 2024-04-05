use crate::{
    evaluation::{moves::score_mmv_lva, position, psqt_tapered},
    moves::{
        attacked::in_check::is_in_check, move_data::MoveItem,
        precalculate::cache::PrecalculatedCache, pseudolegal::all::generate_pseudolegal_moves,
    },
    state::{game::GameState, movelist::MoveList, player::Player},
};

use super::{cache::SearchCache, killer::SimpleMove};

pub fn quiescence(
    game: &mut GameState,
    ply: u8,
    max_ply: u8,
    mut alpha: i32,
    beta: i32,
    pv: &mut Vec<SimpleMove>,
    cache: &PrecalculatedCache,
    search_cache: &mut SearchCache,
    nodes: &mut u128,
) -> i32 {
    *nodes += 1;

    let player = game.side_to_move;
    let color = if player == Player::White { 1 } else { -1 };

    let stand_pat = color * psqt_tapered::eval(game);
    // let stand_pat = color * position::simple(game);

    if ply == max_ply {
        return stand_pat;
    }

    // let in_check = is_in_check(player, &game, cache);

    if stand_pat >= beta {
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }

    let mut moveslist = MoveList::new();
    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    // would be better to order by static exchange and prune < 0
    score_mmv_lva(&mut moveslist);
    // score_moves_see(&mut moveslist, game, cache);

    for index in 0..moveslist.len() {
        moveslist.sort_move(index);
        let move_item: &MoveItem = &moveslist.moves[index];
        if move_item.capturing {
            let unmake_metadata = game.make_move(move_item);
            if !is_in_check(player, &game, cache) {
                let score = -quiescence(
                    game,
                    ply + 1,
                    max_ply,
                    -beta,
                    -alpha,
                    pv,
                    cache,
                    search_cache,
                    nodes,
                );
                game.unmake_move(&move_item, unmake_metadata);

                if score >= beta {
                    return beta;
                }
                if score > alpha {
                    alpha = score;
                }
            } else {
                game.unmake_move(&move_item, unmake_metadata);
            }
        } else {
            break;
        }
    }

    return alpha;
}
