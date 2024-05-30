use crate::state::player::Player;
use crate::time::{TeriminationStatus, TimeControl};
use crate::moves::data::MoveItem;

use crate::search::killer::{init_killer_moves, KillerMoves};

pub struct SearchInfo {
    pub total_nodes: u64,
    pub iteration_nodes: u64,
    pub seldepth: u8,
    pub iteration_qnodes: u64,
    pub timer: TimeControl,
    pub terimination_status: TeriminationStatus,
    pub check_termination_node_interval: u64,
    pub killer_moves: KillerMoves,
    pub best_move: Option<MoveItem>,
}

impl SearchInfo {
    pub fn init(timer: TimeControl) -> SearchInfo {
        SearchInfo {
            killer_moves: init_killer_moves(),
            total_nodes: 0,
            iteration_nodes: 0,
            iteration_qnodes: 0,
            seldepth: 0,
            timer,
            best_move: None,
            terimination_status: TeriminationStatus::Distant,
            check_termination_node_interval: 0xFFF,
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

    #[inline(always)]
    pub fn update_termination_status(&mut self, side: Player, ply: u8, check_depth: bool) {
        self.terimination_status = self.timer.check_termination(
            side,
            ply,
            self.total_nodes,
            self.check_termination_node_interval,
            check_depth
        )
    }
}
