use crate::constants::search::{FULL_DEPTH_MOVES, REDUCTION_LIMIT};
use crate::evaluation::psqt_tapered;
use crate::moves::data::MoveItem;
use crate::moves::generator::movegen::generate_pseudolegal_moves;
use crate::moves::generator::precalculated_lookups::cache::PrecalculatedCache;
use crate::moves::scoring::score_moves;
use crate::search::cache::SearchCache;
use crate::state::game::GameState;
use crate::state::movelist::MoveList;
use crate::state::pieces::Piece;
use crate::state::player::Player;
use crate::utils::in_check::is_in_check;

use super::killer::{store_killer_move, SimpleMove};
use super::quiescence::quiescence;
use super::transposition::{NodeType, TTable};
use super::zobrist::ZobristRandomKeys;

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;
const STALEMATE: i32 = 0;
const CHECKMATE: i32 = NEG_INF;

pub struct NegamaxResult {
    pub score: i32,
}

pub fn negamax(
    game: &mut GameState,
    mut depth: u8,
    ply: u8,
    max_ply: u8,
    mut alpha: i32,
    beta: i32,
    cache: &PrecalculatedCache,
    search_cache: &mut SearchCache,
    nodes: &mut u128,
    q_nodes: &mut u128,
    keys: &ZobristRandomKeys,
    tt: &mut TTable,
    best_move: &mut Option<MoveItem>,
    seldepth: &mut u8,
) -> NegamaxResult {
    let player = game.side_to_move;

    let mut node_type = NodeType::All;

    *seldepth = std::cmp::max(*seldepth, ply);

    let mut tt_move = SimpleMove {
        from: 0,
        to: 0,
        promotion: Piece::King,
    };
    if game.half_move_clock >= 50 {
        return NegamaxResult { score: 0 };
    }

    if let Some((simple_move, score, d)) = tt.probe(game.hash, alpha, beta) {
        if ply != 0 && d >= depth {
            return NegamaxResult { score };
        };
        tt_move = simple_move;
    }
    // handle base case
    if ply == max_ply {
        *nodes += 1;
        let color = if player == Player::White { 1 } else { -1 };
        let score = color * psqt_tapered::eval(game);
        return NegamaxResult { score };
    }
    if depth <= 0 {
        let score = quiescence(
            game,
            ply,
            max_ply,
            alpha,
            beta,
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
        return NegamaxResult { score };

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
        let r = 2;
        let prev_enpassant = game.make_null_move(keys);
        let null_score = -negamax(
            game,
            depth - 1 - r,
            ply + 1,
            max_ply,
            -beta,
            -beta + 1,
            cache,
            search_cache,
            nodes,
            q_nodes,
            keys,
            tt,
            best_move,
            seldepth,
        )
        .score;
        game.unmake_null_move(prev_enpassant, keys);

        if null_score >= beta {
            return NegamaxResult { score: beta };
        }
    }

    // check extension
    if in_check {
        depth += 1;
    }

    let mut best_move_idx: i32 = -1;
    let mut best_score: i32 = -INF;

    let mut moveslist = MoveList::new();
    generate_pseudolegal_moves(&mut moveslist, &game, player, cache, false);
    score_moves(&mut moveslist, search_cache, ply as usize, player, tt_move);
    let mut legal_moves_count: u8 = 0;
    let mut moves_searched = 0;

    for index in 0..moveslist.len() {
        moveslist.sort_move(index);
        let move_item = &moveslist.moves[index];
        let unmake_metadata = game.make_move(&move_item, keys);

        // illegal move
        if is_in_check(player, game, cache) {
            game.unmake_move(&move_item, unmake_metadata, keys);
            continue;
        };

        legal_moves_count += 1;

        let mut score;
        if moves_searched == 0 {
            score = -negamax(
                game,
                depth - 1,
                ply + 1,
                max_ply,
                -beta,
                -alpha,
                cache,
                search_cache,
                nodes,
                q_nodes,
                keys,
                tt,
                best_move,
                seldepth,
            )
            .score;
        } else {
            if moves_searched >= FULL_DEPTH_MOVES
                && depth >= REDUCTION_LIMIT
                && !in_check
                && !move_item.capturing
                && !move_item.promoting
                && !move_item.castling
            {
                score = -negamax(
                    game,
                    depth - 2,
                    ply + 1,
                    max_ply,
                    -(alpha + 1),
                    -alpha,
                    cache,
                    search_cache,
                    nodes,
                    q_nodes,
                    keys,
                    tt,
                    best_move,
                    seldepth,
                )
                .score;
            } else {
                score = alpha + 1; // done to trigger research
            }

            if score > alpha {
                // like pvs
                score = -negamax(
                    game,
                    depth - 1,
                    ply + 1,
                    max_ply,
                    -(alpha + 1),
                    -alpha,
                    cache,
                    search_cache,
                    nodes,
                    q_nodes,
                    keys,
                    tt,
                    best_move,
                    seldepth,
                )
                .score;
                if score > alpha && score < beta {
                    score = -negamax(
                        game,
                        depth - 1,
                        ply + 1,
                        max_ply,
                        -beta,
                        -alpha,
                        cache,
                        search_cache,
                        nodes,
                        q_nodes,
                        keys,
                        tt,
                        best_move,
                        seldepth,
                    )
                    .score;
                }
            }
        }

        game.unmake_move(&move_item, unmake_metadata, keys);

        moves_searched += 1;

        // if ply == 0 {
        //     println!("{} {}", move_item.notation(), score);
        // }

        if score > best_score {
            best_score = score;
            best_move_idx = index as i32;
            if ply == 0 {
                *best_move = Some(move_item.clone());
            }
        }

        if score > alpha {
            alpha = score;

            node_type = NodeType::Pv;

            // if !move_item.capturing {
            //     search_cache.history_moves[player as usize][move_item.piece as usize]
            //         [move_item.to_pos as usize] = search_cache.history_moves[player as usize]
            //         [move_item.piece as usize][move_item.to_pos as usize]
            //         + (depth as i16);
            // }
        }

        if score >= beta {
            if !move_item.capturing {
                store_killer_move(&mut search_cache.killer_moves, move_item, ply as usize);
            }

            tt.record(game.hash, move_item.into(), score, depth, NodeType::Cut);

            return NegamaxResult { score };
        }
    }

    if legal_moves_count == 0 {
        let score = if in_check {
            CHECKMATE + (ply as i32)
        } else {
            STALEMATE
        };

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

        return NegamaxResult { score };
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

    NegamaxResult { score: best_score }
}
