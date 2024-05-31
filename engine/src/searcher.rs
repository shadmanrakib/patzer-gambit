use std::time::Instant;

use crate::{
    moves::{
        data::MoveItem,
        generator::{
            movegen::generate_pseudolegal_moves, precalculated_lookups::cache::PrecalculatedCache,
        },
    },
    movescoring::{score_captures, score_moves},
    mv::{MoveList, SimpleMove},
    perft,
    pieces::Piece,
    position::GameState,
    searchinfo::SearchInfo,
    settings::{FULL_DEPTH_MOVES, MAX_PLY, REDUCTION_LIMIT, TRANSITION_TABLE_ADDRESSING_BITS},
    time::{TeriminationStatus, TimeControl},
    transposition::{NodeType, TTable},
    zobrist::ZobristHasher,
};

pub const INF: i32 = std::i32::MAX;
pub const NEG_INF: i32 = -INF;
pub const STALEMATE: i32 = 0;
pub const CHECKMATE: i32 = NEG_INF;

pub struct Searcher {
    pub cache: PrecalculatedCache,
    pub zobrist: ZobristHasher,
    pub tt: TTable,
    pub position: GameState,
}

impl Searcher {
    pub fn new() -> Searcher {
        let zobrist = ZobristHasher::init();
        Searcher {
            cache: PrecalculatedCache::create(),
            position: GameState::new(&zobrist),
            zobrist,
            tt: TTable::init(TRANSITION_TABLE_ADDRESSING_BITS),
        }
    }
    pub fn startpos(&mut self) {
        self.position = GameState::new(&self.zobrist)
    }
    pub fn fen(&mut self, fen: String) -> Result<(), String> {
        self.position = GameState::from_fen(fen, &self.zobrist)?;
        Ok(())
    }
    pub fn make_move(&mut self, move_item: MoveItem) -> bool {
        self.position
            .make_move(move_item, &self.cache, &self.zobrist)
    }
    pub fn unmake_move(&mut self) {
        self.position.unmake_move(&self.zobrist)
    }
    pub fn make_null_move(&mut self) -> u64 {
        self.position.make_null_move(&self.zobrist)
    }
    pub fn unmake_null_move(&mut self, enpassant: u64) {
        self.position.unmake_null_move(enpassant);
    }
    pub fn play(&mut self, notation: String) -> bool {
        let m = self.position.notation_to_move(notation, &self.cache);
        if let Ok(move_item) = m {
            return self.make_move(move_item);
        }
        false
    }
    pub fn perft(&mut self, depth: u16) -> u64 {
        perft::perft(&mut self.position, &self.cache, depth, &self.zobrist)
    }

    pub fn get_pv(&mut self, depth: u8) -> Vec<SimpleMove> {
        let mut moves = vec![];
        let mut position = self.position.clone();

        let mut i = 0;
        while i < depth {
            i += 1;
            if let Some((simple_move, _, d)) = self.tt.probe(position.hash, INF, -INF) {
                if d > 0 {
                    let notation = simple_move.to_string();
                    moves.push(simple_move);

                    if d >= 1 {
                        if let Ok(move_item) =
                            position.notation_to_move(notation.clone(), &self.cache)
                        {
                            position.make_move(move_item, &self.cache, &self.zobrist);
                        } else {
                            println!("Not found {}", notation);
                        }
                        continue;
                    }
                }
                break;
            } else {
                break;
            }
        }

        moves
    }

    pub fn go(&mut self, main_search_depth: u8, time_control: TimeControl) -> Option<SimpleMove> {
        let mut best_move: Option<SimpleMove> = None;

        let mut depth = 1;
        let mut info = SearchInfo::init(time_control);

        let mut alpha = NEG_INF;
        let mut beta = INF;

        while depth <= main_search_depth {
            info.update_termination_status(
                self.position.side_to_move,
                depth,
                true,
                self.position.full_move_number,
            );
            if info.terimination_status == TeriminationStatus::Terminated {
                break;
            }

            let iter_start = Instant::now();

            info.new_iteration(); // get clear older iterations useless info/stats
            let score = self.negamax(depth, 0, MAX_PLY, alpha, beta, &mut info);

            let iter_elapsed = iter_start.elapsed();
            let ms = iter_elapsed.as_millis();
            let ns = iter_elapsed.as_nanos();

            let nps = (info.iteration_nodes as u128) * 10_u128.pow(9) / (ns + 1);

            let mut pv = self.get_pv(depth);
            if pv.len() > 0 {
                best_move = Some(pv[0]);
            } else if let Some(ref mv) = info.best_move {
                // this might be an value gotten from an interuption
                let smv = SimpleMove {
                    to: mv.to_pos,
                    from: mv.from_pos,
                    promotion: mv.promotion_piece,
                };
                best_move = Some(smv);
                pv = vec![smv];
            }

            let pv_str_vec: Vec<String> = pv.iter().map(|x| x.to_string()).collect();
            let pv_str = pv_str_vec.join(" ");

            let nodes = info.iteration_nodes;
            let seldepth = info.seldepth;
            println!("info score cp {score} depth {depth} seldepth {seldepth} time {ms} nodes {nodes} nps {nps} pv {pv_str}");

            // the aspiration window was a bad idea, lets reset it and try again
            if score <= alpha || score >= beta {
                alpha = NEG_INF;
                beta = INF;
                continue;
            }

            // otherwise we create next aspiration window
            alpha = score.saturating_sub(70);
            beta = score.saturating_add(70);

            // increase depth
            depth += 1;
        }

        // get occupancy and mark non-empty nodes as ancient so that they don't linger around
        for i in 0..self.tt.table.len() {
            let entry = &mut self.tt.table[i];
            if entry.best_move.to != entry.best_move.from {
                entry.ancient = true;
            }
        }

        if let Some(move_item) = &best_move {
            let short = move_item.to_string();
            println!("bestmove {short}");
        } else {
            println!("bestmove");
        }

        return best_move;
    }

    fn negamax(
        &mut self,
        mut depth: u8,
        ply: u8,
        max_ply: u8,
        mut alpha: i32,
        beta: i32,
        info: &mut SearchInfo,
    ) -> i32 {
        if info.total_nodes & info.check_termination_node_interval == 0
            || info.terimination_status == TeriminationStatus::Soon
        {
            info.update_termination_status(
                self.position.side_to_move,
                ply,
                false,
                self.position.full_move_number,
            );
        }

        info.maximize_seldepth(ply);
        info.increment_node_counts(false);

        let player = self.position.side_to_move;

        let mut node_type = NodeType::All;

        let mut tt_move = SimpleMove {
            from: 0,
            to: 0,
            promotion: Piece::King,
        };
        // handle base cases
        if self.position.has_insufficient_material() {
            return 0;
        }
        if let Some((simple_move, score, d)) = self.tt.probe(self.position.hash, alpha, beta) {
            if d >= depth {
                return score;
            };

            tt_move = simple_move;
        }
        if ply == max_ply || info.terimination_status == TeriminationStatus::Terminated {
            return self.position.score();
        }
        if depth <= 0 {
            return self.quiescence(ply, max_ply, alpha, beta, info);
        }

        let in_check = self
            .position
            .in_check(self.position.side_to_move, &self.cache);

        // null pruning
        if !in_check && depth >= 3 && ply != 0 && self.position.phase < 180
        // && standpat >= beta
        {
            let r = 2;
            let prev_enpassant = self.make_null_move();
            let null_score = -self.negamax(depth - 1 - r, ply + 1, max_ply, -beta, -beta + 1, info);
            self.unmake_null_move(prev_enpassant);

            if null_score >= beta {
                return beta;
            }
        }

        // check extension
        if in_check {
            depth += 1;
        }

        let mut best_move_idx: i32 = -1;
        let mut best_score: i32 = -INF;

        let mut moveslist = MoveList::new();
        generate_pseudolegal_moves(&mut moveslist, &self.position, player, &self.cache, false);
        score_moves(&mut moveslist, info, ply as usize, &tt_move);
        let mut legal_moves_count: u8 = 0;
        let mut moves_searched = 0;

        let mut best_draw = false;

        for index in 0..moveslist.len() {
            moveslist.sort_move(index);
            let move_item = moveslist.moves[index].clone();

            // illegal move
            if !self.make_move(move_item.clone()) {
                continue;
            }

            legal_moves_count += 1;

            let mut score: i32 = 0;

            let draw = self.position.has_three_fold_rep() || self.position.half_move_clock >= 50;
            if !draw {
                if moves_searched == 0 {
                    score = -self.negamax(depth - 1, ply + 1, max_ply, -beta, -alpha, info);
                } else {
                    if moves_searched >= FULL_DEPTH_MOVES
                        && depth >= REDUCTION_LIMIT
                        && !in_check
                        && !move_item.capturing
                        && !move_item.promoting
                        && !move_item.castling
                    {
                        score =
                            -self.negamax(depth - 2, ply + 1, max_ply, -(alpha + 1), -alpha, info);
                    } else {
                        score = alpha + 1; // done to trigger research
                    }

                    if score > alpha {
                        // like pvs
                        score =
                            -self.negamax(depth - 1, ply + 1, max_ply, -(alpha + 1), -alpha, info);
                        if score > alpha && score < beta {
                            score = -self.negamax(depth - 1, ply + 1, max_ply, -beta, -alpha, info);
                        }
                    }
                }
            }

            self.unmake_move();

            moves_searched += 1;

            // let interupted =
            //     info.controller
            //         .should_stop(true, player, info.total_nodes, ply, false);

            if score > best_score {
                best_score = score;
                best_move_idx = index as i32;
                if ply == 0 {
                    info.best_move = Some(move_item.clone());
                }
                best_draw = draw;
            }

            if score >= beta {
                if !move_item.capturing {
                    info.store_killer_move(&move_item, ply as usize);
                }

                self.tt.record(
                    self.position.hash,
                    (&move_item).into(),
                    score,
                    depth,
                    NodeType::Cut,
                    draw || info.terimination_status == TeriminationStatus::Terminated,
                    // interupted || draw,
                );

                return score;
            }

            if score > alpha {
                alpha = score;
                node_type = NodeType::Pv;
            }
        }

        if legal_moves_count == 0 {
            let score = if in_check {
                CHECKMATE + (ply as i32)
            } else {
                STALEMATE
            };
            return score;
        }

        self.tt.record(
            self.position.hash,
            (&moveslist.moves[(if best_move_idx >= 0 { best_move_idx } else { 0 }) as usize])
                .into(),
            best_score,
            depth,
            node_type,
            best_draw || info.terimination_status == TeriminationStatus::Terminated,
            // best_draw || interupted,
        );

        best_score
    }

    pub fn quiescence(
        &mut self,
        ply: u8,
        max_ply: u8,
        mut alpha: i32,
        beta: i32,
        info: &mut SearchInfo,
    ) -> i32 {
        if info.total_nodes & info.check_termination_node_interval == 0
            || info.terimination_status == TeriminationStatus::Soon
        {
            info.update_termination_status(
                self.position.side_to_move,
                ply,
                false,
                self.position.full_move_number,
            );
        }

        info.increment_node_counts(true);
        info.maximize_seldepth(ply);

        let player = self.position.side_to_move;
        let stand_pat = self.position.score();

        if ply == max_ply {
            return stand_pat;
        }

        // stand-pat here
        if stand_pat >= beta {
            return stand_pat;
        }

        // position better than alpha already
        if stand_pat > alpha {
            alpha = stand_pat
        }

        let mut moveslist = MoveList::new();
        generate_pseudolegal_moves(&mut moveslist, &self.position, player, &self.cache, true);
        score_captures(&mut moveslist);

        for i in 0..moveslist.len() {
            moveslist.sort_move(i);
            let move_item = moveslist.moves[i].clone();
            // illegal move
            if !self.make_move(move_item) {
                continue;
            }

            // The position is not yet quiet. Go one ply deeper.
            let score = -self.quiescence(ply + 1, max_ply, -beta, -alpha, info);

            self.unmake_move();

            // beta cutoff
            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }
}
