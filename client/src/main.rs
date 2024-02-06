use crate::core::fen::FenNotation;


mod utils;
mod core;

fn main() {
   
    let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen = FenNotation::new(fen_str);
    println!("{:?}", fen);
    println!("{:?}", fen.get_board_vec());

}



