use std::{io::IsTerminal, panic, thread, time::Duration};

use game::GameManager;
use utils::SGR;

mod game;
mod utils;

fn main() {
    if !std::io::stdout().is_terminal() {
        cprintln!("Please run this inside a modern terminal", SGR::RedFG);
        std::process::exit(1)
    }

    let is_safe_mode = std::env::args().skip(1).any(|a| a == "-s" || a == "--safe");
    if is_safe_mode {
        // to detect if a panic occured, if yes recover from it since we must give the control of the terminal back to the user
        println!("Game launching in safe mode...");
        // thread::sleep(Duration::from_secs(1));
        let game_exiting_result = panic::catch_unwind(|| {
            let mut gm = GameManager::init();
            if let Err(why) = gm.start() {
                cprintln!(format!("Game crashed: {why:?}"), SGR::RedFG)
            }
        });
        if game_exiting_result.is_err() {
            print!("\x1b[2J\x1b[3J\x1b[?25h"); // clear screen and show cursor
            cprintln!(
                format!("Fatal game crash: recovered from main. Clearing screen"),
                SGR::RedFG
            )
        }
    } else {
        let mut gm = GameManager::init();
        if let Err(why) = gm.start() {
            cprintln!(format!("Game crashed: {why:?}"), SGR::RedFG)
        }
    }
}
