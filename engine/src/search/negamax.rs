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

use crate::constants::search::MAX_PLY;
use crate::evaluation::moves::score_moves;
use crate::evaluation::psqt_tapered;
use crate::moves::attacked::in_check::is_in_check;
use crate::moves::move_data::MoveItem;
use crate::moves::precalculate::cache::PrecalculatedCache;
use crate::moves::pseudolegal::all::generate_pseudolegal_moves;
use crate::search::cache::SearchCache;
use crate::state::game::GameState;
use crate::state::movelist::MoveList;
use crate::state::pieces::Piece;
use crate::state::player::Player;

use super::killer::{store_killer_move, SimpleMove};
use super::quiescence::quiescence;
use super::transposition::{NodeType, TTable};
use super::zobrist::ZobristRandomKeys;

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;
const STALEMATE: i32 = 0;
const CHECKMATE: i32 = NEG_INF;

pub struct NegamaxResult {
    // pub move_item: Option<MoveItem>,
    pub score: i32,
}

pub fn negamax(
    game: &mut GameState,
    mut depth: u8,
    ply: u8,
    max_ply: u8,
    mut alpha: i32,
    beta: i32,
    // pv: &mut Vec<SimpleMove>,
    pv_table: &mut [[SimpleMove; MAX_PLY as usize]; MAX_PLY as usize],
    pv_size: &mut [usize; MAX_PLY as usize],
    cache: &PrecalculatedCache,
    search_cache: &mut SearchCache,
    nodes: &mut u128,
    q_nodes: &mut u128,
    keys: &ZobristRandomKeys,
    tt: &mut TTable,
    best_move: &mut Option<MoveItem>,
) -> NegamaxResult {
    pv_size[ply as usize] = ply as usize;

    let player = game.side_to_move;

    let mut node_type = NodeType::All;

    // if GameState::from_fen(game.to_fen(), keys).unwrap().hash != game.hash {
    //     println!(
    //         "{depth} {} {}",
    //         GameState::from_fen(game.to_fen(), keys).unwrap().hash,
    //         game.hash
    //     );
    //     panic!();
    // }

    if let Some((simple_move, score, d)) = tt.probe(game.hash, alpha, beta) {
        if d >= depth {
            let mut moveslist = MoveList::new();
            // generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
            for index in 0..moveslist.len() {
                let move_item = &moveslist.moves[index];

                if move_item.from_pos == simple_move.from
                    && move_item.to_pos == simple_move.to
                    && move_item.promotion_piece == simple_move.promotion
                {
                    return NegamaxResult {
                        score,
                    };
                }
            }
            return NegamaxResult {
                score,
            };
        }
    }

    // handle base case
    if ply == max_ply {
        *nodes += 1;

        let color = if player == Player::White { 1 } else { -1 };
        let score = color * psqt_tapered::eval(game);
        // let score = color * position::simple(game);
        return NegamaxResult {
            score,
        };
    }
    if depth <= 0 {
        let score = quiescence(
            game,
            ply,
            max_ply,
            alpha,
            beta,
            // pv,
            pv_table,
            pv_size,
            cache,
            search_cache,
            q_nodes,
            keys,
        );
        tt.record(
            game.hash,
            SimpleMove {
                from: 0,
                to: 0,
                promotion: Piece::King,
            },
            score,
            depth,
            NodeType::Pv,
        );
        return NegamaxResult {
            score,
        };

        // return quiescence(game, ply, max_ply, alpha, beta, cache, search_cache);
    }

    *nodes += 1;

    let in_check = is_in_check(player, game, cache);

    // null pruning
    if !in_check
        && depth >= 3
        && ply != 0
        && game.phase < 170
        && game.bitboards.boards[player as usize][Piece::Pawn as usize] != 0
        && game.bitboards.boards[player.opponent() as usize][Piece::Pawn as usize] != 0
    {
        let prev_enpassant = game.make_null_move(keys);
        let reduction = 2;
        let null_score = -negamax(
            game,
            depth - 1 - reduction,
            ply + 1,
            max_ply,
            -beta,
            -beta + 1,
            // &mut child_pv,
            pv_table,
            pv_size,
            cache,
            search_cache,
            nodes,
            q_nodes,
            keys,
            tt,
            best_move,
        )
        .score;
        game.unmake_null_move(prev_enpassant, keys);

        if null_score >= beta {
            return NegamaxResult {
                score: beta,
            };
        }
    }

    // check extension
    if in_check {
        depth += 1;
    }

    let mut best_move_idx: i32 = -1;
    let mut best_score: i32 = -INF;

    let mut moveslist = MoveList::new();
    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    score_moves(
        &mut moveslist,
        search_cache,
        ply as usize,
        player,
        // &pv,
        pv_table,
        pv_size,
    );
    let mut legal_moves_count: u8 = 0;
    let mut found_pv_node = false;

    for index in 0..moveslist.len() {
        moveslist.sort_move(index);
        let move_item = &moveslist.moves[index];
        let unmake_metadata = game.make_move(&move_item, keys);

        // illegal move
        if is_in_check(player, game, cache) {
            game.unmake_move(&move_item, unmake_metadata, keys);
            continue;
        };

        // println!("m {}", move_item.pure_algebraic_coordinate_notation());

        // wooo, we have a legal move
        legal_moves_count += 1;

        // let mut child_pv = Vec::new();
        let mut score;
        if found_pv_node {
            // perform pvs search, i.e prove other moves are worse
            score = -negamax(
                game,
                depth - 1,
                ply + 1,
                max_ply,
                -alpha - 1,
                -alpha,
                // &mut child_pv,
                pv_table,
                pv_size,
                cache,
                search_cache,
                nodes,
                q_nodes,
                keys,
                tt,
                best_move,
            )
            .score;

            // proof failed, resort to normal search
            if score > alpha && score < beta {
                score = -negamax(
                    game,
                    depth - 1,
                    ply + 1,
                    max_ply,
                    -beta,
                    -alpha,
                    // &mut child_pv,
                    pv_table,
                    pv_size,
                    cache,
                    search_cache,
                    nodes,
                    q_nodes,
                    keys,
                    tt,
                    best_move,
                )
                .score;
            }
        } else {
            score = -negamax(
                game,
                depth - 1,
                ply + 1,
                max_ply,
                -beta,
                -alpha,
                // &mut child_pv,
                pv_table,
                pv_size,
                cache,
                search_cache,
                nodes,
                q_nodes,
                keys,
                tt,
                best_move,
            )
            .score;
        }

        game.unmake_move(&move_item, unmake_metadata, keys);

        // println!("u {}", move_item.pure_algebraic_coordinate_notation());

        if score > best_score {
            best_score = score;
            best_move_idx = index as i32;
        }

        if score >= beta {
            if !move_item.capturing {
                store_killer_move(&mut search_cache.killer_moves, move_item, ply as usize);
            }

            tt.record(game.hash, move_item.into(), score, depth, NodeType::Cut);

            return NegamaxResult {
                score,
            };
        }

        if score > alpha {
            alpha = score;

            node_type = NodeType::Pv;

            found_pv_node = true; // assume we have a pv node, so we can do pvs

            if !move_item.capturing {
                search_cache.history_moves[player as usize][move_item.piece as usize]
                    [move_item.to_pos as usize] += 2_i16.pow(depth as u32);
            }

            if ply == 0{
                *best_move = Some(move_item.clone());
            }
            
            // update pv
            pv_table[ply as usize][ply as usize] = move_item.into();
            let next_ply = (ply + 1) as usize;
            for copying in next_ply..pv_size[next_ply] {
                pv_table[ply as usize][copying] = pv_table[next_ply][copying];
            }
            pv_size[ply as usize] = pv_size[next_ply];
        }
    }

    if legal_moves_count == 0 {
        let score = if in_check { CHECKMATE } else { STALEMATE };

        tt.record(
            game.hash,
            SimpleMove {
                to: 0,
                from: 0,
                promotion: Piece::King,
            },
            score,
            depth,
            NodeType::Pv,
        );

        return NegamaxResult {
            score,
        };
    }

    if best_move_idx >= 0 {
        tt.record(
            game.hash,
            (&moveslist.moves[best_move_idx as usize]).into(),
            best_score,
            depth,
            node_type,
        );
    } else {
        tt.record(
            game.hash,
            SimpleMove {
                to: 0,
                from: 0,
                promotion: Piece::King,
            },
            best_score,
            depth,
            node_type,
        );
    }

    NegamaxResult {
        score: best_score,
    }
}
