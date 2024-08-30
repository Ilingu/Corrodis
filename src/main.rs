use std::io::IsTerminal;

use game::GameManager;
use utils::SGR;

mod game;
mod utils;

fn main() {
    if !std::io::stdout().is_terminal() {
        cprintln!("Please run this inside a modern terminal", SGR::RedFG);
        std::process::exit(1)
    }

    let gm = GameManager::init();
}
