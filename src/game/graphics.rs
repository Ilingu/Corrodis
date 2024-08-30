use std::io::{self, Write};

use termion::raw::{IntoRawMode, RawTerminal};
use termsize::Size;

use crate::{
    cprintln,
    utils::{UVec2, SGR},
};

use std::io::{BufWriter, StdoutLock};

pub struct GameGraphics {
    screen: BufWriter<RawTerminal<StdoutLock<'static>>>,
    pub size: Size,
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

        Self { screen, size }
    }

    // basic shapes - abstraction of cursor/colors calls

    // debug
    pub fn debug(&mut self, msg: &str) -> io::Result<()> {
        write!(self.screen, "{}", msg)
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
    pub fn move_cursor(&mut self, pos: UVec2) -> io::Result<()> {
        write!(self.screen, "\x1b[{};{}H", pos.x, pos.y)
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
