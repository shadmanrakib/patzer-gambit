mod constants;
mod fen;
mod moves;
mod state;
mod evaluation;
mod search;

#[cfg(test)]
mod tests;

use std::time::SystemTime;

use moves::{attacked::in_check::is_in_check, precalculate::cache::PrecalculatedCache, pseudolegal::all::generate_pseudolegal_moves};
use state::{game::GameState, movelist::MoveList};

use crate::{
    moves::{
        move_data::MoveItem,
        perft::{perft, perft_unmake},
    },
    state::{boards::BitBoard, pieces::Piece, player::Player},
};

fn fixer(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    generate_pseudolegal_moves(
        &mut move_list,
        game,
        game.side_to_move,
        cache,
    );
    for index in 0..move_list.len() {
        let move_item = &move_list.moves[index];
        let player = game.side_to_move;
        let b4 = game.to_fen().to_string();
        let copy = game.clone();
        let unmake_metadata = game.make_move(move_item);
        // must do opponent since make move toggles opponents
        if !is_in_check(player, game, cache) {
            let move_nodes = fixer(game, cache, depth - 1);
            nodes += move_nodes;
        }
        let make = game.to_fen();
        // replace with unset
        game.unmake_move(&move_item, unmake_metadata);
        let after = game.to_fen().to_string();
        if b4 != after {
            // game.print_state();
            // let mut g = GameState::from_fen(b4.clone()).unwrap();
            // g.make_move(move_item);
            // g.print_state();
            println!("b4: {b4}");
            println!("make: {make}");
            println!("after: {after}");
            println!("{}", move_item.pure_algebraic_coordinate_notation());
            println!("{:?}", move_item);
        }

        if copy != *game {
            println!("{}", move_item.pure_algebraic_coordinate_notation());
            println!("{:?}", move_item);
            println!("copy: {:?}", copy);
            println!("game: {:?}", game);
            copy.bitboards.boards[1][Piece::Rook as usize].print_board();
            game.bitboards.boards[1][Piece::Rook as usize].print_board();
            panic!();
        }
    }

    return nodes;
}


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
    // let game_str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string();
    // let game_str: String = "1rb4r/pkPp3p/1b1P3n/1Q6/N3Pp2/8/P1P3PP/7K w - - 1 0".into();
    let game_str = "r3k2r/Pp1p1ppp/1bp2nbN/nPP5/BB2P3/q2P1N1P/P5P1/Rq1Q1RK1 b kq - 0 3".to_string();

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
    // fixer(&mut game, &cache, 6);

    // let mut movelist = MoveList::new();

    // moves::pseudolegal::all::generate_pseudolegal_moves(
    //     &mut movelist,
    //     &game,
    //     game.side_to_move,
    //     &cache,
    // );
    // for i in 0..movelist.len() {
    //     println!("Move {i}: {:?} {:?}", &movelist.moves[i].piece, &movelist.moves[i].pure_algebraic_coordinate_notation());
    //     let unmake = game.make_move(&movelist.moves[i]);
    //     game.unmake_move(&movelist.moves[i], unmake.clone());

        // if game_str != game.to_fen() {
        //     println!(
        //         "{i} {} =========================",
        //         &movelist.moves[i].pure_algebraic_coordinate_notation()
        //     );
        //     println!("{:?}", &unmake);
        //     game.print_state();
        //     game.bitboards.occupied.print_board();
        //     game.bitboards.boards[0][1].print_board();
        //     game.bitboards.pos_to_player[0].print_board();
        //     game.bitboards.pos_to_player[1].print_board();
        //     break;
        // }
    // }

    fixer(&mut game, &cache, 6);


    // println!("Perft (depth = {}): {}", 1, perft(&mut game, &cache, 1));
    // let depth = 5;
    // let result = search::minmax::iterative_deepening(
    //     game,
    //     &true,
    //     &cache,
    // );
    // println!("Move: {:?}", result.unwrap().pure_algebraic_coordinate_notation());

    // println!(
    //     "Perft Unmake (depth = {}): {:?}",
    //     depth,
    //     perft_unmake(&mut game, &cache, depth)
    // );
    // let fen_un: String = game.to_fen();
    // println!(
    //     "Perft (depth = {}): {:?}",
    //     depth,
    //     perft(&mut game, &cache, depth)
    // );
    // let fen_copy: String = game.to_fen();

    // println!("{}", fen_un);
    // println!("{}", fen_copy);



    // println!(
    //     "Perft (depth = {}): {:?}",
    //     depth,
    //     perft(&mut game, &cache, depth)
    // );

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
