use std::{
    fs::OpenOptions, io::{self, BufRead, Write}, sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    }, thread
};

use crate::{controller::Controller, searcher::Searcher, state::player::Player, time::TimeControl};

// Controller so we can stop search when needed
pub struct UciSearchController {
    pub(crate) terminated: Arc<AtomicBool>,
}

impl Controller for UciSearchController {
    fn should_stop(&self, _: bool, _: Player, _: u128, _: u8) -> bool {
        self.terminated.load(Ordering::SeqCst)
    }
}

impl UciSearchController {
    fn stop(&self) {
        self.terminated.store(true, Ordering::SeqCst)
    }
    fn start(&self) {
        self.terminated.store(false, Ordering::SeqCst)
    }
}

fn parse_position(searcher: &mut Searcher, parts: &Vec<&str>) {
    if parts.len() < 2 {
        return;
    }

    let old_pos = searcher.position.clone();

    let mut idx = 1;
    match parts[idx] {
        "startpos" => {
            searcher.startpos();
            idx = 2;
        }
        "fen" if parts.len() >= 8 => {
            let fen = parts[2..8].join(" ");
            if searcher.fen(fen).is_err() {
                return;
            }
            idx = 8;
        }
        _ => return,
    }

    if idx >= parts.len() {
        return;
    }

    if parts[idx] != "moves" {
        searcher.position = old_pos;
        return;
    }

    idx += 1; // consume "moves" string

    // play all moves specified
    while idx < parts.len() {
        match searcher
            .position
            .notation_to_move(parts[idx].into(), &searcher.cache)
        {
            Ok(move_item) => searcher.make_move(&move_item),
            Err(_) => {
                searcher.position = old_pos;
                break;
            }
        }
        idx += 1;
    }
}

fn parse_go(parts: Vec<String>, stop_controller: Arc<dyn Controller>) -> TimeControl {
    let mut time = TimeControl::new(stop_controller);
    let mut iter = parts.iter().skip(1); // Skip the "go" command
    while let Some(token) = iter.next() {
        // Match the token against known commands
        match &**token {
            "wtime" => {
                if let Some(value) = iter.next() {
                    if let Ok(time_value) = value.parse::<u128>() {
                        time.set_wtime(time_value);
                    }
                }
            }
            "btime" => {
                if let Some(value) = iter.next() {
                    if let Ok(time_value) = value.parse::<u128>() {
                        time.set_btime(time_value);
                    }
                }
            }
            "winc" => {
                if let Some(value) = iter.next() {
                    if let Ok(inc_value) = value.parse::<u128>() {
                        time.set_winc(inc_value);
                    }
                }
            }
            "binc" => {
                if let Some(value) = iter.next() {
                    if let Ok(inc_value) = value.parse::<u128>() {
                        time.set_binc(inc_value);
                    }
                }
            }
            "movestogo" => {
                if let Some(value) = iter.next() {
                    if let Ok(moves_value) = value.parse::<usize>() {
                        time.set_moves_to_go(moves_value);
                    }
                }
            }
            "ponder" => {
                time.set_ponder();
            }
            "infinite" => {
                time.set_infinite();
            }
            "depth" => {
                if let Some(value) = iter.next() {
                    if let Ok(depth_value) = value.parse::<u8>() {
                        time.set_depth(depth_value);
                    }
                }
            }
            "nodes" => {
                if let Some(value) = iter.next() {
                    if let Ok(nodes_value) = value.parse::<u128>() {
                        time.set_nodes(nodes_value);
                    }
                }
            }
            "movetime" => {
                if let Some(value) = iter.next() {
                    if let Ok(mt_value) = value.parse::<u128>() {
                        time.set_move_time(mt_value);
                    }
                }
            }
            "mate" => {
                if let Some(value) = iter.next() {
                    if let Ok(mate_value) = value.parse::<u8>() {
                        time.set_mate_in_x(mate_value);
                    }
                }
            }
            _ => {} // Ignore unknown commands
        }
    }
    time
}

// Event Loop
pub fn uci_loop() {
    let stdin = io::stdin();

    let controller = Arc::new(UciSearchController {
        terminated: Arc::new(AtomicBool::new(false)),
    });

    let searcher = Arc::new(Mutex::new(Searcher::new()));

    intro();

    // let mut file = OpenOptions::new()
    //     .append(true)
    //     .create(true)
    //     .open("/Users/shadmanrakib/Desktop/patzer-gambit/engine/output.txt").unwrap();

    //     writeln!(file, "=========").unwrap();
    //     writeln!(file, "=========").unwrap();

    for line in stdin.lock().lines() {
        let line = line.unwrap_or("".into());
        // let l_clone = line.clone();
        let parts: Vec<&str> = line.split_whitespace().collect();

        // writeln!(file, "{}", l_clone).unwrap();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "uci" => uciok(),
            "isready" => println!("readyok"),
            "ucinewgame" => searcher.lock().unwrap().startpos(),
            "position" => parse_position(&mut searcher.lock().unwrap(), &parts),
            "go" => {
                let s = searcher.clone();
                let c: Arc<UciSearchController> = controller.clone();
                let p: Vec<String> = parts.iter().map(|item| item.to_string()).collect();
                c.start();
                thread::spawn(move || {
                    let time: Arc<TimeControl> = Arc::new(parse_go(p, c.clone()));
                    s.lock().unwrap().go(100, time);
                });
            }
            "stop" => controller.stop(),
            "quit" => {
                controller.stop();
                break;
            }
            "d" => searcher.lock().unwrap().position.print_state(),
            "play" => {
                let mut s = searcher.lock().unwrap();
                if parts.len() > 1 {
                    let m = s.position.notation_to_move(parts[1].into(), &s.cache);
                    if let Ok(move_item) = m {
                        s.make_move(&move_item);
                    }
                }
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

fn intro() {
    println!("-------------------------------------------------------------------------------------------------------");
    println!("");
    println!("");
    println!("██████╗░░█████╗░████████╗███████╗███████╗██████╗░   ░██████╗░░█████╗░███╗░░░███╗██████╗░██╗████████╗");
    println!("██╔══██╗██╔══██╗╚══██╔══╝╚════██║██╔════╝██╔══██╗   ██╔════╝░██╔══██╗████╗░████║██╔══██╗██║╚══██╔══╝");
    println!("██████╔╝███████║░░░██║░░░░░███╔═╝█████╗░░██████╔╝   ██║░░██╗░███████║██╔████╔██║██████╦╝██║░░░██║░░░");
    println!("██╔═══╝░██╔══██║░░░██║░░░██╔══╝░░██╔══╝░░██╔══██╗   ██║░░╚██╗██╔══██║██║╚██╔╝██║██╔══██╗██║░░░██║░░░");
    println!("██║░░░░░██║░░██║░░░██║░░░███████╗███████╗██║░░██║   ╚██████╔╝██║░░██║██║░╚═╝░██║██████╦╝██║░░░██║░░░");
    println!("╚═╝░░░░░╚═╝░░╚═╝░░░╚═╝░░░╚══════╝╚══════╝╚═╝░░╚═╝   ░╚═════╝░╚═╝░░╚═╝╚═╝░░░░░╚═╝╚═════╝░╚═╝░░░╚═╝░░░");
    println!("");
    println!("");
    println!("A (partial) UCI chess engine that's a whole lot worser than drunk magnus but magnitudes better than me.");
    println!("");
    println!("");
    println!("-------------------------------------------------------------------------------------------------------");
    println!("");
}
