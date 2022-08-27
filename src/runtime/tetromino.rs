use tui::style::Color;

pub struct TetrominoInfo {
    x: usize,
    y: usize,
    direction: TetrominoDirection,
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

pub enum Tetromino {
    StraightTetromino(TetrominoInfo),
    SquareTetromino(TetrominoInfo),
    TTetromino(TetrominoInfo),
    LTetromino(TetrominoInfo),
    JTetromino(TetrominoInfo),
    SkewTetromino(TetrominoInfo),
    ZTetromino(TetrominoInfo),
}