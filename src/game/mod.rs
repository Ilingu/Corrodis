use std::{
    process::Command,
    thread,
    time::{Duration, Instant},
    vec,
};

use anyhow::Result;
use graphics::GameGraphics;
use termion::{event::Key, input::TermRead};
use types::Tetrominoe;

use crate::utils::{Uvec2, SGR};

mod graphics;
mod types;

const FPS: usize = 1;
const BACKGROUD_COLOR: SGR = SGR::BlackBG;

pub struct GameManager {
    graphics: GameGraphics,
    cells: Vec<Vec<SGR>>,

    tetrominoe: Tetrominoe,
    next_tetrominoes: Vec<Tetrominoe>,

    pause: bool,
}

impl GameManager {
    pub fn init() -> Self {
        let graphics = GameGraphics::init();
        let cells = vec![
            vec![BACKGROUD_COLOR; graphics.term_size.cols as usize];
            graphics.term_size.rows as usize
        ];

        let mut s = Self {
            cells,
            tetrominoe: Tetrominoe::new(&graphics.inner_box_size, graphics.scale, None, None),
            next_tetrominoes: vec![],
            graphics,
            pause: false,
        };

        s.next_tetrominoes = (0..3)
            .map(|i| Tetrominoe::new(&s.graphics.inner_box_size, 1, None, Some(s.nt_pos(i))))
            .collect();
        s
    }

    pub fn pick_next_tetrominoe(&mut self) {
        let new_tetrominoe = self.next_tetrominoes.remove(0);
        self.tetrominoe = Tetrominoe::from_self(
            &new_tetrominoe,
            &self.graphics.inner_box_size,
            Some(self.graphics.scale),
            None,
        );

        for i in 0..=1 {
            self.next_tetrominoes[i] = Tetrominoe::from_self(
                &self.next_tetrominoes[i],
                &self.graphics.inner_box_size,
                None,
                Some(self.nt_pos(i)),
            )
        }
        self.next_tetrominoes.push(Tetrominoe::new(
            &self.graphics.inner_box_size,
            1,
            None,
            Some(self.nt_pos(2)),
        ));
    }
    /// helper to compute next tetrominoes position according to its rank
    fn nt_pos(&self, rank: usize) -> Uvec2 {
        let ix = self.graphics.inner_box_size.cols as usize;
        let bx = self.graphics.inner_box_size.cols as usize;
        let x = ix + bx / 9;
        match rank {
            0 => Uvec2::new(x, 1),
            1 => Uvec2::new(x, 6),
            2 => Uvec2::new(x, 11),
            _ => unreachable!(),
        }
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
                    Key::Char('w') => {
                        self.clear_tetrominoe();
                        self.tetrominoe.rotate(true, &self.graphics.inner_box_size)
                    }
                    Key::Char('e') => {
                        self.clear_tetrominoe();
                        self.tetrominoe.rotate(false, &self.graphics.inner_box_size)
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

            // add optinal text to screen (score, time, title) --> this should be after self.render()

            let elapsed = now.elapsed();
            if elapsed < Duration::from_millis(1000 / FPS as u64) {
                thread::sleep(Duration::from_millis(1000 / FPS as u64) - elapsed)
            }
        }

        self.graphics.clear()?;
        self.graphics.clear_history()?;
        self.graphics.show_cursor()?;
        self.graphics.apply()?;
        Command::new("clear").spawn()?;
        Ok(())
    }

    pub fn compute_next_frame(&mut self) {
        self.clear_tetrominoe();
        self.clear_nt();
        if self.tetrominoe.now.elapsed() >= self.tetrominoe.still_time {
            // self.tetrominoe.fall();
            self.tetrominoe.now = Instant::now();
        }
        self.pick_next_tetrominoe();
        self.draw_tetrominoe(self.tetrominoe.color);
        self.draw_nt();
    }
}
