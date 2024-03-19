mod constants;
mod evaluation;
mod fen;
mod moves;
mod search;
mod state;

#[cfg(test)]
mod tests;

use std::time::SystemTime;

use crate::moves::perft::perft_unmake;

fn main() {
    // let game = state::game::GameState::from_fen(
    //     "2b5/4Bpbp/7r/p1Np4/2pP1P1P/5P1p/1k6/1B3R1K b - d3 0 13".into(),
    // )
    // .unwrap();
    // let mut game = state::game::GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()).unwrap();
    // let mut game = state::game::GameState::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPnPPP/R3K2R w KQkq - 0 1".into()).unwrap();

    // "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
    // let game = state::game::GameState::from_fen("1pK5/1p1p4/b7/1PP5/1k4b1/1PP1b3/8/2R5 b - - 0 1".into())
    //     .unwrap();
    // let game = state::game::GameState::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".into())
    // .unwrap();

    // let mut game = state::game::GameState::from_fen(
    //     "rnbqkbnr/1ppppppp/p7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 1".into(),
    // )
    // .unwrap();
    // let game_str = "rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 3";
    // let game_str = "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1";

    let game_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    // let game_str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string();
    // let game_str: String = "1rb4r/pkPp3p/1b1P3n/1Q6/N3Pp2/8/P1P3PP/7K w - - 1 0".into();
    // let game_str = "r3k2r/Pp1p1ppp/1bp2nbN/nPP5/BB2P3/q2P1N1P/P5P1/Rq1Q1RK1 b kq - 0 3".to_string();
    // let game_str = "2r3k1/p4p2/3Rp2p/1p2P1pK/8/1P4P1/P3Q2P/1q6 b - - 0 1".to_string();
    // let game_str = "r2qkb1r/pp2nppp/3p4/2pNN1B1/2BnP3/3P4/PPP2PPP/R2bK2R w KQkq - 1 0";

    // let game_str = "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1";
    // let game_str = "r3k2r/Pppp1ppp/1b3nbN/nPB5/B1P1P3/q4N2/Pp1P2PP/R2Q1RK1 b kq - 1 1";
    // let game_str = "r3k2r/Pppp1ppp/1b3nbN/nPq5/B1P1P3/5N2/Pp1P2PP/R2Q1RK1 w kq - 0 2";
    // let game_str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q2/1PPBBPpP/2R1K2R b Kkq a3 0 2";
    // let game_str = "rnbqkbnr/pppp1ppp/4p3/8/8/PP6/2PPPPPP/RNBQKBNR b KQkq - 0 2";

    let start_time = SystemTime::now();
    let cache = moves::precalculate::cache::PrecalculatedCache::create();
    println!(
        "Pre-calculated cache create time: {} ms",
        SystemTime::now()
            .duration_since(start_time)
            .unwrap()
            .as_millis()
    );

    let mut game = state::game::GameState::from_fen(game_str.to_string()).unwrap();

    // let mut movelist = MoveList::new();
    let mut halfs = 0;
    while false {
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
            let result = search::think::iterative_deepening(game.clone(), &true, &cache);
            if let Some(m) = result {
                println!("{}", m.pure_algebraic_coordinate_notation());
            } else {
                println!("halfs: {halfs}");
                println!("no moves gg");
            }
        } else {
            halfs += 1;
            let m = game.notation_to_move(notation, &cache);
            if let Ok(move_item) = m {
                game.make_move(&move_item);
            } else {
                println!("not found");
            }
        }
    }
}
