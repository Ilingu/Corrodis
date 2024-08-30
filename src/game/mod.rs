use std::{
    thread,
    time::{Duration, Instant},
    vec,
};

use anyhow::Result;
use graphics::GameGraphics;
use nanorand::{Rng, WyRand};
use termion::{event::Key, input::TermRead};
use types::{Orientation, Tetrominoe, TetrominoeType};

use crate::utils::SGR;

mod graphics;
mod types;

const FPS: usize = 1;
const BACKGROUD_COLOR: SGR = SGR::BlackBG;

pub struct GameManager {
    graphics: GameGraphics,
    cells: Vec<Vec<SGR>>,

    tetrominoe: Tetrominoe,
    orientation: Orientation,
    next_tetrominoes: [TetrominoeType; 3],

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
            cells,
            orientation: Orientation::N,
            tetrominoe: Tetrominoe::new(&graphics.inner_box_size, graphics.scale),
            next_tetrominoes: [0, 0, 0].map(|_| TetrominoeType::random()),
            graphics,
            pause: false,
        }
    }

    pub fn pick_next_tetrominoe(&mut self) {
        self.tetrominoe = {
            let mut nt = Tetrominoe::new(&self.graphics.inner_box_size, self.graphics.scale);
            nt.ttype = self.next_tetrominoes[0];
            nt
        };
        self.orientation = Orientation::N;

        self.next_tetrominoes[0] = self.next_tetrominoes[1];
        self.next_tetrominoes[1] = self.next_tetrominoes[2];

        let mut rng = WyRand::new();
        self.next_tetrominoes[1] = rng.generate_range(0_u8..=6).into();
    }

    pub fn start(&mut self) -> Result<()> {
        // init terminal screen to clean everything to start drawing
        self.graphics.hide_cursor()?;
        self.graphics.clear()?;
        self.graphics.clear_history()?;
        self.graphics.move_cursor(1, 1)?;

        self.draw_tetris_box();
        self.graphics.apply()?;

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
                    Key::Char('w') => self.orientation = self.orientation.next_cw(),
                    Key::Char('e') => self.orientation = self.orientation.next_ccw(),
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
                    }
                    _ => {}
                }
            }

            if self.pause {
                continue;
            }

            self.compute_next_frame();
            self.render()?;
            self.graphics.apply()?;

            // TODO: add optinal text to screen (score, time, title) --> this should be after self.render()

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

    pub fn compute_next_frame(&mut self) {
        self.clear_tetrominoe();
        self.pick_next_tetrominoe();
        self.draw_tetrominoe(self.tetrominoe.color);
    }
}
