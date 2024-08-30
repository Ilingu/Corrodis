use std::time::Duration;

use nanorand::{Rng, WyRand};
use termsize::Size;

use crate::utils::{Uvec2, SGR};

pub struct Tetrominoe {
    pub ttype: TetrominoeType,
    pub vertices_pos: Vec<Uvec2>,
    pub still_time: Duration,
    pub color: SGR,
}

impl Tetrominoe {
    pub fn new(inner_box_size: &Size, scale: u8) -> Self {
        let mut rng = WyRand::new();
        let scale = scale as usize;

        let ttype = TetrominoeType::random();
        let (x, y) = (
            rng.generate_range(2_usize..(inner_box_size.cols as usize - 1)), // to fix with scale, causes panic
            1,
        );

        let mut vertices_pos = Vec::with_capacity(4);
        vertices_pos.push(Uvec2::new(x, y));
        match ttype {
            TetrominoeType::Bar => {
                for i in 1..4 {
                    vertices_pos.push(Uvec2::new(x, y + i * scale))
                }
            }
            TetrominoeType::Square => {
                vertices_pos.push(Uvec2::new(x + scale, y));
                vertices_pos.push(Uvec2::new(x, y + scale));
                vertices_pos.push(Uvec2::new(x + scale, y + scale));
            }
            TetrominoeType::Pyramid => {
                vertices_pos.push(Uvec2::new(x - scale, y));
                vertices_pos.push(Uvec2::new(x + scale, y));
                vertices_pos.push(Uvec2::new(x, y + scale));
            }
            TetrominoeType::LLeft => {
                for i in 1..3 {
                    vertices_pos.push(Uvec2::new(x, y + i * scale))
                }
                vertices_pos.push(Uvec2::new(x - scale, y + 2 * scale))
            }
            TetrominoeType::LRight => {
                for i in 1..3 {
                    vertices_pos.push(Uvec2::new(x, y + i * scale))
                }
                vertices_pos.push(Uvec2::new(x + scale, y + 2 * scale))
            }
            TetrominoeType::SnakeLeft => {
                vertices_pos.push(Uvec2::new(x - scale, y));
                vertices_pos.push(Uvec2::new(x, y + scale));
                vertices_pos.push(Uvec2::new(x + scale, y + scale));
            }
            TetrominoeType::SnakeRight => {
                vertices_pos.push(Uvec2::new(x + scale, y));
                vertices_pos.push(Uvec2::new(x, y + scale));
                vertices_pos.push(Uvec2::new(x - scale, y + scale));
            }
        };

        Self {
            ttype,
            vertices_pos,
            still_time: Duration::from_millis(250), // should vary according to score
            color: [
                SGR::BlueBG,
                SGR::CyanBG,
                SGR::GreenBG,
                SGR::MagentaBG,
                SGR::RedBG,
            ][rng.generate_range(0_usize..5)],
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TetrominoeType {
    Bar,
    Square,
    Pyramid,
    LLeft,
    LRight,
    SnakeLeft,
    SnakeRight,
}

impl TetrominoeType {
    pub fn random() -> Self {
        let mut rng = WyRand::new();
        rng.generate_range(0_u8..=6).into()
    }
}

impl From<u8> for TetrominoeType {
    fn from(id: u8) -> Self {
        match id {
            0 => TetrominoeType::Bar,
            1 => TetrominoeType::Square,
            2 => TetrominoeType::Pyramid,
            3 => TetrominoeType::LLeft,
            4 => TetrominoeType::LRight,
            5 => TetrominoeType::SnakeLeft,
            6 => TetrominoeType::SnakeRight,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Orientation {
    N,
    W,
    E,
    S,
}
impl Orientation {
    pub fn next_ccw(&self) -> Self {
        match self {
            Orientation::N => Orientation::W,
            Orientation::W => Orientation::S,
            Orientation::E => Orientation::N,
            Orientation::S => Orientation::E,
        }
    }
    pub fn next_cw(&self) -> Self {
        match self {
            Orientation::N => Orientation::E,
            Orientation::W => Orientation::N,
            Orientation::E => Orientation::S,
            Orientation::S => Orientation::W,
        }
    }
}
