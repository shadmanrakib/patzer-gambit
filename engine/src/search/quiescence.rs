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

use crate::{
    constants::search::MAX_PLY,
    evaluation::{
        moves::score_captures,
        psqt_tapered,
    },
    moves::{
        attacked::in_check::is_in_check, move_data::MoveItem,
        precalculate::cache::PrecalculatedCache, pseudolegal::all::generate_pseudolegal_moves,
    },
    state::{game::GameState, movelist::MoveList, player::Player},
};

use super::{
    cache::SearchCache,
    killer::SimpleMove,
    zobrist::ZobristRandomKeys,
};

pub fn quiescence(
    game: &mut GameState,
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
    keys: &ZobristRandomKeys,
) -> i32 {
    *nodes += 1;

    let player = game.side_to_move;
    let color = if player == Player::White { 1 } else { -1 };

    let stand_pat = color * psqt_tapered::eval(game);

    if ply >= max_ply {
        return stand_pat;
    }

    // stand-pat here
    if stand_pat >= beta {
        return stand_pat;
    }

    // position better than alpha already
    if stand_pat > alpha {
        alpha = stand_pat
    }

    let mut moveslist = MoveList::new();
    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    score_captures(&mut moveslist);

    for i in 0..moveslist.len() {
        moveslist.sort_move(i);
        let move_item: &MoveItem = &moveslist.moves[i];

        if !move_item.capturing {
            continue;
        }

        let unmake_metadata = game.make_move(move_item, keys);

        if is_in_check(player, &game, cache) {
            game.unmake_move(move_item, unmake_metadata, keys);
            continue;
        }

        // The position is not yet quiet. Go one ply deeper.
        let score = -quiescence(
            game,
            ply + 1,
            max_ply,
            -beta,
            -alpha,
            // pv: &mut Vec<SimpleMove>,
            pv_table,
            pv_size,
            cache,
            search_cache,
            nodes,
            keys,
        );

        game.unmake_move(move_item, unmake_metadata, keys);

        // beta cutoff
        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }

    alpha
}
