use rand::{Rng, thread_rng};
use tui::style::Color;
use tui::style::Color::{Blue, Green, LightBlue, Magenta, Yellow};
use crate::runtime::tetromino::Tetromino::{JTetromino, LTetromino, SkewTetromino, SquareTetromino, StraightTetromino, TTetromino, ZTetromino};
use crate::runtime::tetromino::TetrominoDirection::{BOTTOM, LEFT, RIGHT, TOP};


pub struct TetrominoInfo {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) direction: TetrominoDirection,
    pub(crate) color: Color,
}

pub struct Point {
    x: usize,
    y: usize,
    color: Color,
}

impl Default for TetrominoInfo {
    fn default() -> Self {
        TetrominoInfo {
            x: 0,
            y: 0,
            direction: TetrominoDirection::TOP,
            color: Color::White,
        }
    }
}

impl TetrominoInfo {
    pub fn new(x: usize, y: usize, direction: TetrominoDirection, color: Color) -> TetrominoInfo {
        TetrominoInfo {
            x,
            y,
            direction,
            color,
        }
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn direction(&self) -> &TetrominoDirection {
        &self.direction
    }
    pub fn color(&self) -> Color {
        self.color
    }
}

pub enum TetrominoDirection {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

impl From<u8> for TetrominoDirection {
    fn from(i: u8) -> Self {
        match i {
            0 => TOP,
            1 => BOTTOM,
            2 => RIGHT,
            3 => LEFT,
            _ => panic!("Unreachable!")
        }
    }
}

pub enum Tetromino {
    StraightTetromino(TetrominoInfo),
    SquareTetromino(TetrominoInfo),
    TTetromino(TetrominoInfo),
    LTetromino(TetrominoInfo),
    JTetromino(TetrominoInfo),
    SkewTetromino(TetrominoInfo),
    ZTetromino(TetrominoInfo),
}

impl From<(u8, TetrominoInfo)> for Tetromino {
    fn from((i, info): (u8, TetrominoInfo)) -> Self {
        match i {
            0 => StraightTetromino(info),
            1 => SquareTetromino(info),
            2 => TTetromino(info),
            3 => LTetromino(info),
            4 => JTetromino(info),
            5 => SkewTetromino(info),
            6 => ZTetromino(info),
            _ => panic!("Unreachable!")
        }
    }
}

const COLOR_MODEL: [Color; 7] = [LightBlue, Yellow, Magenta, Blue, Blue, Green, Green];

impl Tetromino {
    pub fn next_rng() -> Tetromino {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..=20);
        let i = rng.gen_range(0..=6);
        let direction = rng.gen_range(0..=3);
        let tetromino_info = TetrominoInfo::new(
            x,
            0,
            TetrominoDirection::from(direction),
            COLOR_MODEL[i as usize],
        );
        Tetromino::from((i, tetromino_info))
    }

    fn info(&mut self) -> &mut TetrominoInfo {
        match self {
            StraightTetromino(info) => info,
            SquareTetromino(info) => info,
            TTetromino(info) => info,
            LTetromino(info) => info,
            JTetromino(info) => info,
            SkewTetromino(info) => info,
            ZTetromino(info) => info
        }
    }

    pub fn step_x_left(&mut self) {
        let info = self.info();
        info.x += 1;
    }
    pub fn step_x_right(&mut self) {
        let mut info = self.info();
        info.x += 1;
    }
    pub fn step_y_down(&mut self) {
        let mut info = self.info();
        info.y += 1;
    }
    pub fn step_next_direction(&mut self) {
        let mut info = self.info();
        let direction: u8 = match info.direction {
            TOP => 0,
            RIGHT => 1,
            BOTTOM => 2,
            LEFT => 3
        };
        if direction == 3 {
            info.direction = TetrominoDirection::from(0);
        } else {
            info.direction = TetrominoDirection::from(direction + 1);
        }
    }
}