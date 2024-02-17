mod constants;
mod moves;
mod state;

use std::time::SystemTime;

use crate::{
    moves::attacked::square_attacked::times_square_attacked,
    state::{bitboards::BitBoard, pieces::Piece, player::Player},
};

fn main() {
    // let game = state::game::GameState::from_fen(
    //     "2b5/4Bpbp/7r/p1Np4/2pP1P1P/5P1p/1k6/1B3R1K b - d3 0 13".into(),
    // )
    // .unwrap();
    // let game = state::game::GameState::from_fen("rnbqkbnr/8/8/8/8/8/8/RNBQKBNR w KQkq - 0 1".into()).unwrap();
    let game = state::game::GameState::from_fen("1pK5/1p1p4/b7/1PP5/1k4b1/1PP1b3/8/2R5 b - - 0 1".into())
        .unwrap();

    let start_time = SystemTime::now();
    let cache = moves::precalculate::cache::PrecalculatedCache::create();
    println!(
        "Pre-calculated cache create time: {} ms",
        SystemTime::now()
            .duration_since(start_time)
            .unwrap()
            .as_millis()
    );
    // let game = state::game::GameState::from_fen("r7/pPPppp2/p2P/p7/p7/4p/PppPpPPP/R2R4 w KQkq - 0 1".into()).unwrap();
    game.print_state();
    println!(
        "attacked: {}",
        times_square_attacked(
            game.bitboards
                .get_board_by_piece(Piece::King(Player::Black))
                .clone()
                .pop_mut(),
            Player::White,
            &game,
            &cache,
        )
    );
    // pawns_test(&game, &cache, state::player::Player::White);
    // knights_test(&game, &cache, state::player::Player::White);
    // king_test(&game, &cache, state::player::Player::White);
    // rooks_test(&game, &cache, state::player::Player::White);
    // bishops_test(&game, &cache, state::player::Player::White);
}

fn pawns_test(
    game: &state::game::GameState,
    _cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let (
        single_forward_non_promotion,
        single_forward_promotion,
        double_foreward,
        capture_non_promotion,
        capture_promotion,
    ) = moves::pseudolegal::pawn::generate_pawn_moves(&game, player);
    println!("pawn single_forward_non_promotion:");
    for m in single_forward_non_promotion {
        println!("{:?}", m);
    }
    println!("pawn single_forward_promotion:");
    for m in single_forward_promotion {
        println!("{:?}", m);
    }
    println!("pawn double_foreward:");
    for m in double_foreward {
        println!("{:?}", m);
    }
    println!("pawn capture_non_promotion:");
    for m in capture_non_promotion {
        println!("{:?}", m);
    }
    println!("pawn capture_promotion:");
    for m in capture_promotion {
        println!("{:?}", m);
    }
}

fn knights_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let (silents, captures) =
        moves::pseudolegal::knight::generate_knight_moves(&game, player, cache);

    println!("knight silents:");
    for m in silents {
        println!("{:?}", m);
    }
    println!("knight captures:");
    for m in captures {
        println!("{:?}", m);
    }
}

fn king_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let (silents, captures) = moves::pseudolegal::king::generate_king_moves(&game, player, cache);

    println!("king silents:");
    for m in silents {
        println!("{:?}", m);
    }
    println!("king captures:");
    for m in captures {
        println!("{:?}", m);
    }
}

fn bishops_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let (silents, captures) =
        moves::pseudolegal::bishop::generate_bishops_moves_on_the_fly(&game, player, cache);

    println!("bishop silents:");
    for m in silents {
        println!("{:?}", m);
    }
    println!("bishop captures:");
    for m in captures {
        println!("{:?}", m);
    }
}

fn rooks_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let (silents, captures) =
        moves::pseudolegal::rook::generate_rooks_moves_on_the_fly(&game, player, cache);

    println!("rook silents:");
    for m in silents {
        println!("{:?}", m);
    }
    println!("rook captures:");
    for m in captures {
        println!("{:?}", m);
    }
}

fn queens_test(
    game: &state::game::GameState,
    cache: &moves::precalculate::cache::PrecalculatedCache,
    player: state::player::Player,
) {
    let (silents, captures) =
        moves::pseudolegal::rook::generate_rooks_moves_on_the_fly(&game, player, cache);

    println!("queens silents:");
    for m in silents {
        println!("{:?}", m);
    }
    println!("queens captures:");
    for m in captures {
        println!("{:?}", m);
    }
}
