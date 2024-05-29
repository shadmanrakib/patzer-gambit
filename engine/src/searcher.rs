use std::{
    sync::Arc,
    time::Instant,
};

use crate::{
    constants::search::{
        FULL_DEPTH_MOVES, MAX_PLY, REDUCTION_LIMIT, TRANSITION_TABLE_ADDRESSING_BITS,
    },
    controller::Controller,
    evaluation::psqt_tapered,
    moves::{
        data::MoveItem,
        generator::{
            movegen::generate_pseudolegal_moves, precalculated_lookups::cache::PrecalculatedCache,
        },
        scoring::score_moves,
    },
    search::{
        cache::SearchCache,
        killer::{store_killer_move, SimpleMove},
        quiescence::quiescence,
        transposition::{NodeType, TTable},
        zobrist::ZobristHasher,
    },
    state::{game::GameState, moves::MoveList, pieces::Piece, player::Player},
    utils::in_check::is_in_check,
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
    pub fn make_move(&mut self, move_item: &MoveItem) {
        self.position.make_move(move_item, &self.zobrist);
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
                        if let Ok(move_item) =  position.notation_to_move(notation.clone(), &self.cache) {
                            position.make_move(&move_item, &self.zobrist);
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

    pub fn go(
        &mut self,
        main_search_depth: u8,
        controller: Arc<dyn Controller>,
    ) -> Option<SimpleMove> {
        let mut best_move: Option<SimpleMove> = None;
        let mut bm: Option<MoveItem> = None;

        // let mut file: std::fs::File = OpenOptions::new()
        //     .append(true)
        //     .create(true)
        //     .open("/Users/shadmanrakib/Desktop/patzer-gambit/engine/output2.txt")
        //     .unwrap();

        let mut depth = 1;
        let mut search_cache = SearchCache::init();

        let mut alpha = NEG_INF;
        let mut beta = INF;

        // let fen = self.position.to_fen();
        // let f: String = format!("position fen {fen}");
        // writeln!(file, "{}", f).unwrap();

        while depth <= main_search_depth {
            let iter_start = Instant::now();

            let mut nodes = 0;
            let mut q_nodes = 0;
            let mut seldepth = 0;

            let score = self.negamax(
                depth,
                0,
                MAX_PLY,
                alpha,
                beta,
                &mut search_cache,
                &mut nodes,
                &mut q_nodes,
                &mut seldepth,
                controller.clone(),
                &mut bm,
            );

            let iter_elapsed = iter_start.elapsed();
            let ms = iter_elapsed.as_millis();
            let ns = iter_elapsed.as_nanos();

            let nps = (nodes as u128) * 10_u128.pow(9) / (ns + 1);

            let total_nps = (nodes + q_nodes) * 10_u128.pow(9) / (ns + 1);

            let pv = self.get_pv(depth);
            let pv_str_vec: Vec<String> = pv.iter().map(|x| x.to_string()).collect();
            let pv_str = pv_str_vec.join(" ");
            if pv.len() > 0 {
                best_move = Some(SimpleMove::from(pv[0]));
            }

            // the aspiration window was a bad idea, lets retry without it
            if score <= alpha || score >= beta {
                alpha = NEG_INF;
                beta = INF;
                continue;
            }

            println!("info score cp {score} depth {depth} seldepth {seldepth} time {ms} nodes {nodes} nps {total_nps} pv {pv_str}");
            // let f: String = format!("info score cp {score} depth {depth} seldepth {seldepth} time {ms} nodes {nodes} nps {total_nps} pv {pv_str}");
            // writeln!(file, "{}", f).unwrap();

            alpha = score.saturating_sub(70);
            beta = score.saturating_add(70);

            depth += 1;

            if controller.should_stop(false, self.position.side_to_move, nodes, depth) {
                break;
            }
        }

        let mut occ: u64 = 0;
        for i in 0..self.tt.table.len() {
            let entry = &mut self.tt.table[i];
            if entry.best_move.to != entry.best_move.from {
                entry.ancient = true;
                occ += 1;
            }
        }

        if let Some(move_item) = &best_move {
            let short = move_item.to_string();
            println!("bestmove {short}");
            if let Some(mv) = &bm {
                let short2 = mv.notation();
                println!("apples bestmove correct {} {short} {short2}", short == short2);
            }
        } else {
            println!("bestmove");
        }

        println!("occ {occ} of {}", self.tt.size);

        return best_move;
    }

    fn negamax(
        &mut self,
        mut depth: u8,
        ply: u8,
        max_ply: u8,
        mut alpha: i32,
        beta: i32,
        search_cache: &mut SearchCache,
        nodes: &mut u128,
        q_nodes: &mut u128,
        seldepth: &mut u8,
        controller: Arc<dyn Controller>,
        best_move: &mut Option<MoveItem>,
    ) -> i32 {
        let player = self.position.side_to_move;

        let mut node_type = NodeType::All;

        *seldepth = std::cmp::max(*seldepth, ply);

        let mut tt_move = SimpleMove {
            from: 0,
            to: 0,
            promotion: Piece::King,
        };
        // if game.half_move_clock >= 50 || game.has_three_fold_rep() {
        //     return NegamaxResult {
        //         score: 0,
        //         // mate: 0,
        //         // mated: false,
        //     };
        // }

        if let Some((simple_move, score, d)) = self.tt.probe(self.position.hash, alpha, beta) {
            if d >= depth {
                return score;
            };

            tt_move = simple_move;
        }
        // handle base case
        if ply == max_ply || controller.should_stop(true, player, *nodes, ply) {
            *nodes += 1;
            let color = if player == Player::White { 1 } else { -1 };
            let score = color * psqt_tapered::eval(&self.position);
            return score;
        }
        if depth <= 0 {
            let score = quiescence(
                &mut self.position,
                ply,
                max_ply,
                alpha,
                beta,
                &self.cache,
                search_cache,
                nodes,
                &self.zobrist,
                seldepth,
                controller.clone(),
            );
            // let interupted = controller.should_stop(true, player, *nodes, ply);
            // tt.record(
            //     game.hash,
            //     SimpleMove {
            //         from: 0,
            //         to: 0,
            //         promotion: Piece::King,
            //     },
            //     score,
            //     depth,
            //     NodeType::Pv,
            //     interupted,
            // );
            return score;

            // return quiescence(game, ply, max_ply, alpha, beta, cache, search_cache);
        }

        *nodes += 1;

        let in_check = is_in_check(player, &self.position, &self.cache);

        // let color = if player == Player::White { 1 } else { -1 };
        // let standpat = psqt_tapered::eval(game) * color;

        // null pruning
        if !in_check && depth >= 3 && ply != 0 && self.position.phase < 180
        // && standpat >= beta
        // && game.bitboards.boards[player as usize][Piece::Pawn as usize] != 0
        // && game.bitboards.boards[player.opponent() as usize][Piece::Pawn as usize] != 0
        {
            let r = 2;
            let prev_enpassant = self.position.make_null_move(&self.zobrist);
            let null_score = -self.negamax(
                depth - 1 - r,
                ply + 1,
                max_ply,
                -beta,
                -beta + 1,
                search_cache,
                nodes,
                q_nodes,
                seldepth,
                controller.clone(),
                best_move,
            );
            self.position.unmake_null_move(prev_enpassant, &self.zobrist);

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
        score_moves(&mut moveslist, search_cache, ply as usize, player, tt_move);
        let mut legal_moves_count: u8 = 0;
        let mut moves_searched = 0;

        let mut best_draw = false;

        for index in 0..moveslist.len() {
            moveslist.sort_move(index);
            let move_item = &moveslist.moves[index];
            let unmake_metadata = self.position.make_move(&move_item, &self.zobrist);

            // illegal move
            if is_in_check(player, &self.position, &self.cache) {
                self.position.unmake_move(&move_item, unmake_metadata, &self.zobrist);
                continue;
            };

            legal_moves_count += 1;

            let mut score: i32;

            let draw = self.position.has_three_fold_rep() || self.position.half_move_clock >= 50;
            if draw {
                score = 0;
            } else {
                if moves_searched == 0 {
                    score = -self.negamax(
                        depth - 1,
                        ply + 1,
                        max_ply,
                        -beta,
                        -alpha,
                        search_cache,
                        nodes,
                        q_nodes,
                        seldepth,
                        controller.clone(),
                        best_move,
                    );
                } else {
                    if moves_searched >= FULL_DEPTH_MOVES
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
                            search_cache,
                            nodes,
                            q_nodes,
                            seldepth,
                            controller.clone(),
                            best_move,
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
                            search_cache,
                            nodes,
                            q_nodes,
                            seldepth,
                            controller.clone(),
                            best_move,
                        );
                        if score > alpha && score < beta {
                            score = -self.negamax(
                                depth - 1,
                                ply + 1,
                                max_ply,
                                -beta,
                                -alpha,
                                search_cache,
                                nodes,
                                q_nodes,
                                seldepth,
                                controller.clone(),
                                best_move,
                            );
                        }
                    }
                }
            }

            self.position.unmake_move(&move_item, unmake_metadata, &self.zobrist);

            moves_searched += 1;

            // if ply == 0 {
            //     println!("{} {}", move_item.notation(), score);
            // }

            let interupted = controller.should_stop(true, player, *nodes, ply);

            if score > best_score {
                best_score = score;
                best_move_idx = index as i32;
                if ply == 0 {
                    *best_move = Some(move_item.clone());
                }
                best_draw = draw;
            }

            if score >= beta {
                if !move_item.capturing {
                    store_killer_move(&mut search_cache.killer_moves, move_item, ply as usize);
                }

                self.tt.record(
                    self.position.hash,
                    move_item.into(),
                    score,
                    depth,
                    NodeType::Cut,
                    interupted || draw,
                );

                return score;
            }

            if score > alpha {
                alpha = score;

                node_type = NodeType::Pv;

                // if !move_item.capturing {
                //     search_cache.history_moves[player as usize][move_item.piece as usize]
                //         [move_item.to_pos as usize] = search_cache.history_moves[player as usize]
                //         [move_item.piece as usize][move_item.to_pos as usize]
                //         + (depth as i16);
                // }
            }
        }

        let interupted = controller.should_stop(true, player, *nodes, ply);

        if legal_moves_count == 0 {
            let score = if in_check {
                CHECKMATE + (ply as i32)
            } else {
                STALEMATE
            };

            // println!("checkmate! {}", score);

            // tt.record(
            //     game.hash,
            //     SimpleMove {
            //         to: 0,
            //         from: 0,
            //         promotion: Piece::King,
            //     },
            //     score,
            //     depth,
            //     NodeType::Pv,
            //     interupted,
            // );

            return score;
        }

        self.tt.record(
            self.position.hash,
            (&moveslist.moves[(if best_move_idx >= 0 { best_move_idx } else { 0 }) as usize])
                .into(),
            best_score,
            depth,
            node_type,
            best_draw,
        );

        best_score
    }
}
