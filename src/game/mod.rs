use std::{
    collections::HashMap,
    process::Command,
    thread,
    time::{Duration, Instant},
    vec,
};

use anyhow::{anyhow, Result};
use graphics::GameGraphics;
use termion::{event::Key, input::TermRead};
use types::Tetrominoe;

use crate::utils::{Uvec2, SGR};

mod graphics;
mod types;

const FPS: usize = 30;
const BACKGROUD_COLOR: SGR = SGR::BlackBG;

pub struct GameManager {
    graphics: GameGraphics,
    cells: Vec<Vec<SGR>>,
    cols_borders: Vec<usize>,

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
            pause: false,
            cols_borders: vec![
                graphics.box_size.rows as usize - 1;
                graphics.inner_box_size.cols as usize
            ],
            graphics,
        };

        s.next_tetrominoes = (0..3)
            .map(|i| Tetrominoe::new(&s.graphics.inner_box_size, 1, None, Some(s.nt_pos(i))))
            .collect();
        s
    }

    pub fn pick_next_tetrominoe(&mut self) {
        let new_tetrominoe = self.next_tetrominoes.remove(0);
        self.tetrominoe = Tetrominoe::from_self(&new_tetrominoe, Some(self.graphics.scale), None);

        for i in 0..=1 {
            self.next_tetrominoes[i] =
                Tetrominoe::from_self(&self.next_tetrominoes[i], None, Some(self.nt_pos(i)))
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
                    Key::Char('w') if !self.pause => {
                        self.clear_tetrominoe();
                        self.tetrominoe.rotate(true)
                    }
                    Key::Char('e') if !self.pause => {
                        self.clear_tetrominoe();
                        self.tetrominoe.rotate(false)
                    }
                    Key::Left | Key::Char('a') if !self.pause => {
                        self.clear_tetrominoe();
                        self.tetrominoe.translate_left()
                    }
                    Key::Right | Key::Char('d') if !self.pause => {
                        self.clear_tetrominoe();
                        self.tetrominoe.translate_right()
                    }
                    Key::Char('p') => self.pause = !self.pause,
                    _ => {}
                }
            }

            if !self.pause {
                let is_game_over = self.compute_next_frame();
                if is_game_over {
                    break;
                }

                self.render()?;
                self.graphics.apply()?;
                // add optinal text to screen (score, time, title) --> this should be after self.render()
            }

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

    pub fn is_collision(&self) -> bool {
        for vp in &self.tetrominoe.vertices_pos {
            let gapscale = self.graphics.scale - 1;
            let ny = vp.y + gapscale;
            if ny >= self.cols_borders[vp.x] {
                return true;
            }
        }
        false
    }

    pub fn log_new_border(&mut self) -> Result<()> {
        let mut group_cols: HashMap<usize, Vec<usize>> = HashMap::new();
        for vp in &self.tetrominoe.vertices_pos {
            group_cols
                .entry(vp.x)
                .and_modify(|rows| rows.push(vp.y))
                .or_insert(vec![vp.y]);
        }
        for (col, rows) in group_cols {
            let highest_y = *rows.iter().min().ok_or(anyhow!(""))?;

            let gapscale = self.graphics.scale - 1;
            for j in 0..gapscale {
                if self.cols_borders[col + j] > highest_y {
                    self.cols_borders[col + j] = highest_y;
                }
            }
        }

        Ok(())
    }

    pub fn check_row_clear(&mut self) {
        for row in self.cells.iter_mut() {
            if row.iter().all(|c| c != &BACKGROUD_COLOR) {
                for c in row {
                    *c = BACKGROUD_COLOR
                }
            }
        }
    }

    /// return true if game over
    pub fn compute_next_frame(&mut self) -> bool {
        if self.tetrominoe.now.elapsed() >= self.tetrominoe.still_time {
            self.clear_tetrominoe();
            self.clear_nt();
            match self.is_collision() {
                true => {
                    if self.tetrominoe.vertices_pos.iter().any(|vec| vec.y == 1) {
                        return true; // Game over
                    }

                    if self.log_new_border().is_ok() {
                        self.draw_tetrominoe(self.tetrominoe.color); // persistent image
                        self.check_row_clear();
                    }
                    self.pick_next_tetrominoe();
                }
                false => {
                    self.tetrominoe.fall();
                    self.tetrominoe.now = Instant::now();
                }
            }
            self.draw_tetrominoe(self.tetrominoe.color);
            self.draw_nt();
        }
        false
    }
}
