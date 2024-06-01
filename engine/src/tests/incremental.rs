#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{
        evaluation,
        movegen::MoveGenerator,
        mv::MoveList,
        position::PositionState,
        zobrist::ZobristHasher,
    };

    struct IncTest {
        fen: String,
        depth: u16,
    }

    #[test]
    pub fn inc_test() {
        let generator = MoveGenerator::create();
        let keys = ZobristHasher::init();

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
                fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"
                    .into(),
                depth: 4,
            },
        ];

        println!("========= WITHOUT PRUNING =========");
        for case in &cases {
            println!("start case depth: {} \t fen: {}", case.depth, case.fen);
            let mut position = PositionState::from_fen(case.fen.to_string(), &keys).unwrap();
            inc_test_fn(&mut position, &generator, case.depth, &keys);
            println!("finish case depth: {} \t fen: {}", case.depth, case.fen);
        }
    }

    fn inc_test_fn(
        position: &mut PositionState,
        generator: &MoveGenerator,
        depth: u16,
        zobrist: &ZobristHasher,
    ) -> () {
        let now = Instant::now();

        if depth == 0 {
            return;
        }

        let mut moves_list = generator.generate_moves(position);
        for index in 0..moves_list.len() {
            let move_item = moves_list.moves[index].clone();
            if position.make_move(move_item, generator, zobrist) {
                _inc_test(position, generator, depth - 1, zobrist);
                position.unmake_move(zobrist);
            }
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {} ms", elapsed.as_millis());
    }

    fn _inc_test(
        position: &mut PositionState,
        generator: &MoveGenerator,
        depth: u16,
        zobrist: &ZobristHasher,
    ) -> () {
        if position.hash != zobrist.hash(position) {
            println!("Fails to incrementally update hash");
            panic!();
        }
        let (phase, opening, endposition) = evaluation::init(position);
        if position.hash != zobrist.hash(position) {
            println!("Fails to incrementally update hash");
            panic!();
        }
        if {
            position.phase != phase
                || position.opening[0] != opening[0]
                || position.opening[1] != opening[1]
                || position.endposition[0] != endposition[0]
                || position.endposition[1] != endposition[1]
        } {
            println!(
                "Fails to incrementally update value. fen: {}",
                position.to_fen()
            );
            println!("static: p:{:?}\to:{:?}\te:{:?}", phase, opening, endposition);
            println!(
                "inc: p:{:?}\to:{:?}\te:{:?}",
                position.phase, position.opening, position.endposition
            );
            panic!();
        }

        if depth == 0 {
            return;
        }

        let moves_list = generator.generate_moves(position);
        for index in 0..moves_list.len() {
            let move_item = moves_list.moves[index].clone();
            if position.make_move(move_item, generator, zobrist) {
                _inc_test(position, generator, depth - 1, zobrist);
                position.unmake_move(zobrist);
            }
        }
    }
}
