use crate::{
    moves::generator::precalculated_lookups::cache::PrecalculatedCache,
    state::{game::GameState, moves::MoveList},
    zobrist::ZobristHasher,
};

use std::time::Instant;

#[allow(dead_code)]
pub fn perft(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    keys: &ZobristHasher,
) -> u64 {
    let now = Instant::now();

    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    super::generator::movegen::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
        false,
    );
    for index in 0..move_list.len() {
        let move_item = &move_list.moves[index];

        let cloned = game.clone();
        let player = game.side_to_move;
        let _unmake_metadata = game.make_move(move_item, keys);
        // must do opponent since make move toggles opponents
        if !game.in_check(player, cache) {
            let move_nodes = _perft(game, cache, depth - 1, keys);
            nodes += move_nodes;
            println!("{}: {}", move_item.notation(), move_nodes);
        }
        // replace with unset
        // game.unmake_move(&move_item, unmake_metadata);
        game.set(cloned);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    return nodes;
}

#[allow(dead_code)]
fn _perft(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    keys: &ZobristHasher,
) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    super::generator::movegen::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
        false,
    );
    for index in 0..move_list.len() {
        let move_item = &move_list.moves[index];
        let cloned = game.clone();
        let player = game.side_to_move;
        let _unmake_metadata = game.make_move(move_item, keys);
        // must do opponent since make move toggles opponents
        if !game.in_check(player, cache) {
            let move_nodes = _perft(game, cache, depth - 1, keys);
            nodes += move_nodes;
        }
        // replace with unset
        game.set(cloned);
    }

    return nodes;
}

#[allow(dead_code)]
pub fn perft_unmake(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    keys: &ZobristHasher,
) -> u64 {
    let now = Instant::now();

    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    super::generator::movegen::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
        false,
    );
    println!("{:?}", move_list.len());
    for index in 0..move_list.len() {
        let move_item = &move_list.moves[index];
        // let cloned = game.clone();

        let player = game.side_to_move;
        let unmake_metadata = game.make_move(move_item, keys);
        // must do opponent since make move toggles opponents
        if !game.in_check(player, cache) {
            let move_nodes = _perft_unmake(game, cache, depth - 1, keys);
            nodes += move_nodes;
            println!("{}: {}", move_item.notation(), move_nodes);
        }
        // replace with unset
        game.unmake_move(&move_item, unmake_metadata, keys);
        // game.set(cloned);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    return nodes;
}

#[allow(dead_code)]
fn _perft_unmake(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    keys: &ZobristHasher,
) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    super::generator::movegen::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
        false,
    );
    for index in 0..move_list.len() {
        let move_item = &move_list.moves[index];
        let player = game.side_to_move;
        let unmake_metadata = game.make_move(move_item, keys);
        // must do opponent since make move toggles opponents
        if !game.in_check(player, cache) {
            let move_nodes = _perft_unmake(game, cache, depth - 1, keys);
            nodes += move_nodes;
        }
        // replace with unset
        game.unmake_move(&move_item, unmake_metadata, keys);
    }

    return nodes;
}
