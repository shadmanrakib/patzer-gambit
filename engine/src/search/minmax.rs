/*
   function alphabeta(node, depth, α, β, maximizingPlayer) is
       if depth == 0 or node is terminal then
           return the heuristic value of node
       if maximizingPlayer then
           value := −∞
           for each child of node do
               value := max(value, alphabeta(child, depth − 1, α, β, FALSE))
               α := max(α, value)
               if value ≥ β then
                   break (* β cutoff *)
           return value
       else
           value := +∞
           for each child of node do
               value := min(value, alphabeta(child, depth − 1, α, β, TRUE))
               β := min(β, value)
               if value ≤ α then
                   break (* α cutoff *)
           return value
*/

/*
function negamax(node, depth, α, β, color) is
    if depth = 0 or node is a terminal node then
        return color × the heuristic value of node

    childNodes := generateMoves(node)
    childNodes := orderMoves(childNodes)
    value := −∞
    foreach child in childNodes do
        value := max(value, −negamax(child, depth − 1, −β, −α, −color))
        α := max(α, value)
        if α ≥ β then
            break (* cut-off *)
    return value
*/

use crate::{
    evaluation,
    moves::{
        attacked::in_check::is_in_check, move_data::MoveItem,
        precalculate::cache::PrecalculatedCache, pseudolegal::all::generate_pseudolegal_moves,
    },
    state::{game::GameState, movelist::MoveList, pieces::Piece, player::Player},
};

pub fn iterative_deepening(
    mut game: GameState,
    has_time: &bool,
    cache: &PrecalculatedCache,
) -> Option<MoveItem> {
    const MAX_DEPTH: u8 = 5;
    let mut best_move: Option<MoveItem> = None;
    let mut depth = 0;
    while depth <= MAX_DEPTH && *has_time {
        println!("max depth {}", depth);
        let player = game.side_to_move;
        negamax(
            &mut game,
            depth,
            0,
            std::i32::MIN,
            std::i32::MAX,
            if player == Player::White {
                1
            } else {
                -1
            },
            &mut best_move,
            false,
            has_time,
            cache,
        );
        depth += 1;
    }
    return best_move;
}

pub fn negamax(
    game: &mut GameState,
    depth: u8,
    ply: u8,
    mut alpha: i32,
    mut beta: i32,
    color: i32,
    best_move: &mut Option<MoveItem>,
    last_move_captures: bool,
    has_time: &bool,
    cache: &PrecalculatedCache,
) -> i32 {
    /*
        if depth = 0 or node is a terminal node then
        return color × the heuristic value of node
    */

    let player = game.side_to_move;
    let in_check = is_in_check(player, &game, cache);

    let mut moveslist = MoveList::new();
    generate_pseudolegal_moves(&mut moveslist, &game, player, cache);
    let mut legal_moves_count: u8 = 0;

    if depth == 0 || !*has_time {
        return evaluation::position::simple(game, in_check, moveslist.len() > 0, color);
    }

    let mut value = std::i32::MIN;

    for index in 0..moveslist.len() {
        let move_item: &MoveItem = &moveslist.moves[index];

        let player = game.side_to_move;
        let copy = game.clone();
        let unmake_metadata = game.make_move(move_item);

        if !is_in_check(player, &game, cache) {
            if move_item.captured_piece == Piece::King {
                return std::i32::MAX;
            }

            legal_moves_count += 1;
            
            let score = -1
                * negamax(
                    game,
                    depth - 1,
                    ply + 1,
                    -beta,
                    -alpha,
                    -color,
                    best_move,
                    move_item.capturing,
                    has_time,
                    cache,
                );

            if score > value {
                value = score;
                if ply == 0 {
                    *best_move = Some(move_item.clone());
                    // println!("Move: {:?} {} WOO", move_item.pure_algebraic_coordinate_notation(), score);
                }
            }

            if ply == 0 {
                println!("Move: {:?} {}", move_item.pure_algebraic_coordinate_notation(), score);
            }

            alpha = std::cmp::max(alpha, value);

            // if alpha >= beta {
            //     break;
            // }
        }

        // game.unmake_move(&move_item, unmake_metadata);
        game.set(copy);
    }

    return value;
}
