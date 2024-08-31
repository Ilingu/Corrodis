use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use termsize::Size;

#[macro_export]
macro_rules! cprintln {
    ($msg:expr, $c:expr) => {
        println!("\x1b[{}m{}\x1b[0m", $c, $msg)
    };
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum SGR {
    Reset = 0,
    Bold,
    Light,
    Italic,
    Underline,
    Strike = 9,

    BlackFG = 30,
    RedFG,
    GreenFG,
    YellowFG,
    BlueFG,
    MagentaFG,
    CyanFG,
    WhiteFG,

    BlackBG = 40,
    RedBG,
    GreenBG,
    YellowBG,
    BlueBG,
    MagentaBG,
    CyanBG,
    WhiteBG,

    BrightBlackFG = 90,
    BrightRedFG,
    BrightGreenFG,
    BrightYellowFG,
    BrightBlueFG,
    BrightMagentaFG,
    BrightCyanFG,
    BrightWhiteFG,

    BrightBlackBG = 100,
    BrightRedBG,
    BrightGreenBG,
    BrightYellowBG,
    BrightBlueBG,
    BrightMagentaBG,
    BrightCyanBG,
    BrightWhiteBG,
}

impl From<SGR> for u8 {
    fn from(c: SGR) -> Self {
        c as u8
    }
}

impl Display for SGR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Uvec2 {
    pub x: usize,
    pub y: usize,
}

impl Uvec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add for Uvec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
