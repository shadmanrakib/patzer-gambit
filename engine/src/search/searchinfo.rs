use std::sync::Arc;

use crate::{controller::Controller, moves::data::MoveItem, state::pieces::Piece};

use super::killer::{init_killer_moves, KillerMoves};

pub struct SearchInfo {
    pub total_nodes: u64,
    pub iteration_nodes: u64,
    pub seldepth: u8,
    pub iteration_qnodes: u64,
    pub controller: Arc<dyn Controller>,
    pub killer_moves: KillerMoves,
    pub best_move: Option<MoveItem>,
}

impl SearchInfo {
    pub fn init(controller: Arc<dyn Controller>) -> SearchInfo {
        SearchInfo {
            killer_moves: init_killer_moves(),
            total_nodes: 0,
            iteration_nodes: 0,
            iteration_qnodes: 0,
            seldepth: 0,
            controller,
            best_move: None,
        }
    }
    pub fn new_iteration(&mut self) {
        self.iteration_nodes = 0;
        self.iteration_qnodes = 0;
    }

    #[inline(always)]
    pub fn increment_node_counts(&mut self, is_quiescence: bool) {
        self.total_nodes += 1;
        self.iteration_nodes += 1;
        if is_quiescence {
            self.iteration_qnodes += 1;
        }
    }

    #[inline(always)]
    pub fn maximize_seldepth(&mut self, local_ply: u8) {
        self.seldepth = std::cmp::max(self.seldepth, local_ply);
    }
}
