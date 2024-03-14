/*
   function alphabeta(node, depth, α, β, maximizingPlayer) is
       if depth == 0 or node is terminal then
           return the heuristic value of node
       if maximizingPlayer then
           value := −∞
           for each child of node do
               value := max(value, alphabeta(child, depth − 1, α, β, FALSE))
               α := max(α, value)
               if value ≥ β then
                   break (* β cutoff *)
           return value
       else
           value := +∞
           for each child of node do
               value := min(value, alphabeta(child, depth − 1, α, β, TRUE))
               β := min(β, value)
               if value ≤ α then
                   break (* α cutoff *)
           return value
*/

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

use std::time::Instant;

use crate::{
    evaluation::{self, moves::score_moves},
    moves::{
        attacked::in_check::is_in_check, move_data::MoveItem,
        precalculate::cache::PrecalculatedCache, pseudolegal::all::generate_pseudolegal_moves,
    },
    state::{game::GameState, movelist::MoveList, pieces::Piece, player::Player},
};

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;

pub fn iterative_deepening(
    mut game: GameState,
    has_time: &bool,
    cache: &PrecalculatedCache,
) -> Option<MoveItem> {
    let now = Instant::now();

    const MAX_PLY: u8 = 30;
    const MAX_MAIN_SEARCH_DEPTH: u8 = 8;
    let mut best_move: Option<MoveItem> = None;
    let mut depth = 0;
    while depth <= MAX_MAIN_SEARCH_DEPTH && *has_time {
        println!("max depth {}", depth);
        let player = game.side_to_move;
        let color = if player == Player::White { 1 } else { -1 };

        let score = negamax(
            &mut game,
            depth,
            0,
            MAX_PLY,
            NEG_INF,
            INF,
            color,
            &mut best_move,
            cache,
            false,
        );
        println!("Score: {score}");
        if let Some(m) = &best_move {
            println!("Best: {:?}", m.pure_algebraic_coordinate_notation());
        }
        if score == INF {
            break;
        }
        depth += 1;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    return best_move;
}

pub fn quiescence(
    game: &mut GameState,
    depth: u8,
    quiescence_depth: u8,
    ply: u8,
    max_ply: u8,
    mut alpha: i32,
    beta: i32,
    color: i32,
    best_move: &mut Option<MoveItem>,
    cache: &PrecalculatedCache,
) -> i32 {
    let player = game.side_to_move;
    let in_check = is_in_check(player, &game, cache);

    let mut moveslist = MoveList::new();

    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    score_moves(&mut moveslist);
    let mut legal_moves_count: u8 = 0;

    if quiescence_depth == 0 {
        for index in 0..moveslist.len() {
            let move_item: &MoveItem = &moveslist.moves[index];
            let unmake_metadata = game.make_move(move_item);
            if !is_in_check(player, &game, cache) {
                legal_moves_count += 1;
            }
            game.unmake_move(&move_item, unmake_metadata);
        }
        if in_check && legal_moves_count == 0 {
            return NEG_INF;
        }
        return color * evaluation::position::simple(game);
    }

    let mut max = -std::i32::MAX;

    for index in 0..moveslist.len() {
        moveslist.sort_move(index);
        let move_item: &MoveItem = &moveslist.moves[index];
        if in_check || move_item.capturing {
            let unmake_metadata = game.make_move(move_item);
            if !is_in_check(player, &game, cache) {
                legal_moves_count += 1;

                let score = -quiescence(
                    game,
                    depth - 1,
                    quiescence_depth - 1,
                    ply + 1,
                    max_ply,
                    -beta,
                    -alpha,
                    -color,
                    best_move,
                    cache,
                );

                game.unmake_move(&move_item, unmake_metadata);

                max = std::cmp::max(max, score);

                alpha = std::cmp::max(alpha, max);
                if alpha >= beta {
                    break;
                }
            } else {
                game.unmake_move(&move_item, unmake_metadata);
            }
        }
    }

    if in_check && legal_moves_count == 0 {
        let q = quiescence(
            game,
            0,
            0,
            ply + 1,
            max_ply,
            alpha,
            beta,
            color,
            best_move,
            cache,
        );
        return q;
    }

    return max;
}

pub fn negamax(
    game: &mut GameState,
    depth: u8,
    ply: u8,
    max_ply: u8,
    mut alpha: i32,
    beta: i32,
    color: i32,
    best_move: &mut Option<MoveItem>,
    cache: &PrecalculatedCache,
    last_move_capturing: bool,
) -> i32 {
    let player = game.side_to_move;
    let in_check = is_in_check(player, &game, cache);

    let mut moveslist = MoveList::new();

    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    score_moves(&mut moveslist);
    let mut legal_moves_count: u8 = 0;

    // no extensions should happen
    if ply == max_ply {
        return color * evaluation::position::simple(game);
    }
    if depth == 0 {
        // check extension
        if in_check {
            return negamax(
                game,
                depth + 1,
                ply,
                max_ply,
                alpha,
                beta,
                color,
                best_move,
                cache,
                last_move_capturing,
            );
        }

        // quiescence search
        if last_move_capturing {}

        return color * evaluation::position::simple(game);
    }

    let mut max = -std::i32::MAX;

    for index in 0..moveslist.len() {
        moveslist.sort_move(index);
        let move_item: &MoveItem = &moveslist.moves[index];
        let unmake_metadata = game.make_move(move_item);

        if !is_in_check(player, &game, cache) {
            legal_moves_count += 1;

            let score = -negamax(
                game,
                depth - 1,
                ply + 1,
                max_ply,
                -beta,
                -alpha,
                -color,
                best_move,
                cache,
                move_item.capturing,
            );

            game.unmake_move(&move_item, unmake_metadata);

            max = std::cmp::max(max, score);

            if ply == 0 {
                if score == max {
                    *best_move = Some(move_item.clone());
                }
            }

            alpha = std::cmp::max(alpha, max);
            if alpha >= beta {
                break;
            }
        } else {
            game.unmake_move(&move_item, unmake_metadata);
        }
    }

    if in_check && legal_moves_count == 0 {
        return NEG_INF;
    }

    return max;
}
