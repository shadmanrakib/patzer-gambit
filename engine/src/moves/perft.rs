use super::{
    attacked::in_check::is_in_check, move_data::MoveItem, precalculate::cache::PrecalculatedCache,
};
use crate::state::game::GameState;

use std::time::Instant;

pub fn perft0(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    print_individual_move_nodes: bool,
) -> (u64, u64, u64, u64) {
    let mut nodes = 0;
    let mut captures = 0;
    let mut castling = 0;
    let mut checks = 0;
    let mut legal: i32 = 0;

    if depth == 0 {
        return (1, 0, 0, 0);
    }

    let mut move_list = Vec::<MoveItem>::with_capacity(255);
    super::pseudolegal::all::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
    );
    for move_item in move_list {
        let cloned = game.clone();
        let player = game.side_to_move;
        game.make_move(&move_item);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            if move_item.capturing {
                captures += 1;
            }
            if move_item.castling {
                castling += 1;
            }
            if is_in_check(player.opponent(), game, cache) {
                checks += 1;
            }

            legal += 1;

            let move_nodes = perft0(game, cache, depth - 1, false);
            nodes += move_nodes.0;
            captures += move_nodes.1;
            castling += move_nodes.2;
            checks += move_nodes.3;

            if print_individual_move_nodes {
                println!(
                    "{}: {}",
                    move_item.pure_algebraic_coordinate_notation(),
                    move_nodes.0
                );
            }
        } else {
            // game.print_board();
        }
        // replace with unset
        game.set(cloned);
    }

    if legal == 0 {
        // println!("checkmate");
    }

    return (nodes, captures, castling, checks);
}

pub fn perft(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
    let now = Instant::now();

    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = Vec::<MoveItem>::with_capacity(255);
    super::pseudolegal::all::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
    );
    for move_item in move_list {
        let cloned = game.clone();
        let player = game.side_to_move;
        game.make_move(&move_item);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            let move_nodes = _perft(game, cache, depth - 1);
            nodes += move_nodes;
            // println!(
            //     "{}: {}",
            //     move_item.pure_algebraic_coordinate_notation(),
            //     move_nodes
            // );
        }
        // replace with unset
        game.set(cloned);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    return nodes;
}

fn _perft(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = Vec::<MoveItem>::with_capacity(255);
    super::pseudolegal::all::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
    );
    for move_item in move_list {
        let cloned = game.clone();
        let player = game.side_to_move;
        game.make_move(&move_item);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            let move_nodes = _perft(game, cache, depth - 1);
            nodes += move_nodes;
        }
        // replace with unset
        game.set(cloned);
    }

    return nodes;
}
