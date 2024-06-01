use crate::{
    moves::{SimpleMove, Move},
    player::Player,
    settings::{MAX_KILLER_MOVES, MAX_PLY},
    time::{TeriminationStatus, TimeControl},
};

pub type KillerMoves = [[SimpleMove; MAX_KILLER_MOVES]; MAX_PLY as usize];

pub struct SearchInfo {
    pub total_nodes: u64,
    pub iteration_nodes: u64,
    pub seldepth: u8,
    pub iteration_qnodes: u64,
    pub timer: TimeControl,
    pub terimination_status: TeriminationStatus,
    pub check_termination_node_interval: u64,
    pub killer_moves: KillerMoves,
    pub best_move: Option<Move>,
}

impl SearchInfo {
    pub fn init(timer: TimeControl) -> SearchInfo {
        SearchInfo {
            killer_moves: [[SimpleMove::NULL_MOVE; MAX_KILLER_MOVES]; MAX_PLY as usize],
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
    pub fn update_termination_status(
        &mut self,
        side: Player,
        ply: u8,
        check_depth: bool,
        full_moves: u32,
    ) {
        self.terimination_status = self.timer.check_termination(
            side,
            ply,
            self.total_nodes,
            self.check_termination_node_interval,
            check_depth,
            full_moves,
        )
    }
    #[inline(always)]
    pub fn store_killer_move(&mut self, current_move: &Move, ply: usize) {
        let first_killer = self.killer_moves[ply as usize][0];
        // only place if current move isn't killer (to prevent it being filled with one move)
        if !first_killer.is_similar(current_move) {
            // Shift all the moves one index upward...
            for i in (1..MAX_KILLER_MOVES).rev() {
                self.killer_moves[ply][i] = self.killer_moves[ply][i - 1];
            }
            // and add the new killer move in the first spot.
            self.killer_moves[ply as usize][0] = current_move.into();
        }
    }
}
