use crate::{movegen::MoveGenerator, position::PositionState, zobrist::ZobristHasher};

use std::time::Instant;

pub fn perft(
    position: &mut PositionState,
    generator: &MoveGenerator,
    depth: u16,
    zobrist: &ZobristHasher,
) -> u64 {
    let now = Instant::now();

    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let moves_list = generator.generate_moves(position);

    for index in 0..moves_list.len() {
        let move_item = &moves_list.moves[index];
        if position.make_move(move_item.clone(), generator, zobrist) {
            let move_nodes = _perft(position, generator, depth - 1, zobrist);
            nodes += move_nodes;
            println!("{}: {}", move_item.to_string(), move_nodes);
            position.unmake_move(zobrist);
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());
    println!("Total: {}", nodes);

    return nodes;
}

fn _perft(
    position: &mut PositionState,
    generator: &MoveGenerator,
    depth: u16,
    zobrist: &ZobristHasher,
) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let moves_list = generator.generate_moves(position);

    for index in 0..moves_list.len() {
        let move_item = &moves_list.moves[index];
        if position.make_move(move_item.clone(), generator, zobrist) {
            let move_nodes = _perft(position, generator, depth - 1, zobrist);
            nodes += move_nodes;
            position.unmake_move(zobrist);
        }
    }

    return nodes;
}
