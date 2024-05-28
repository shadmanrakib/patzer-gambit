#[cfg(test)]
mod tests {
    use crate::{
        moves::{
            self,
            perft::{perft, perft_unmake},
        },
        search::zobrist::ZobristRandomKeys,
        state,
    };

    struct PerftTest {
        fen: String,
        depth: u16,
        expected_nodes: u64,
    }

    #[test]
    fn perft_suite() {
        // from https://www.chessprogramming.org/Perft_Results and using stockfish perft for expected
        println!("perft suite");
        let tests = [
            PerftTest {
                fen: "8/2p5/3p4/KP5r/4P2k/8/6p1/7R b - - 1 3".into(),
                depth: 2,
                expected_nodes: 62,
            },
            PerftTest {
                fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into(),
                depth: 6,
                expected_nodes: 119_060_324,
            },
            PerftTest {
                fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".into(),
                depth: 5,
                expected_nodes: 193_690_690,
            },
            PerftTest {
                fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".into(),
                depth: 7,
                expected_nodes: 178_633_661,
            },
            PerftTest {
                fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".into(),
                depth: 5,
                expected_nodes: 15_833_292,
            },
            PerftTest {
                fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10"
                    .into(),
                depth: 5,
                expected_nodes: 164_075_551,
            },
        ];

        let cache = moves::generator::precalculated_lookups::cache::PrecalculatedCache::create();
        let keys = ZobristRandomKeys::init();
        
        for test in tests {
            let mut game = state::game::GameState::from_fen(test.fen.to_string(), &keys).unwrap();
            println!("Test {}", &test.fen);
            let nodes = perft_unmake(&mut game, &cache, test.depth, &keys);
            println!("Found: {}\tExpected: {}", nodes, test.expected_nodes);
            perft(&mut game, &cache, test.depth, &keys);
            assert_eq!(nodes, test.expected_nodes);
        }
    }
}
