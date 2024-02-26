use super::{
    attacked::in_check::is_in_check, move_data::MoveItem, precalculate::cache::PrecalculatedCache,
};
use crate::state::game::GameState;

use std::time::Instant;

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
        let unmake_metadata = game.make_move(&move_item);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            let move_nodes = _perft(game, cache, depth - 1);
            nodes += move_nodes;
            println!(
                "{}: {}",
                move_item.pure_algebraic_coordinate_notation(),
                move_nodes
            );
        }
        // replace with unset
        // game.unmake_move(&move_item, unmake_metadata);
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
        let unmake_metadata = game.make_move(&move_item);
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

pub fn perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
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
    println!("{:?}", move_list.len());
    for move_item in move_list {
        // let cloned = game.clone();

        let player = game.side_to_move;
        let unmake_metadata = game.make_move(&move_item);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            let move_nodes = _perft_unmake(game, cache, depth - 1);
            nodes += move_nodes;
            println!(
                "{}: {}",
                move_item.pure_algebraic_coordinate_notation(),
                move_nodes
            );
        }
        // replace with unset
        game.unmake_move(&move_item, unmake_metadata);
        // game.set(cloned);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    return nodes;
}

fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
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
        let player = game.side_to_move;
        let unmake_metadata = game.make_move(&move_item);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            let move_nodes = _perft_unmake(game, cache, depth - 1);
            nodes += move_nodes;
        }
        // replace with unset
        game.unmake_move(&move_item, unmake_metadata);
    }

    return nodes;
}
