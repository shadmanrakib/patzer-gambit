use crate::{
    moves::generator::precalculated_lookups::cache::PrecalculatedCache,
    state::{game::GameState, moves::MoveList},
    zobrist::ZobristHasher,
};

use std::time::Instant;

pub fn perft(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    zobrist: &ZobristHasher,
) -> u64 {
    let now = Instant::now();

    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    crate::moves::generator::movegen::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
        false,
    );

    for index in 0..move_list.len() {
        let move_item = &move_list.moves[index];
        if game.make_move(move_item.clone(), cache, zobrist) {
            let move_nodes = _perft(game, cache, depth - 1, zobrist);
            nodes += move_nodes;
            println!("{}: {}", move_item.notation(), move_nodes);
            game.unmake_move(zobrist);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());
    println!("Total: {}", nodes);

    return nodes;
}

fn _perft(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    zobrist: &ZobristHasher,
) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    crate::moves::generator::movegen::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
        false,
    );

    for index in 0..move_list.len() {
        let move_item = &move_list.moves[index];
        if game.make_move(move_item.clone(), cache, zobrist) {
            let move_nodes = _perft(game, cache, depth - 1, zobrist);
            nodes += move_nodes;
            game.unmake_move(zobrist);
        }
    }

    return nodes;
}
