use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};

use crate::player::Player;

#[derive(PartialEq)]
pub enum TeriminationStatus {
    Terminated,
    Soon,
    Distant,
}

#[derive(Debug)]
pub struct TimeControl {
    pub times: [u128; 2],
    pub increments: [u128; 2],
    pub moves_to_go: Option<usize>, // moves left in current time control
    pub ponder: bool,
    pub infinite: bool,
    pub start_time: Instant,
    pub depth: Option<u8>,
    pub move_time: Option<u128>,
    pub nodes: Option<u64>,
    pub search_mate_in: Option<u8>,
    pub stopped: Arc<AtomicBool>,
}

impl TimeControl {
    pub fn new(stopped: Arc<AtomicBool>) -> TimeControl {
        TimeControl {
            times: [0, 0],
            increments: [0, 0],
            moves_to_go: None,
            ponder: false,
            infinite: false,
            start_time: Instant::now(),
            depth: None,
            move_time: None,
            nodes: None,
            search_mate_in: None,
            stopped,
        }
    }

    pub fn elapsed(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }

    #[inline(always)]
    pub fn check_termination(
        &self,
        side: Player,
        ply: u8,
        nodes: u64,
        check_node_interval: u64,
        check_depth: bool,
        full_moves: u32,
    ) -> TeriminationStatus {
        if self.stopped.load(Ordering::SeqCst) {
            return TeriminationStatus::Terminated;
        }

        if self.infinite {
            return TeriminationStatus::Distant;
        }

        if let Some(depth) = self.depth {
            if check_depth && ply > depth {
                return TeriminationStatus::Terminated;
            }
            return TeriminationStatus::Distant;
        }

        let elapsed = self.elapsed();
        if let Some(mt) = self.move_time {
            if elapsed >= mt {
                return TeriminationStatus::Terminated;
            }
            let stopping_overhead = 20;
            if elapsed >= mt - stopping_overhead {
                return TeriminationStatus::Soon;
            }
            return TeriminationStatus::Distant;
        }

        if let Some(moves) = self.search_mate_in {
            if ply > (moves * 2) {
                return TeriminationStatus::Terminated;
            }
            if ply > (moves * 2) - 1 {
                return TeriminationStatus::Soon;
            }
            return TeriminationStatus::Distant;
        }

        if let Some(n) = self.nodes {
            if nodes > n {
                return TeriminationStatus::Terminated;
            }
            if nodes > n - check_node_interval {
                return TeriminationStatus::Soon;
            }
            return TeriminationStatus::Distant;
        }

        let comms_overhead = 100;
        let time_left: u128 = self.times[side as usize];
        let moves_to_go = self.moves_to_go.unwrap_or_else(|| {
            40 - std::cmp::min(<u32 as TryInto<usize>>::try_into(full_moves).unwrap(), 20)
        }) + 1;
        let target = time_left / <usize as TryInto<u128>>::try_into(moves_to_go).unwrap();
        let mut allocated_time = std::cmp::min(target, 5000);
        if allocated_time > 0 {
            allocated_time -= comms_overhead;
        }

        if elapsed >= allocated_time {
            return TeriminationStatus::Terminated;
        }
        let stopping_overhead = 70;
        if elapsed >= allocated_time - stopping_overhead {
            return TeriminationStatus::Soon;
        }
        return TeriminationStatus::Distant;
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
        self.moves_to_go = Some(moves);
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
