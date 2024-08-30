use std::{
    thread,
    time::{Duration, Instant},
    vec,
};

use anyhow::Result;
use graphics::GameGraphics;
use termion::{event::Key, input::TermRead};

use crate::utils::SGR;

mod graphics;

const FPS: usize = 30;
const BACKGROUD_COLOR: SGR = SGR::BlackBG;

pub struct GameManager {
    graphics: GameGraphics,
    cells: Vec<Vec<SGR>>,

    pause: bool,
}

impl GameManager {
    pub fn init() -> Self {
        let graphics = GameGraphics::init();
        let cells = vec![
            vec![BACKGROUD_COLOR; graphics.term_size.cols as usize];
            graphics.term_size.rows as usize
        ];
        Self {
            graphics,
            cells,
            pause: false,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        // init terminal screen to clean everything to start drawing
        self.graphics.hide_cursor()?;
        self.graphics.clear()?;
        self.graphics.clear_history()?;
        self.graphics.move_cursor(1, 1)?;
        self.graphics.apply()?;

        self.draw_tetris_box();

        // Input event listener init
        let mut key_listener = termion::async_stdin().keys();

        // game loop
        loop {
            let now = Instant::now();

            // Read input (if any)
            let input = key_listener.next();
            if let Some(Ok(key)) = input {
                match key {
                    Key::Esc | Key::Char('q') => break,
                    Key::Char('w') => {
                        // rotateCW
                    }
                    Key::Char('e') => {
                        // rotateCCW
                    }
                    Key::AltDown | Key::Char('s') => {
                        // speed up block
                    }
                    Key::Char(' ') => {
                        // drop block
                    }
                    Key::AltLeft | Key::Char('a') => {
                        // move block left
                    }
                    Key::AltRight | Key::Char('d') => {
                        // move block right
                    }
                    Key::Char('p') => {
                        // pause/resume game
                        self.pause = !self.pause;
                    }
                    Key::Char('n') => {
                        // new game
                        *self = Self::init();
                    }
                    _ => {}
                }
            }

            if self.pause {
                continue;
            }

            // render frame
            self.render()?;
            self.graphics.apply()?;

            let elapsed = now.elapsed();
            if elapsed < Duration::from_millis(1000 / FPS as u64) {
                thread::sleep(Duration::from_millis(1000 / FPS as u64) - elapsed)
            }
        }

        self.graphics.clear()?;
        self.graphics.clear_history()?;
        self.graphics.show_cursor()?;
        self.graphics.apply()?;
        Ok(())
    }
}
