use std::time::Instant;

use crate::{
    evaluation::psqt_tapered,
    moves::{self, generator::precalculated_lookups::cache::PrecalculatedCache},
    search::zobrist::{calculate_zobrist_hash, ZobristRandomKeys},
    state::{self, game::GameState, movelist::MoveList},
    utils::in_check::is_in_check,
};

struct IncTest {
    fen: String,
    depth: u16,
}

pub fn inc_test() {
    let cache = moves::generator::precalculated_lookups::cache::PrecalculatedCache::create();
    let keys = ZobristRandomKeys::init();

    let cases = [
        IncTest {
            fen: "8/2p5/3p4/KP5r/4P2k/8/6p1/7R b - - 1 3".into(),
            depth: 5,
        },
        IncTest {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into(),
            depth: 5,
        },
        IncTest {
            fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".into(),
            depth: 4,
        },
        IncTest {
            fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".into(),
            depth: 4,
        },
        IncTest {
            fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".into(),
            depth: 4,
        },
        IncTest {
            fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".into(),
            depth: 4,
        },
    ];

    println!("========= WITHOUT PRUNING =========");
    for case in &cases {
        println!("start case depth: {} \t fen: {}", case.depth, case.fen);
        let mut game = state::game::GameState::from_fen(case.fen.to_string(), &keys).unwrap();
        inc_test_fn(&mut game, &cache, case.depth, &keys);
        println!("finish case depth: {} \t fen: {}", case.depth, case.fen);
    }
}

fn inc_test_fn(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    keys: &ZobristRandomKeys,
) -> () {
    let now = Instant::now();

    if depth == 0 {
        return;
    }

    if game.hash != calculate_zobrist_hash(game, keys) {
        println!("Fails to incrementally update hash");
        panic!();
    }
    let (phase, opening, endgame) = psqt_tapered::init(game);
    if game.hash != calculate_zobrist_hash(game, keys) {
        println!("Fails to incrementally update hash");
        panic!();
    }
    if game.phase != phase
        || game.opening[0] != opening[0]
        || game.opening[1] != opening[1]
        || game.endgame[0] != endgame[0]
        || game.endgame[1] != endgame[1]
    {
        println!("Fails to incrementally update value");
        println!("static: p:{:?}\to:{:?}\te:{:?}", phase, opening, endgame);
        println!(
            "inc: p:{:?}\to:{:?}\te:{:?}",
            game.phase, game.opening, game.endgame
        );
        panic!();
    }

    let mut move_list = MoveList::new();
    crate::moves::generator::movegen::generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
        false,
    );
    // println!("{:?}", move_list.len());
    for index in 0..move_list.len() {
        let move_item = &move_list.moves[index];
        // let cloned = game.clone();

        let player = game.side_to_move;
        let unmake_metadata = game.make_move(move_item, keys);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            _inc_test(game, cache, depth - 1, keys);
        }
        // replace with unset
        game.unmake_move(&move_item, unmake_metadata, keys);
        // game.set(cloned);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms", elapsed.as_millis());

    return;
}

fn _inc_test(
    game: &mut GameState,
    cache: &PrecalculatedCache,
    depth: u16,
    keys: &ZobristRandomKeys,
) -> () {
    if depth == 0 {
        return;
    }

    if game.hash != calculate_zobrist_hash(game, keys) {
        println!("Fails to incrementally update hash");
        panic!();
    }
    let (phase, opening, endgame) = psqt_tapered::init(game);
    if game.hash != calculate_zobrist_hash(game, keys) {
        println!("Fails to incrementally update hash");
        panic!();
    }
    if {
        game.phase != phase
            || game.opening[0] != opening[0]
            || game.opening[1] != opening[1]
            || game.endgame[0] != endgame[0]
            || game.endgame[1] != endgame[1]
    } {
        println!(
            "Fails to incrementally update value. fen: {}",
            game.to_fen()
        );
        println!("static: p:{:?}\to:{:?}\te:{:?}", phase, opening, endgame);
        println!(
            "inc: p:{:?}\to:{:?}\te:{:?}",
            game.phase, game.opening, game.endgame
        );
        panic!();
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
        let player = game.side_to_move;
        let unmake_metadata = game.make_move(move_item, keys);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            _inc_test(game, cache, depth - 1, keys);
        }
        // replace with unset
        game.unmake_move(&move_item, unmake_metadata, keys);
    }

    return;
}
