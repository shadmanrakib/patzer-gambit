mod constants;
mod evaluation;
mod fen;
mod moves;
mod search;
mod state;
mod utils;

#[cfg(test)]
mod tests;

use std::time::SystemTime;

use crate::{
    constants::search::{MAX_MAIN_SEARCH_DEPTH, TRANSITION_TABLE_ADDRESSING_BITS},
    moves::data::{MoveItem, UnmakeMoveMetadata},
    search::{transposition::TTable, zobrist::ZobristRandomKeys},
};

fn main() {
    let game_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let game_str = "r7/8/8/8/8/8/8/8 w KQkq - 0 1";

    // let game_str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    // let game_str = "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9";

    // let game_str = "r5rk/2p1Nppp/3p3P/pp2p1P1/4P3/2qnPQK1/8/R6R w - - 1 0";

    let start_time = SystemTime::now();
    let cache = moves::generator::precalculated_lookups::cache::PrecalculatedCache::create();
    println!(
        "Pre-calculated cache create time: {} ms",
        SystemTime::now()
            .duration_since(start_time)
            .unwrap()
            .as_millis()
    );

    // let game_str = "3r1r1k/p2q1p1n/1p1N3p/2p1bp1n/P2p3N/8/1PPQ1B1P/1K1R2R1 w - - 1 0";
    // let game_str = "r3k3/8/8/3N4/8/5q2/3B1Q1P/4K3 w - - 1 0";
    // let game_str = "r3k2q/8/8/3N4/8/8/3B1Q1P/4K3 w - - 1 0";
    // let game_str = "r3k3/8/8/3N4/8/8/3B4/4K3 w - - 1 0";
    // let game_str = "r3k3/8/8/3N4/8/8/3B4/5K2 b - - 1 0";
    // let game_str = "2qrb1k1/1p2bppp/1n2p3/p3P3/3NN3/1P4Q1/PB3PPP/3R2K1 b - - 0 1";
    let keys = ZobristRandomKeys::init();
    let mut game = state::game::GameState::from_fen(game_str.to_string(), &keys).unwrap();
    game.print_state();
    println!("{:?} {}", game.opening, game.hash);
    let mut tt = TTable::init(TRANSITION_TABLE_ADDRESSING_BITS);

    let mut halfs = 0;
    let mut undos: Vec<(MoveItem, UnmakeMoveMetadata)> = vec![];

    loop {
        // break;
        println!("Move: ");
        let mut notation = String::new();
        std::io::stdin().read_line(&mut notation).unwrap();
        notation = notation.trim().to_string();
        if notation == "skip" {
            continue;
        }
        if notation == "d" {
            game.print_state();
        } else if notation == "think" {
            println!("thinking...");
            let result = search::think::iterative_deepening(
                game.clone(),
                &true,
                &cache,
                MAX_MAIN_SEARCH_DEPTH,
                &keys,
                &mut tt,
            );
            if let Some(m) = result {
                println!("{}", m.notation());
            } else {
                println!("halfs: {halfs}");
                println!("no moves gg");
            }
        } else if notation == "play" {
            println!("thinking...");
            let result = search::think::iterative_deepening(
                game.clone(),
                &true,
                &cache,
                MAX_MAIN_SEARCH_DEPTH,
                &keys,
                &mut tt,
            );
            if let Some(m) = result {
                let unmake_metadata = game.make_move(&m, &keys);
                undos.push((m.clone(), unmake_metadata));
                println!("Played {}", m.notation());
            } else {
                println!("halfs: {halfs}");
                println!("no moves gg");
            }
        } else if notation == "undo" {
            if undos.len() > 0 {
                let (move_item, unmake_metadata) = undos.pop().unwrap();
                game.unmake_move(&move_item, unmake_metadata, &keys);
            } else {
                println!("nothing to undo")
            }
        } else {
            halfs += 1;
            let m = game.notation_to_move(notation, &cache);
            if let Ok(move_item) = m {
                let unmake_metadata = game.make_move(&move_item, &keys);
                undos.push((move_item, unmake_metadata));
            } else {
                println!("not found");
            }
        }
    }
}
