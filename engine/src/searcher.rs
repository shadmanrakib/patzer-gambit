use std::{ops::Deref, sync::Arc, time::Instant};

use crate::{
    constants::search::{MAX_PLY, TRANSITION_TABLE_SIZE},
    controller::Controller,
    moves::{data::MoveItem, generator::precalculated_lookups::cache::PrecalculatedCache},
    search::{
        cache::SearchCache, negamax::negamax, transposition::TTable, zobrist::ZobristRandomKeys,
    },
    state::game::GameState,
};

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;

pub struct Searcher {
    pub cache: PrecalculatedCache,
    pub zobrist: ZobristRandomKeys,
    pub tt: TTable,
    pub position: GameState,
}

impl Searcher {
    pub fn new() -> Searcher {
        let zobrist = ZobristRandomKeys::init();
        Searcher {
            cache: PrecalculatedCache::create(),
            position: GameState::new(&zobrist),
            zobrist,
            tt: TTable::init(TRANSITION_TABLE_SIZE),
        }
    }
    pub fn startpos(&mut self) {
        self.position = GameState::new(&self.zobrist)
    }
    pub fn fen(&mut self, fen: String) -> Result<(), String> {
        self.position = GameState::from_fen(fen, &self.zobrist)?;
        Ok(())
    }
    pub fn make_move(&mut self, move_item: &MoveItem) {
        self.position.make_move(move_item, &self.zobrist);
    }

    pub fn get_pv(&mut self) -> Vec<String> {
        let mut moves = vec![];
        let mut position = self.position.clone();

        while let Some((simple_move, _, d)) = self.tt.probe(position.hash, INF, -INF) {
            if d > 0 {
                let notation = simple_move.to_string();
                moves.push(notation.clone());
                if d >= 1 {
                    let move_item = position.notation_to_move(notation, &self.cache).unwrap();
                    position.make_move(&move_item, &self.zobrist);
                    continue;
                }
            }
            break;
        }

        moves
    }

    pub fn go(
        &mut self,
        main_search_depth: u8,
        controller: Arc<dyn Controller>,
    ) -> Option<MoveItem> {
        let mut best_move: Option<MoveItem> = None;

        let mut depth = 1;
        let mut search_cache = SearchCache::init();

        let mut alpha = NEG_INF;
        let mut beta = INF;

        while depth <= main_search_depth && !controller.has_terminated() {
            let iter_start = Instant::now();

            let mut nodes = 0;
            let mut q_nodes = 0;
            let mut seldepth = 0;

            let result = negamax(
                &mut self.position,
                depth,
                0,
                MAX_PLY,
                alpha,
                beta,
                &self.cache,
                &mut search_cache,
                &mut nodes,
                &mut q_nodes,
                &self.zobrist,
                &mut self.tt,
                &mut best_move,
                &mut seldepth,
                controller.clone(),
            );

            let score = result.score;

            let iter_elapsed = iter_start.elapsed();
            let ms = iter_elapsed.as_millis();
            let ns = iter_elapsed.as_nanos();

            let nps = (nodes as u128) * 10_u128.pow(9) / (ns + 1);

            let total_nps = (nodes + q_nodes) * 10_u128.pow(9) / (ns + 1);

            if let Some(m) = &best_move {
                // println!("info currmove {short} depth {depth} seldepth {seldepth} score cp {score} time {ms} nodes {nodes} nps {nps} qnodes {q_nodes} tnps {total_nps}");
                // println!("info score cp {score} depth {depth} seldepth {seldepth} time {ms} nodes {nodes} nps {nps} qnodes {q_nodes} tnps {total_nps} pv {short}");
                let pv = self.get_pv().join(" ");
                println!("info score cp {score} depth {depth} seldepth {seldepth} time {ms} nodes {nodes} nps {total_nps} pv {pv}");
            } else {
                println!("info depth {depth} score cp {score} seldepth {seldepth} time {ms} nodes {nodes} nps {nps} qnodes {q_nodes} tnps {total_nps}");
            }

            // if score == INF || score == NEG_INF {
            //     break;
            // }

            // the aspiration window was a bad idea, lets retry without it
            if score <= alpha || score >= beta {
                alpha = NEG_INF;
                beta = INF;
                println!("retrying");
                continue;
            }

            alpha = score.saturating_sub(70);
            beta = score.saturating_add(70);
            depth += 1;
        }

        // println!("pv: {:?}", pv);

        if let Some(move_item) = &best_move {
            let short = move_item.notation();
            println!("bestmove {short}");
        }

        return best_move;
    }
}
