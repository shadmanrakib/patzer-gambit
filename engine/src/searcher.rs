use std::time::Instant;

use crate::{
    movegen::MoveGenerator,
    moves::{Move, SimpleMove},
    perft,
    position::PositionState,
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
    pub generator: MoveGenerator,
    pub zobrist: ZobristHasher,
    pub tt: TTable,
    pub position: PositionState,
}

impl Searcher {
    pub fn new() -> Searcher {
        let zobrist = ZobristHasher::init();
        Searcher {
            generator: MoveGenerator::create(),
            position: PositionState::new(&zobrist),
            zobrist,
            tt: TTable::init(TRANSITION_TABLE_ADDRESSING_BITS),
        }
    }
    pub fn startpos(&mut self) {
        self.position = PositionState::new(&self.zobrist)
    }
    pub fn fen(&mut self, fen: String) -> Result<(), String> {
        self.position = PositionState::from_fen(fen, &self.zobrist)?;
        Ok(())
    }
    pub fn make_move(&mut self, move_item: Move) -> bool {
        self.position
            .make_move(move_item, &self.generator, &self.zobrist)
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
        let m = self.position.notation_to_move(notation, &self.generator);
        if let Ok(move_item) = m {
            return self.make_move(move_item);
        }
        false
    }
    pub fn perft(&mut self, depth: u16) -> u64 {
        perft::perft(&mut self.position, &self.generator, depth, &self.zobrist)
    }

    pub fn get_pv(&mut self, depth: u8) -> Vec<SimpleMove> {
        let mut moves = vec![];
        let mut position = self.position.clone();

        let mut i = 0;
        while i < depth {
            i += 1;
            if let (tt_move, Some(_)) = self.tt.probe(position.hash, 1, INF, -INF) {
                let notation = tt_move.to_string();
                moves.push(tt_move);

                if tt_move != SimpleMove::NULL_MOVE {
                    if let Ok(move_item) =
                        position.notation_to_move(notation.clone(), &self.generator)
                    {
                        position.make_move(move_item, &self.generator, &self.zobrist);
                    }
                    continue;
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

        let alpha = NEG_INF;
        let beta = INF;

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
            let score = self.negamax(depth, 0, MAX_PLY, alpha, beta, &mut info, false);

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

            let is_checkmate =
                CHECKMATE.abs() - 256 <= score.abs() && score.abs() <= CHECKMATE.abs();
            if is_checkmate {
                // we have a mate
                let sign = if score.is_negative() { -1 } else { 1 };
                let mate_diff = CHECKMATE.abs() - score.abs();
                let mate_in = (mate_diff / 2 + mate_diff % 2) * (sign);
                println!("info score mate {mate_in} depth {depth} seldepth {seldepth} time {ms} nodes {nodes} nps {nps} pv {pv_str}");
            } else {
                // regular score
                println!("info score cp {score} depth {depth} seldepth {seldepth} time {ms} nodes {nodes} nps {nps} pv {pv_str}");
            }

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
        can_null_prune: bool,
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

        let mut node_type = NodeType::All;

        // handle base cases

        // insufficient material
        if self.position.has_insufficient_material() {
            return 0;
        }

        // table might have useful info that we can use
        let (tt_move, tt_score) = self.tt.probe(self.position.hash, depth, alpha, beta);
        if let Some(score) = tt_score {
            return score;
        }

        // we need to end
        if ply == max_ply || info.terimination_status == TeriminationStatus::Terminated {
            return self.position.score();
        }

        if depth <= 0 {
            return self.quiescence(ply, max_ply, alpha, beta, info);
        }

        let in_check = self
            .position
            .in_check(self.position.side_to_move, &self.generator);

        // null pruning
        if can_null_prune && !in_check && depth >= 3 && self.position.phase < 180 {
            let r = 2;
            let prev_enpassant = self.make_null_move();
            let null_score = -self.negamax(
                depth - 1 - r,
                ply + 1,
                max_ply,
                -beta,
                -beta + 1,
                info,
                false,
            );
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

        let mut moves_list = self.generator.generate_moves(&self.position);
        moves_list.score_moves(info, ply as usize, &tt_move);

        let mut legal_moves_count: u16 = 0;

        let mut best_draw = false;

        for index in 0..moves_list.len() {
            moves_list.sort_move(index);
            let move_item = moves_list.moves[index].clone();

            // illegal move
            if !self.make_move(move_item.clone()) {
                continue;
            }

            let mut score: i32 = 0;

            let draw = self.position.has_three_fold_rep() || self.position.half_move_clock >= 50;
            if !draw {
                if legal_moves_count == 0 {
                    score = -self.negamax(depth - 1, ply + 1, max_ply, -beta, -alpha, info, false);
                } else {
                    if legal_moves_count >= FULL_DEPTH_MOVES
                        && depth >= REDUCTION_LIMIT
                        && !in_check
                        && !move_item.capturing
                        && !move_item.promoting
                        && !move_item.castling
                    {
                        score = -self.negamax(
                            depth - 2,
                            ply + 1,
                            max_ply,
                            -(alpha + 1),
                            -alpha,
                            info,
                            true,
                        );
                    } else {
                        score = alpha + 1; // done to trigger research
                    }

                    if score > alpha {
                        // like pvs
                        score = -self.negamax(
                            depth - 1,
                            ply + 1,
                            max_ply,
                            -(alpha + 1),
                            -alpha,
                            info,
                            true,
                        );
                        if score > alpha && score < beta {
                            score = -self.negamax(
                                depth - 1,
                                ply + 1,
                                max_ply,
                                -beta,
                                -alpha,
                                info,
                                true,
                            );
                        }
                    }
                }
            }

            legal_moves_count += 1;

            self.unmake_move();

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
            (&moves_list.moves[(if best_move_idx >= 0 { best_move_idx } else { 0 }) as usize])
                .into(),
            best_score,
            depth,
            node_type,
            best_draw || info.terimination_status == TeriminationStatus::Terminated,
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

        if self.position.has_insufficient_material() {
            return 0;
        }

        let stand_pat = self.position.score();

        if ply == max_ply {
            return stand_pat;
        }

        // stand-pat here
        if stand_pat >= beta {
            return stand_pat;
        }

        if stand_pat > alpha {
            alpha = stand_pat
        }

        let mut moves_list = self.generator.generate_captures(&self.position);
        moves_list.score_captures(&self.position, &self.generator);

        for i in 0..moves_list.len() {
            moves_list.sort_move(i);
            let move_item = &moves_list.moves[i];

            // likely a bad move
            if move_item.score < 0 {
                break; // all others after it will be bad too
            };

            // illegal move
            if !self.make_move(move_item.clone()) {
                continue;
            }

            let score = -self.quiescence(ply + 1, max_ply, -beta, -alpha, info);

            // The position is not yet quiet. Go one ply deeper.
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
