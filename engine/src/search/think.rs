/*
function negamax(node, depth, α, β, color) is
    if depth = 0 or node is a terminal node then
        return color × the heuristic value of node

    childNodes := generateMoves(node)
    childNodes := orderMoves(childNodes)
    value := −∞
    foreach child in childNodes do
        value := max(value, −negamax(child, depth − 1, −β, −α, −color))
        α := max(α, value)
        if α ≥ β then
            break (* cut-off *)
    return value
*/

/*
int Quiesce( int alpha, int beta ) {
    int stand_pat = Evaluate();
    if( stand_pat >= beta )
        return beta;
    if( alpha < stand_pat )
        alpha = stand_pat;

    until( every_capture_has_been_examined )  {
        MakeCapture();
        score = -Quiesce( -beta, -alpha );
        TakeBackMove();

        if( score >= beta )
            return beta;
        if( score > alpha )
           alpha = score;
    }
    return alpha;
}
*/

use std::time::Instant;

use crate::{
    constants::search::{MAX_MAIN_SEARCH_DEPTH, MAX_PLY},
    moves::{move_data::MoveItem, precalculate::cache::PrecalculatedCache},
    search::{cache::SearchCache, negamax::negamax},
    state::game::GameState,
};

use super::killer::SimpleMove;

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;

pub fn iterative_deepening(
    mut game: GameState,
    has_time: &bool,
    cache: &PrecalculatedCache,
    max_depth: u8,
) -> Option<MoveItem> {
    let mut best_move: Option<MoveItem> = None;

    let mut depth = 0;
    let mut search_cache = SearchCache::init();

    // println!("s: {} {:?} {:?}", game.phase, game.opening, game.endgame);
    let mut pv: Vec<SimpleMove> = Vec::new();

    while depth <= max_depth && *has_time {
        let iter_start = Instant::now();

        let mut nodes = 0;
        let mut q_nodes = 0;

        let result = negamax(
            &mut game,
            depth,
            0,
            MAX_PLY,
            NEG_INF,
            INF,
            &mut pv,
            cache,
            &mut search_cache,
            &mut nodes,
            &mut q_nodes,
        );

        best_move = result.move_item;
        let score = result.score;

        let iter_elapsed = iter_start.elapsed();
        let ms = iter_elapsed.as_millis();
        let ns = iter_elapsed.as_nanos();

        let nps = (nodes as u128) * 10_u128.pow(9) / ns;

        if let Some(m) = &best_move {
            let short = m.pure_algebraic_coordinate_notation();
            println!("info currmove {short} depth {depth} score cp {score} time {ms} nodes {nodes} nps {nps} qnodes {q_nodes}");
        } else {
            println!("info depth {depth} score cp {score} time {ms} nodes {nodes} nps {nps} qnodes {q_nodes}");
        }
        if score == INF || score == NEG_INF {
            break;
        }
        depth += 1;
    }

    println!("pv: {:?}", pv);

    if let Some(move_item) = &best_move {
        let short = move_item.pure_algebraic_coordinate_notation();
        println!("bestmove {short}");
    }

    return best_move;
}
