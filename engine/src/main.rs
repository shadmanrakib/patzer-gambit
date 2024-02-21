mod constants;
mod fen;
mod moves;
mod state;

#[cfg(test)]
mod tests;

use std::time::SystemTime;

use crate::{
    moves::{
        move_data::MoveItem,
        perft::{perft, perft_unmake},
    },
    state::{boards::BitBoard, pieces::Piece, player::Player},
};

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
    // let game_str = "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1";
    // let game_str = "r3k2r/Pppp1ppp/1b3nbN/nPB5/B1P1P3/q4N2/Pp1P2PP/R2Q1RK1 b kq - 1 1";
    // let game_str = "r3k2r/Pppp1ppp/1b3nbN/nPq5/B1P1P3/5N2/Pp1P2PP/R2Q1RK1 w kq - 0 2";
    // let game_str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q2/1PPBBPpP/2R1K2R b Kkq a3 0 2";
    // let game_str = "rnbqkbnr/pppp1ppp/4p3/8/8/PP6/2PPPPPP/RNBQKBNR b KQkq - 0 2";
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
    // game.bitboards.boards[Player::White as usize][Piece::Pawn as usize].print_board();

    // let mut movelist: Vec<MoveItem> = vec![];
    // moves::pseudolegal::all::generate_pseudolegal_moves(
    //     &mut movelist,
    //     &game,
    //     game.side_to_move,
    //     &cache,
    // );

    // moves::pseudolegal::all::generate_pseudolegal_moves(
    //     &mut movelist,
    //     &game,
    //     game.side_to_move,
    //     &cache,
    // );
    // for i in 0..movelist.len() {
    //     let unmake = game.make_move(&movelist[i]);
    //     game.unmake_move(&movelist[i], unmake.clone());

    //     if game_str != game.to_fen() {
    //         println!(
    //             "{i} {} =========================",
    //             &movelist[i].pure_algebraic_coordinate_notation()
    //         );
    //         println!("{:?}", &unmake);
    //         game.print_state();
    //         game.bitboards.occupied.print_board();
    //         game.bitboards.boards[0][1].print_board();
    //         game.bitboards.pos_to_player[0].print_board();
    //         game.bitboards.pos_to_player[1].print_board();
    //         break;
    //     }
    // }

    // println!("Perft (depth = {}): {}", 1, perft(&mut game, &cache, 1));
    let depth = 6;
    println!(
        "Perft (depth = {}): {:?}",
        depth,
        perft_unmake(&mut game, &cache, depth)
    );

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
