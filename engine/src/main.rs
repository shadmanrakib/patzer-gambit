mod constants;
mod evaluation;
mod fen;
mod moves;
mod search;
mod state;

#[cfg(test)]
mod tests;

use std::time::SystemTime;

use crate::moves::move_data::{MoveItem, UnmakeMoveMetadata};

fn main() {
    let game_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let start_time = SystemTime::now();
    let cache = moves::precalculate::cache::PrecalculatedCache::create();
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

    let mut game = state::game::GameState::from_fen(game_str.to_string()).unwrap();
    game.print_state();
    println!("{:?}", game.opening);
    // let result = search::think::iterative_deepening(game.clone(), &true, &cache, MAX_MAIN_SEARCH_DEPTH);

    let mut halfs = 0;
    let mut undos: Vec<(MoveItem, UnmakeMoveMetadata)> = vec![];

    loop {
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
            let result = search::think::iterative_deepening(game.clone(), &true, &cache, 9);
            if let Some(m) = result {
                println!("{}", m.pure_algebraic_coordinate_notation());
            } else {
                println!("halfs: {halfs}");
                println!("no moves gg");
            }
        } else if notation == "play" {
            println!("thinking...");
            let result = search::think::iterative_deepening(game.clone(), &true, &cache, 9);
            if let Some(m) = result {
                let unmake_metadata = game.make_move(&m);
                undos.push((m.clone(), unmake_metadata));
                println!("Played {}", m.pure_algebraic_coordinate_notation());
            } else {
                println!("halfs: {halfs}");
                println!("no moves gg");
            }
        } else if notation == "undo" {
            if undos.len() > 0 {
                let (move_item, unmake_metadata) = undos.pop().unwrap();
                game.unmake_move(&move_item, unmake_metadata);
            } else {
                println!("nothing to undo")
            }
        } else {
            halfs += 1;
            let m = game.notation_to_move(notation, &cache);
            if let Ok(move_item) = m {
                let unmake_metadata = game.make_move(&move_item);
                undos.push((move_item, unmake_metadata));
            } else {
                println!("not found");
            }
        }
    }
}
