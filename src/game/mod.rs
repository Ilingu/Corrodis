use std::{thread, time::Duration};

use anyhow::Result;
use graphics::GameGraphics;
use termion::{event::Key, input::TermRead};

use crate::utils::UVec2;

mod graphics;
mod logic;

pub struct GameManager {
    pub graphics: GameGraphics,
}

impl GameManager {
    pub fn init() -> Self {
        let graphics = GameGraphics::init();
        Self { graphics }
    }

    pub fn start(&mut self) -> Result<()> {
        // init terminal screen to clean everything to start drawing
        self.graphics.hide_cursor()?;
        self.graphics.clear()?;
        self.graphics.move_cursor(UVec2::new(1, 1))?;
        self.graphics.apply()?;

        // Input event listener init
        let mut key_listener = termion::async_stdin().keys();

        loop {
            // Read input (if any)
            let input = key_listener.next();
            if let Some(Ok(key)) = input {
                match key {
                    Key::Esc | Key::Char('q') => break,
                    Key::Char('r') => {
                        // rotate piece
                        todo!()
                    }
                    // to determine...
                    _ => {}
                }
            }

            // compute frame
            // render frame
            self.graphics.debug("loop")?;

            self.graphics.apply()?;
            thread::sleep(Duration::from_millis(100))
        }

        self.graphics.clear()?;
        self.graphics.clear_history()?;
        self.graphics.show_cursor()?;
        self.graphics.apply()?;
        Ok(())
    }
}
