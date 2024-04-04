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
    evaluation::{
        self,
        moves::{score_mmv_lva, score_moves},
        position, psqt_tapered,
    },
    moves::{
        attacked::in_check::is_in_check, move_data::MoveItem,
        precalculate::cache::PrecalculatedCache, pseudolegal::all::generate_pseudolegal_moves,
    },
    search::{
        cache::SearchCache,
        negamax::{negamax2, NegamaxResult},
    },
    state::{game::GameState, movelist::MoveList, player::Player},
};

use super::killer::store_killer_move;

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;

fn eval(game: &GameState) -> i32 {
    return position::simple(game);
    // return psqt_tapered::eval(game);
}

pub fn iterative_deepening(
    mut game: GameState,
    has_time: &bool,
    cache: &PrecalculatedCache,
    max_depth: u8,
) -> Option<MoveItem> {
    let start = Instant::now();

    let mut best_move: Option<MoveItem> = None;
    let mut score = 0;

    let mut depth = 0;
    let mut search_cache = SearchCache::init();

    // println!("s: {} {:?} {:?}", game.phase, game.opening, game.endgame);

    while depth <= max_depth && *has_time {
        // println!("depth {}", depth);
        // let iter_start = Instant::now();

        let result = negamax2(&mut game, depth, 0, MAX_MAIN_SEARCH_DEPTH, NEG_INF, INF, cache, &mut search_cache);
        best_move = result.move_item;
        score = result.score; 

        // println!("Score: {score}");
        // if let Some(m) = &best_move {
        //     println!("Best: {:?}", m.pure_algebraic_coordinate_notation());
        // }
        // let iter_elapsed = iter_start.elapsed();
        // println!("Iter Elapsed: {} ms", iter_elapsed.as_millis());

        if score == INF || score == NEG_INF {
            break;
        }
        depth += 1;
    }

    // println!("e: {} {:?} {:?}", game.phase, game.opening, game.endgame);
    depth -= 1;
    let total_elapsed = start.elapsed();
    println!(
        "Total Elapsed: {} ms, Depth: {depth}, Score: {score}",
        total_elapsed.as_millis()
    );

    return best_move;
}

pub fn quiescence(
    game: &mut GameState,
    ply: u8,
    max_ply: u8,
    mut alpha: i32,
    beta: i32,
    color: i32,
    best_move: &mut Option<MoveItem>,
    cache: &PrecalculatedCache,
    search_cache: &mut SearchCache,
) -> i32 {
    if ply == max_ply {
        return color * eval(game);
    }

    let player = game.side_to_move;
    let in_check = is_in_check(player, &game, cache);

    let stand_pat = {
        // if in_check {
        //     // we want to check for checkmate, this will extend it to do that
        //     negamax(
        //         game,
        //         0,
        //         ply,
        //         max_ply,
        //         alpha,
        //         beta,
        //         color,
        //         best_move,
        //         cache,
        //         true,
        //         search_cache,
        //     )
        // } else {
        color * eval(game)
        // }
    };

    if stand_pat >= beta {
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }

    let mut moveslist = MoveList::new();
    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    score_mmv_lva(&mut moveslist, search_cache, ply as usize);
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
                    -color,
                    best_move,
                    cache,
                    search_cache,
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

pub fn negamax(
    game: &mut GameState,
    mut depth: u8,
    ply: u8,
    max_ply: u8,
    mut alpha: i32,
    beta: i32,
    best_move: &mut Option<MoveItem>,
    cache: &PrecalculatedCache,
    search_cache: &mut SearchCache,
) -> i32 {
    let player = game.side_to_move;
    let in_check = is_in_check(player, &game, cache);
    let color = if player == Player::White { 1 } else { -1 };

    // check extension
    if in_check {
        depth += 1;
    }

    if depth == 0 || ply >= max_ply {
        // quiescence search
        // if last_move_capturing {
        //     return quiescence(
        //         game,
        //         ply,
        //         max_ply,
        //         alpha,
        //         beta,
        //         color,
        //         best_move,
        //         cache,
        //         search_cache,
        //     );
        // }

        return color * eval(game);
    }

    let mut moveslist = MoveList::new();
    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    score_moves(&mut moveslist, search_cache, ply as usize);
    let mut legal_moves_count: u8 = 0;

    let mut max = -std::i32::MAX;

    for index in 0..moveslist.len() {
        moveslist.sort_move(index);
        let move_item: &MoveItem = &moveslist.moves[index];
        let unmake_metadata = game.make_move(move_item);

        // illegal
        if is_in_check(player, &game, cache) {
            game.unmake_move(&move_item, unmake_metadata);
            continue;
        }

        legal_moves_count += 1;

        let score = -negamax(
            game,
            depth - 1,
            ply + 1,
            max_ply,
            -beta,
            -alpha,
            best_move,
            cache,
            search_cache,
        );

        if ply == 0 && "e1f1" == move_item.pure_algebraic_coordinate_notation() {
            game.print_board();
            let s = -negamax(
                game,
                depth - 1,
                ply + 1,
                max_ply,
                -INF,
                INF,
                best_move,
                cache,
                search_cache,
            );
            println!(
                "OOOF {} {} {beta} {alpha}  -  {s}",
                move_item.pure_algebraic_coordinate_notation(),
                score
            );
        }

        game.unmake_move(&move_item, unmake_metadata);
        if score > alpha {
            if ply == 0 {
                *best_move = Some(move_item.clone());
            }
        }
        alpha = std::cmp::max(alpha, score);
        if ply == 0 {
            println!(
                "{} {}",
                move_item.pure_algebraic_coordinate_notation(),
                score
            );
        }

        if score >= beta {
            if !move_item.capturing {
                store_killer_move(&mut search_cache.killer_moves, move_item, ply as usize);
            }
            return beta;
        }
    }

    if in_check && legal_moves_count == 0 {
        return NEG_INF;
    } else if legal_moves_count == 0 {
        return 0;
    }

    return alpha;
}
