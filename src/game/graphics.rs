use std::io::{self, Write};

use termion::raw::{IntoRawMode, RawTerminal};
use termsize::Size;

use crate::{cprintln, utils::SGR};

use std::io::{BufWriter, StdoutLock};

use super::GameManager;

pub struct GameGraphics {
    screen: BufWriter<RawTerminal<StdoutLock<'static>>>,
    pub term_size: Size,
    pub tetris_size: Size,
    pub offset: Size,
}

impl GameGraphics {
    pub fn init() -> Self {
        let stdout = match io::stdout().lock().into_raw_mode() {
            Ok(stdout) => stdout,
            Err(_) => {
                cprintln!(
                    "Couldn't get terminal into raw mode. Please use a modern terminal",
                    SGR::RedFG
                );
                std::process::exit(1);
            }
        };
        let screen = io::BufWriter::new(stdout);

        let size = match termsize::get() {
            Some(s) => s,
            None => {
                cprintln!(
                    "Couldn't get terminal dimension. Please use a modern terminal",
                    SGR::RedFG
                );
                std::process::exit(1);
            }
        }; // Size { rows: 45, cols: 190 }

        if size.cols / 2 < 50 || size.rows - 2 < 30 {
            cprintln!("Terminal window too small to render コロディス", SGR::RedFG);
            std::process::exit(1);
        }

        let tetris_size = Size {
            rows: size.rows - 2,
            cols: size.cols / 2,
        };
        let offset = Size {
            rows: 1,
            cols: size.cols / 4,
        };

        Self {
            screen,
            tetris_size,
            offset,
            term_size: size,
        }
    }

    // writing tool
    pub fn debug(&mut self, msg: &str) -> io::Result<()> {
        write!(self.screen, "{}", msg)
    }
    pub fn blank(&mut self) -> io::Result<()> {
        write!(self.screen, " ")
    }

    // cursor
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.screen, "\x1b[?25l")
    }
    pub fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.screen, "\x1b[?25h")
    }
    pub fn clear(&mut self) -> io::Result<()> {
        write!(self.screen, "\x1b[2J")
    }
    pub fn clear_history(&mut self) -> io::Result<()> {
        write!(self.screen, "\x1b[3J")
    }
    pub fn move_cursor(&mut self, x: usize, y: usize) -> io::Result<()> {
        write!(self.screen, "\x1b[{};{}H", y, x)
    }

    // colors
    pub fn reset_colors(&mut self) -> io::Result<()> {
        write!(self.screen, "\x1b[{}m", SGR::Reset)
    }
    pub fn set_colors(&mut self, sgr: &[SGR]) -> io::Result<()> {
        write!(
            self.screen,
            "\x1b[{}m",
            sgr.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(";")
        )
    }

    /// apply update
    pub fn apply(&mut self) -> io::Result<()> {
        self.screen.flush()
    }
}

impl GameManager {
    pub fn draw_tetris_box(&mut self) {
        const BOX_COLOR: SGR = SGR::YellowBG;

        /* Box Drawing */
        let (h, w) = (
            self.graphics.tetris_size.rows as usize,
            self.graphics.tetris_size.cols as usize,
        );
        let (oy, ox) = (
            self.graphics.offset.rows as usize,
            self.graphics.offset.cols as usize,
        );

        for y in (oy)..(h + oy) {
            if y == oy || y == h + oy - 1 {
                for x in ox..(w + ox) {
                    self.cells[y][x] = BOX_COLOR;
                }
            } else {
                self.cells[y][ox] = BOX_COLOR;
                self.cells[y][w + ox - 1] = BOX_COLOR;
            }
        }

        // border delimiter for "next" tedrinos
        let border_x = ox + w - (w / 5);
        for y in (oy)..(h + oy) {
            self.cells[y][border_x] = BOX_COLOR;
        }

        /* Title Drawing */
        let (rt_ox, rt_oy) = ((self.graphics.term_size.cols / 15) as usize, 1_usize);
        let (rt_w, rt_h) = (
            (self.graphics.term_size.cols / 12) as usize,
            ((self.graphics.term_size.rows - 2) / 6) as usize,
        );

        // first letter - コ
        const FL_COLOR: SGR = SGR::BlueBG;
        for x in rt_ox..(rt_w + rt_ox) {
            self.cells[rt_oy][x] = FL_COLOR;
            self.cells[rt_oy + rt_h - 1][x] = FL_COLOR;
        }
        for y in rt_oy..(rt_h + rt_oy) {
            self.cells[y][rt_ox + rt_w] = FL_COLOR;
        }

        // second letter - ロ
        const SL_COLOR: SGR = SGR::CyanBG;
        let base_y = rt_oy + rt_h + 1;
        for x in rt_ox..(rt_w + rt_ox) {
            self.cells[base_y][x] = SL_COLOR;
            self.cells[base_y + rt_h - 2][x] = SL_COLOR;
        }
        for y in (rt_oy + rt_h + 1)..(base_y + rt_h) {
            self.cells[y][rt_ox] = SL_COLOR;
            self.cells[y][rt_ox + rt_w] = SL_COLOR;
        }

        // third letter - デ
        const TL_COLOR: SGR = SGR::GreenBG;
        let base_y = rt_oy + rt_h * 2 + 2;

        self.cells[base_y][rt_ox + rt_w] = TL_COLOR;
        self.cells[base_y + 1][rt_ox + rt_w + 1] = TL_COLOR;

        self.cells[base_y][rt_ox + rt_w + 2] = TL_COLOR;
        self.cells[base_y + 1][rt_ox + rt_w + 1 + 2] = TL_COLOR;

        for x in (rt_ox + 1)..(rt_w + rt_ox - 1) {
            self.cells[base_y][x] = TL_COLOR;
        }
        for x in rt_ox..(rt_w + rt_ox) {
            self.cells[base_y + 2][x] = TL_COLOR;
        }
        for i in 0..4 {
            self.cells[base_y + 3 + i][rt_ox + rt_w / 2 - i] = TL_COLOR;
            self.cells[base_y + 2 + i][rt_ox + rt_w / 2 - i] = TL_COLOR;
        }
        if rt_ox < rt_ox + rt_w / 2 - 4 {
            for x in rt_ox..=rt_ox + rt_w / 2 - 4 {
                self.cells[base_y + 3 + 3][x] = TL_COLOR;
            }
        }

        // fourth letter - ィ
        const FOL_COLOR: SGR = SGR::MagentaBG;
        let base_y = base_y + 3 + 3 + 1;
        {
            let mut i = 0;
            while i * 4 <= rt_w {
                for j in 0..4 {
                    self.cells[base_y + i][rt_ox + rt_w - i * 4 - j] = FOL_COLOR;
                }
                i += 1;
            }
        }
        for y in (base_y + 1)..(base_y + 1 + rt_h) {
            self.cells[y][rt_ox + rt_w / 2] = FOL_COLOR;
            self.cells[y][rt_ox + rt_w / 2 + 1] = FOL_COLOR;
        }

        // fifth letter - ス
        const FIL_COLOR: SGR = SGR::RedBG;
        let base_y = base_y + 1 + rt_h + 1;
        for x in rt_ox..(rt_w + rt_ox) {
            self.cells[base_y][x] = FIL_COLOR;
        }
        {
            let mut i = 0;
            'outer: while i * 2 <= rt_w {
                for j in 0..2 {
                    if rt_w - i * 2 - j - 1 == 0 {
                        break 'outer;
                    }
                    self.cells[base_y + 1 + i][rt_ox + rt_w - i * 2 - j - 1] = FIL_COLOR;
                }
                i += 1;
            }
        }
        {
            let mut i = 0;
            'outer: while i * 2 <= rt_w {
                for j in 0..2 {
                    if rt_w + i * 2 + j + 1 == rt_w + rt_ox - 3 {
                        break 'outer;
                    }
                    self.cells[base_y + 1 + i + rt_h / 2][rt_ox + rt_w / 2 + i * 2 + j + 1] =
                        FIL_COLOR;
                }
                i += 1;
            }
        }
    }

    pub fn render(&mut self) -> io::Result<()> {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                self.graphics.move_cursor(x + 1, y + 1)?;
                self.graphics.set_colors(&[*c])?;
                self.graphics.blank()?;
            }
        }
        Ok(())
    }
}
