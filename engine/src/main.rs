mod constants;
mod fen;
mod moves;
mod state;

use std::time::SystemTime;

use crate::moves::perft::perft;

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
    // let game_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game_str = "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1";
    let mut game = state::game::GameState::from_fen(game_str.into()).unwrap();

    let start_time = SystemTime::now();
    let cache = moves::precalculate::cache::PrecalculatedCache::create();
    println!(
        "Pre-calculated cache create time: {} ms",
        SystemTime::now()
            .duration_since(start_time)
            .unwrap()
            .as_millis()
    );
    // position startpos moves a2a4 a7a6 a4a5 b7b5
    // let game = state::game::GameState::from_fen("r7/pPPppp2/p2P/p7/p7/4p/PppPpPPP/R2R4 w KQkq - 0 1".into()).unwrap();
    game.print_state();

    // println!("Perft (depth = {}): {}", 1, perft(&mut game, &cache, 1));
    let depth = 6;
    println!(
        "Perft (depth = {}): {:?}",
        depth,
        perft(&mut game, &cache, depth, true)
    );

    // let moves = moves::pseudolegal::pawn::generate_pawn_moves(&game, game.side_to_move);
    // let m = &moves[0];
    // println!("{} {:?}", m.enpassant, m.pure_algebraic_coordinate_notation());
    // game.make_move(m);
    // game.print_board();



    // pawns_test(&game, &cache, state::player::Player::White);
    // knights_test(&game, &cache, state::player::Player::White);
    // king_test(&game, &cache, state::player::Player::White);
    // rooks_test(&game, &cache, state::player::Player::White);
    // bishops_test(&game, &cache, state::player::Player::White);
    // queens_test(&game, &cache, state::player::Player::White);
    // castling_test(&game, &cache, state::player::Player::White);

    // less: a2a2, c2c2, e2e3, h2h3, e2e4, a2a4, h2h4
    // extra: f2f3, c2c4, f2f4
}

fn pawns_test(
    game: &state::game::GameState,
    _cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let moves = moves::pseudolegal::pawn::generate_pawn_moves(&game, player);
    println!("pawn moves:");
    for m in moves {
        println!("{:?}", m);
    }
}

fn knights_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let moves = moves::pseudolegal::knight::generate_knight_moves(&game, player, cache);

    println!("knight moves:");
    for m in moves {
        println!("{:?}", m);
    }
}

fn king_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let moves = moves::pseudolegal::king::generate_king_moves(&game, player, cache);

    println!("king moves:");
    for m in moves {
        println!("{:?}", m);
    }
}

fn bishops_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let moves = moves::pseudolegal::bishop::generate_bishop_moves(&game, player, cache);

    println!("bishop moves:");
    for m in moves {
        println!("{:?}", m);
    }
}

fn rooks_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let moves = moves::pseudolegal::rook::generate_rook_moves(&game, player, cache);

    println!("rook moves:");
    for m in moves {
        println!("{:?}", m);
    }
}

fn queens_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let moves = moves::pseudolegal::queen::generate_queen_moves(&game, player, cache);

    println!("queens moves:");
    for m in moves {
        println!("{:?}", m);
    }
}

fn castling_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let moves = moves::pseudolegal::castling::generate_castling_moves(&game, player, cache);

    println!("castling moves:");
    for m in moves {
        println!("{:?}", m);
    }
}
