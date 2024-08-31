use std::time::{Duration, Instant};

use nanorand::{Rng, WyRand};
use termsize::Size;

use crate::utils::{Uvec2, SGR};

pub struct Tetrominoe {
    pub ttype: TetrominoeType,
    pub vertices_pos: Vec<Uvec2>,
    pub still_time: Duration,
    pub now: Instant,
    pub color: SGR,
    pub scale: usize,
    inner_box_size: Size,
}

impl Tetrominoe {
    pub fn new(
        inner_box_size: &Size,
        scale: usize,
        ttype: Option<TetrominoeType>,
        pos: Option<Uvec2>,
    ) -> Self {
        let mut rng = WyRand::new();

        let ttype = ttype.unwrap_or(TetrominoeType::random());
        let (x, y) = match pos {
            Some(vec) => (vec.x, vec.y),
            None => {
                let (mut x, y) = (
                    rng.generate_range(
                        2_usize * scale..=(inner_box_size.cols as usize - 2 * scale),
                    ),
                    1,
                );
                x -= x % scale;

                (x, y)
            }
        };

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
            still_time: Duration::from_millis(50), // should vary according to score
            color: [
                SGR::BlueBG,
                SGR::CyanBG,
                SGR::GreenBG,
                SGR::MagentaBG,
                SGR::RedBG,
            ][rng.generate_range(0_usize..5)],
            now: Instant::now(),
            scale,
            inner_box_size: Size {
                rows: inner_box_size.rows,
                cols: inner_box_size.cols,
            },
        }
    }

    pub fn from_self(rhs: &Self, scale: Option<usize>, pos: Option<Uvec2>) -> Self {
        let mut s = Self::new(
            &rhs.inner_box_size,
            scale.unwrap_or(rhs.scale),
            Some(rhs.ttype),
            pos,
        );
        s.still_time = rhs.still_time;
        s.color = rhs.color;
        s
    }

    pub fn rotate(&mut self, ccw: bool) {
        let cp = self.vertices_pos[0];

        let mut rvp = Vec::with_capacity(4);
        rvp.push(cp);

        for vp in self.vertices_pos.iter().skip(1) {
            let (ox, oy) = (cp.x as isize - vp.x as isize, cp.y as isize - vp.y as isize);
            let (rox, roy) = match ccw {
                true => (oy, -ox),
                false => (-oy, ox),
            };

            let (rvpx, rvpy) = (cp.x as isize + rox, cp.y as isize + roy);
            if rvpx >= 2 * self.scale as isize
                && rvpx <= self.inner_box_size.cols as isize - 2 * self.scale as isize
                && rvpy >= 1
                && rvpy <= self.inner_box_size.rows as isize - 2 * self.scale as isize
            {
                rvp.push(Uvec2::new(rvpx as usize, rvpy as usize));
            } else {
                break;
            }
        }
        if rvp.len() == 4 {
            self.vertices_pos = rvp; // update only if all vertices can rotate
        }
    }

    pub fn fall(&mut self) {
        for vp in self.vertices_pos.iter_mut() {
            vp.y += 1
        }
    }

    pub fn translate_right(&mut self) {
        for vp in &self.vertices_pos {
            if vp.x + self.scale > self.inner_box_size.cols as usize - self.scale {
                return;
            }
        }
        for vp in self.vertices_pos.iter_mut() {
            vp.x += self.scale
        }
    }
    pub fn translate_left(&mut self) {
        for vp in &self.vertices_pos {
            if vp.x + self.scale < 2_usize * self.scale {
                return;
            }
        }
        for vp in self.vertices_pos.iter_mut() {
            vp.x -= self.scale
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
