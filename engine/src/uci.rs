use std::{
    io::{self, BufRead},
    ops::DerefMut,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

use crate::{controller::Controller, searcher::Searcher};

// Controller so we can stop search when needed
pub struct UciSearchController {
    pub(crate) terminated: Arc<AtomicBool>,
}

impl Controller for UciSearchController {
    fn has_terminated(&self) -> bool {
        self.terminated.load(Ordering::Acquire)
    }
}

impl UciSearchController {
    fn stop(&self) {
        self.terminated.store(true, Ordering::Release)
    }
    fn start(&self) {
        self.terminated.store(false, Ordering::Release)
    }
}

// Event Loop
pub fn uci_loop() {
    let stdin = io::stdin();

    let controller = Arc::new(UciSearchController {
        terminated: Arc::new(AtomicBool::new(false)),
    });

    let searcher = Arc::new(Mutex::new(Searcher::new()));

    uciok();

    for line in stdin.lock().lines() {
        let line = line.unwrap_or("".into());
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "uci" => uciok(),
            "isready" => println!("readyok"),
            "ucinewgame" => searcher.lock().unwrap().startpos(),
            "position" => {
                if parts.len() < 2 {
                    continue;
                }

                let s = &mut searcher.lock().unwrap();
                let old_pos = s.position.clone();

                let mut idx = 1;
                if parts[idx] == "startpos" {
                    s.startpos();
                    idx = 2
                } else if parts[idx] == "fen" && parts.len() >= 8 {
                    let fen = parts[2..8].join(" ");
                    if s.fen(fen).is_err() {
                        continue;
                    }
                    idx = 8
                } else {
                    continue;
                }

                if idx >= parts.len() {
                    continue;
                }

                if parts[idx] != "moves" {
                    s.position = old_pos;
                    continue;
                }

                idx += 1;
                while idx < parts.len() {
                    let notation = parts[idx].into();
                    let m = s.position.notation_to_move(notation, &s.cache);
                    if let Ok(move_item) = m {
                        s.make_move(&move_item);
                    } else {
                        s.position = old_pos;
                        break;
                    }
                    idx += 1;
                }
            }
            "go" => {
                let s = searcher.clone();
                let c: Arc<UciSearchController> = controller.clone();
                c.start();
                thread::spawn(move || {
                    s.lock().unwrap().go(100, c.clone());
                    c.stop();
                });
            }
            "stop" => {
                controller.stop();
            }
            "quit" => {
                controller.stop();
                break;
            }
            "d" => {
                searcher.lock().unwrap().position.print_state();
            }
            _ => {}
        }
    }
}

fn uciok() {
    println!("id name PatzerGambit");
    println!("id author Shadman Rakib");
    println!("uciok");
}
