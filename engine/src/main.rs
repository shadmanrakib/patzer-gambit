mod constants;
mod fen;
mod moves;
mod state;
mod evaluation;
mod search;

#[cfg(test)]
mod tests;

use std::time::SystemTime;

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

    // let game_str = "2r3k1/p4p2/3Rp1qp/1p2P1pK/8/1P4P1/P3Q2P/8 w - - 1 2";
    // let game_str = "r2qkb1r/pp2NBp1/3p3p/2p1N1B1/3nP3/3P4/PPP2PPP/R2bK2R b KQkq - 0 1";
    // let game_str = "r2qkb1r/pp2nppp/3p4/2pNN1B1/2BnP3/3P4/PPP2PPP/R2bK2R w KQkq - 1 0";
    // let game_str = "r5rk/2p1Nppp/3p3P/pp2p1P1/4P3/2qnPQK1/8/R6R w - - 1 0";
    // let game_str = "r2n1rk1/1ppb2pp/1p1p4/3Ppq1n/2B3P1/2P4P/PP1N1P1K/R2Q1RN1 b - - 0 1";
    // let game_str = "6rk/7p/pp3b2/2pbqP2/5Q2/5R1P/P6P/2B2R1K b - - 0 1";
    let mut game = state::game::GameState::from_fen(game_str.to_string()).unwrap();
    // let a2a4 = game.notation_to_move("a2a4".into(), &cache).unwrap();
    // game.make_move(&a2a4);
    // let a7a6 = game.notation_to_move("a7a6".into(), &cache).unwrap();
    // game.make_move(&a7a6);
    // let a4a5 = game.notation_to_move("a4a5".into(), &cache).unwrap();
    // game.make_move(&a4a5);
    // let b7b5 = game.notation_to_move("b7b5".into(), &cache).unwrap();
    // game.make_move(&b7b5);
    // let c2c4 = game.notation_to_move("c2c4".into(), &cache).unwrap();
    // game.make_move(&c2c4);

    // position startpos moves a2a4 a7a6 a4a5 b7b5
    // let game = state::game::GameState::from_fen("r7/pPPppp2/p2P/p7/p7/4p/PppPpPPP/R2R4 w KQkq - 0 1".into()).unwrap();
    game.print_state();

    // let mut movelist = MoveList::new();
    let result = search::think::iterative_deepening(
        game.clone(),
        &true,
        &cache,
    );
}
