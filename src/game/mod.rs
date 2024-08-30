use std::{thread, time::Duration};

use graphics::GameGraphics;

mod graphics;
mod input;
mod logic;

pub struct GameManager {
    pub graphics: GameGraphics,
}

impl GameManager {
    pub fn init() -> Self {
        let graphics = GameGraphics::init();
        Self { graphics }
    }

    pub fn start(&mut self) {
        loop {
            // get input
            // compute frame
            // render frame
            thread::sleep(Duration::from_millis(50))
        }
    }
}
