use super::{attacked::in_check::is_in_check, precalculate::cache::PrecalculatedCache};
use crate::state::game::GameState;

pub fn perft(game: &mut GameState, cache: &PrecalculatedCache, depth: u16, print_individual_move_nodes: bool) -> (u64, u64, u64, u64) {
    let mut nodes = 0;
    let mut captures = 0;
    let mut castling = 0;
    let mut checks = 0;
    let mut legal: i32 = 0;


    if depth == 0 {
        return (1, 0, 0, 0);
    }

    let moves = super::pseudolegal::all::generate_pseudolegal_moves(game, game.side_to_move, cache);
    for move_item in moves {
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

            let move_nodes = perft(game, cache, depth - 1, false);
            nodes += move_nodes.0;
            captures += move_nodes.1;
            castling += move_nodes.2;
            checks += move_nodes.3;

            if print_individual_move_nodes {
                println!("{}: {}", move_item.pure_algebraic_coordinate_notation(), move_nodes.0);
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
