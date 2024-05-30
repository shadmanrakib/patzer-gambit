use std::{sync::Arc, time::Instant};

use crate::{constants::search::MAX_PLY, controller::Controller, state::player::Player};

pub struct TimeControl {
    pub times: [u128; 2],
    pub increments: [u128; 2],
    pub moves_to_go: usize, // moves left in current time control
    pub ponder: bool,
    pub infinite: bool,
    pub start_time: Instant,
    pub depth: Option<u8>,
    pub move_time: Option<u128>,
    pub nodes: Option<u64>,
    pub search_mate_in: Option<u8>,
    pub stop_controller: Arc<dyn Controller>,
}

impl Controller for TimeControl {
    fn should_stop(&self, in_search: bool, side: Player, nodes: u64, ply_or_depth: u8) -> bool {
        if self
            .stop_controller
            .should_stop(in_search, side, nodes, ply_or_depth)
        {
            return true;
        }

        !self.should_search(side, ply_or_depth, nodes)
    }
}

impl TimeControl {
    pub fn new(stop_controller: Arc<dyn Controller>) -> TimeControl {
        TimeControl {
            times: [0, 0],
            increments: [0, 0],
            moves_to_go: 40,
            ponder: false,
            infinite: false,
            start_time: Instant::now(),
            depth: None,
            move_time: None,
            nodes: None,
            search_mate_in: None,
            stop_controller,
        }
    }

    pub fn elapsed(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }

    pub fn should_search(&self, side: Player, ply: u8, nodes: u64) -> bool {
        if self.infinite {
            return true;
        }

        let elapsed = self.elapsed();

        if let Some(mt) = self.move_time {
            return elapsed < mt;
        }

        if let Some(moves) = self.search_mate_in {
            return ply <= (moves * 2);
        }

        if let Some(depth) = self.depth {
            return ply <= depth;
        }

        if let Some(n) = self.nodes {
            return nodes <= n;
        }

        // we implement Cray Blitz's time control
        let comms_overhead = 100;
        let time_left: u128 = self.times[side as usize];
        let num_book_moves = 0;
        let n_moves = std::cmp::min(num_book_moves, 10);
        let factor = 2 - n_moves / 10;
        let target = time_left / <usize as TryInto<u128>>::try_into(self.moves_to_go).unwrap();
        let allocated_time = factor * target - comms_overhead;

        return elapsed < allocated_time;
    }

    pub fn set_wtime(&mut self, time: u128) {
        self.times[Player::White as usize] = time;
    }
    pub fn set_btime(&mut self, time: u128) {
        self.times[Player::Black as usize] = time;
    }
    pub fn set_winc(&mut self, inc: u128) {
        self.increments[Player::White as usize] = inc;
    }
    pub fn set_binc(&mut self, inc: u128) {
        self.increments[Player::Black as usize] = inc;
    }
    pub fn set_moves_to_go(&mut self, moves: usize) {
        self.moves_to_go = moves;
    }
    pub fn set_ponder(&mut self) {
        self.ponder = true;
    }
    pub fn set_infinite(&mut self) {
        self.infinite = true;
    }
    pub fn set_depth(&mut self, depth: u8) {
        self.depth = Some(depth);
    }
    pub fn set_nodes(&mut self, nodes: u64) {
        self.nodes = Some(nodes);
    }
    pub fn set_move_time(&mut self, time: u128) {
        self.move_time = Some(time);
    }
    pub fn set_mate_in_x(&mut self, x: u8) {
        self.search_mate_in = Some(x);
    }
}
