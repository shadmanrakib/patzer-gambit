use std::time::Instant;

use crate::{
    constants::search::MAX_PLY,
    moves::{data::MoveItem, generator::precalculated_lookups::cache::PrecalculatedCache},
    search::{cache::SearchCache, negamax::negamax},
    state::game::GameState,
};

use super::{transposition::TTable, zobrist::ZobristRandomKeys};

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;

pub fn iterative_deepening(
    mut game: GameState,
    has_time: &bool,
    cache: &PrecalculatedCache,
    main_search_depth: u8,
    keys: &ZobristRandomKeys,
    tt: &mut TTable,
) -> Option<MoveItem> {
    let mut best_move: Option<MoveItem> = None;

    let mut depth = 1;
    let mut search_cache = SearchCache::init();

    let mut alpha = NEG_INF;
    let mut beta = INF;

    println!("hash {}", game.hash);

    while depth <= main_search_depth && *has_time {
        let iter_start = Instant::now();

        let mut nodes = 0;
        let mut q_nodes = 0;

        let result = negamax(
            &mut game,
            depth,
            0,
            MAX_PLY,
            alpha,
            beta,
            cache,
            &mut search_cache,
            &mut nodes,
            &mut q_nodes,
            keys,
            tt,
            &mut best_move,
        );

        let score = result.score;

        let iter_elapsed = iter_start.elapsed();
        let ms = iter_elapsed.as_millis();
        let ns = iter_elapsed.as_nanos();

        let nps = (nodes as u128) * 10_u128.pow(9) / (ns + 1);

        let total_nps = (nodes + q_nodes) * 10_u128.pow(9) / (ns + 1);

        if let Some(m) = &best_move {
            let short = m.notation();
            println!("info currmove {short} depth {depth} score cp {score} time {ms} nodes {nodes} nps {nps} qnodes {q_nodes} tnps {total_nps}");
        } else {
            println!("info depth {depth} score cp {score} time {ms} nodes {nodes} nps {nps} qnodes {q_nodes} tnps {total_nps}");
        }

        if score == INF || score == NEG_INF {
            break;
        }

        // the aspiration window was a bad idea, lets retry without it
        if score <= alpha || score >= beta {
            alpha = NEG_INF;
            beta = INF;
            println!("retrying");
            continue;
        }

        alpha = score - 70;
        beta = score + 70;
        depth += 1;
    }

    // println!("pv: {:?}", pv);

    if let Some(move_item) = &best_move {
        let short = move_item.notation();
        println!("bestmove {short}");
    }

    println!("hash {}", game.hash);

    return best_move;
}
