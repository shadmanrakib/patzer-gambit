#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    };

    use crate::{searcher::Searcher, time::TimeControl};

    #[test]
    fn mate4s_suite() {
        // from https://wtharvey.com/m8n4.txt
        // let cache = moves::generator::precalculated_lookups::cache::PrecalculatedCache::create();
        // let mut tt = TTable::init(TRANSITION_TABLE_ADDRESSING_BITS);
        // let mut tt = TTable::init(TRANSITION_TABLE_ADDRESSING_BITS);

        let mate4s = [
            (
                "r5rk/2p1Nppp/3p3P/pp2p1P1/4P3/2qnPQK1/8/R6R w - - 1 0",
                "h6g7",
            ),
            (
                "1r2k1r1/pbppnp1p/1b3P2/8/Q7/B1PB1q2/P4PPP/3R2K1 w - - 1 0",
                "a4d7",
            ),
            (
                "Q7/p1p1q1pk/3p2rp/4n3/3bP3/7b/PP3PPK/R1B2R2 b - - 0 1",
                "h3g2",
            ),
            (
                "r1bqr3/ppp1B1kp/1b4p1/n2B4/3PQ1P1/2P5/P4P2/RN4K1 w - - 1 0",
                "e4e5",
            ),
            ("r1b3kr/3pR1p1/ppq4p/5P2/4Q3/B7/P5PP/5RK1 w - - 1 0", "e7g7"),
            ("2k4r/1r1q2pp/QBp2p2/1p6/8/8/P4PPP/2R3K1 w - - 1 0", "a6a8"),
            (
                "2r1r3/p3P1k1/1p1pR1Pp/n2q1P2/8/2p4P/P4Q2/1B3RK1 w - - 1 0",
                "f5f6",
            ),
            (
                "r1bk3r/pppq1ppp/5n2/4N1N1/2Bp4/Bn6/P4PPP/4R1K1 w - - 1 0",
                "e5f7",
            ),
            (
                "6kr/pp2r2p/n1p1PB1Q/2q5/2B4P/2N3p1/PPP3P1/7K w - - 1 0",
                "h6g7",
            ),
            (
                "r3k3/pbpqb1r1/1p2Q1p1/3pP1B1/3P4/3B4/PPP4P/5RK1 w - - 1 0",
                "d3g6",
            ),
            (
                "rnb3kr/ppp4p/3b3B/3Pp2n/2BP4/4KRp1/PPP3q1/RN1Q4 w - - 1 0",
                "f3f8",
            ),
            (
                "4r3/p4pkp/q7/3Bbb2/P2P1ppP/2N3n1/1PP2KPR/R1BQ4 b - - 0 1",
                "a6f1",
            ),
            (
                "2r2b2/p2q1P1p/3p2k1/4pNP1/4P1RQ/7K/2pr4/5R2 w - - 1 0",
                "h4h7",
            ),
            (
                "rnbk2r1/ppp2Q1p/8/1B1Pp1q1/8/2N3B1/PPP3P1/R5K1 w - - 1 0",
                "f7g8",
            ),
            (
                "r1bnk2r/pppp1ppp/1b4q1/4P3/2B1N3/Q1Pp1N2/P4PPP/R3R1K1 w - - 1 0",
                "e4f6",
            ),
            (
                "8/6pk/3pp2p/4p1nP/1P2P3/3P1rP1/4qPK1/2QN3R b - - 0 1",
                "f3g3",
            ),
            ("6rk/7p/pp3b2/2pbqP2/5Q2/5R1P/P6P/2B2R1K b - - 0 1", "e5e2"),
            (
                "r2r4/p1p2p1p/n5k1/1p5N/2p2R2/5N2/P1K3PP/5R2 w - - 1 0",
                "f4f6",
            ),
            (
                "r1qbr2k/1p2n1pp/3B1n2/2P1Np2/p4N2/PQ4P1/1P3P1P/3RR1K1 w - - 1 0",
                "e5f7",
            ),
            ("k1K5/7r/8/4B3/1RP5/8/8/8 w - - 1 0", "b4b8"),
        ];

        let stopped = Arc::new(AtomicBool::new(false));
        let searcher = Arc::new(Mutex::new(Searcher::new()));

        for (mate4_fen, mate4_ans) in mate4s {
            searcher.lock().unwrap().fen(mate4_fen.into()).unwrap();
            let s = stopped.clone();
            s.store(false, Ordering::SeqCst);
            let time = TimeControl::new(s);
            let result = searcher.lock().unwrap().go(10, time);
            println!("{}", mate4_fen);
            if let Some(m) = result {
                let ans = m.to_string();
                if mate4_ans == ans {
                    println!("Correct {}", ans);
                } else {
                    searcher.lock().unwrap().position.print_board();
                    println!("fail... expected: {}, got: {}", mate4_ans, ans);
                }
                assert_eq!(mate4_ans, ans);
            } else {
                println!("no moves");
            }
        }
    }
}
