use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::Color;
use tui::symbols::Marker;
use tui::widgets::{Block, Borders};
use tui::widgets::canvas::{Canvas, Line, Painter, Rectangle, Shape};
use crate::app::App;
use crate::ui::Draw;
use crate::runtime::tetromino::{TetrominoDirection, Tetromino::*, TetrominoInfo, Tetromino};

pub(crate) struct MiddlePanel;


impl Draw for MiddlePanel {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, app: &App, area: Rect) {
        let mut tetrominos = Vec::new();
        tetrominos.push(StraightTetromino(TetrominoInfo::new(0, 0, TetrominoDirection::TOP, Color::LightBlue)));
        tetrominos.push(StraightTetromino(TetrominoInfo::new(10, 0, TetrominoDirection::RIGHT, Color::LightBlue)));
        tetrominos.push(StraightTetromino(TetrominoInfo::new(20, 0, TetrominoDirection::BOTTOM, Color::LightBlue)));
        tetrominos.push(StraightTetromino(TetrominoInfo::new(30, 0, TetrominoDirection::LEFT, Color::LightBlue)));
        tetrominos.push(SquareTetromino(TetrominoInfo::new(0, 5, TetrominoDirection::TOP, Color::Yellow)));
        tetrominos.push(SquareTetromino(TetrominoInfo::new(10, 5, TetrominoDirection::RIGHT, Color::Yellow)));
        tetrominos.push(SquareTetromino(TetrominoInfo::new(20, 5, TetrominoDirection::BOTTOM, Color::Yellow)));
        tetrominos.push(SquareTetromino(TetrominoInfo::new(30, 5, TetrominoDirection::LEFT, Color::Yellow)));
        tetrominos.push(TTetromino(TetrominoInfo::new(0, 8, TetrominoDirection::TOP, Color::Magenta)));
        tetrominos.push(TTetromino(TetrominoInfo::new(10, 8, TetrominoDirection::RIGHT, Color::Magenta)));
        tetrominos.push(TTetromino(TetrominoInfo::new(20, 8, TetrominoDirection::BOTTOM, Color::Magenta)));
        tetrominos.push(TTetromino(TetrominoInfo::new(30, 8, TetrominoDirection::LEFT, Color::Magenta)));
        tetrominos.push(LTetromino(TetrominoInfo::new(0, 12, TetrominoDirection::TOP, Color::Blue)));
        tetrominos.push(LTetromino(TetrominoInfo::new(10, 12, TetrominoDirection::RIGHT, Color::Blue)));
        tetrominos.push(LTetromino(TetrominoInfo::new(20, 12, TetrominoDirection::BOTTOM, Color::Blue)));
        tetrominos.push(LTetromino(TetrominoInfo::new(30, 12, TetrominoDirection::LEFT, Color::Blue)));
        tetrominos.push(JTetromino(TetrominoInfo::new(0, 20, TetrominoDirection::TOP, Color::Blue)));
        tetrominos.push(JTetromino(TetrominoInfo::new(10, 20, TetrominoDirection::RIGHT, Color::Blue)));
        tetrominos.push(JTetromino(TetrominoInfo::new(20, 20, TetrominoDirection::BOTTOM, Color::Blue)));
        tetrominos.push(JTetromino(TetrominoInfo::new(30, 20, TetrominoDirection::LEFT, Color::Blue)));
        tetrominos.push(SkewTetromino(TetrominoInfo::new(0, 25, TetrominoDirection::TOP, Color::Green)));
        tetrominos.push(SkewTetromino(TetrominoInfo::new(10, 25, TetrominoDirection::RIGHT, Color::Green)));
        tetrominos.push(SkewTetromino(TetrominoInfo::new(20, 25, TetrominoDirection::BOTTOM, Color::Green)));
        tetrominos.push(SkewTetromino(TetrominoInfo::new(30, 25, TetrominoDirection::LEFT, Color::Green)));
        tetrominos.push(ZTetromino(TetrominoInfo::new(0, 30, TetrominoDirection::TOP, Color::Green)));
        tetrominos.push(ZTetromino(TetrominoInfo::new(10, 30, TetrominoDirection::RIGHT, Color::Green)));
        tetrominos.push(ZTetromino(TetrominoInfo::new(20, 30, TetrominoDirection::BOTTOM, Color::Green)));
        tetrominos.push(ZTetromino(TetrominoInfo::new(30, 30, TetrominoDirection::LEFT, Color::Green)));
        let content = Layout::default()
            .constraints([Constraint::Length(5)])
            .split(area);
        let block = Block::default().borders(Borders::ALL);

        let canvas = Canvas::default()
            .block(block)
            .paint(|ctx| {
                for tetromino in tetrominos.iter() {
                    ctx.draw(tetromino);
                }
            })
            .marker(Marker::Block)
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0]);
        f.render_widget(canvas, content[0]);
    }
}


impl Shape for Tetromino {
    fn draw(&self, painter: &mut Painter) {
        match self {
            StraightTetromino(t) => {
                match t.direction() {
                    TetrominoDirection::TOP | TetrominoDirection::BOTTOM => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y(), t.color());
                        painter.paint(t.x() + 3, t.y(), t.color());
                    }
                    TetrominoDirection::LEFT | TetrominoDirection::RIGHT => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x(), t.y() + 2, t.color());
                        painter.paint(t.x(), t.y() + 3, t.color());
                    }
                }
            }
            SquareTetromino(t) => {
                painter.paint(t.x(), t.y(), t.color());
                painter.paint(t.x() + 1, t.y(), t.color());
                painter.paint(t.x(), t.y() + 1, t.color());
                painter.paint(t.x() + 1, t.y() + 1, t.color());
            }
            TTetromino(t) => {
                match t.direction() {
                    TetrominoDirection::TOP => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y(), t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                    }
                    TetrominoDirection::RIGHT => {
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                        painter.paint(t.x() + 1, t.y() + 2, t.color());
                        painter.paint(t.x(), t.y() + 1, t.color());
                    }
                    TetrominoDirection::BOTTOM => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y(), t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                    }
                    TetrominoDirection::LEFT => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x(), t.y() + 2, t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                    }
                }
            }
            LTetromino(t) => {
                match t.direction() {
                    TetrominoDirection::TOP => {
                        painter.paint(t.x(), t.y() + 2, t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                        painter.paint(t.x() + 1, t.y() + 2, t.color());
                    }
                    TetrominoDirection::RIGHT => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y() + 1, t.color());
                    }
                    TetrominoDirection::BOTTOM => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x(), t.y() + 2, t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                    }
                    TetrominoDirection::LEFT => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y() + 1, t.color());
                    }
                }
            }
            JTetromino(t) => {
                match t.direction() {
                    TetrominoDirection::TOP => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x(), t.y() + 2, t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                    }
                    TetrominoDirection::RIGHT => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y() + 1, t.color());
                    }
                    TetrominoDirection::BOTTOM => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                        painter.paint(t.x() + 2, t.y() + 1, t.color());
                    }
                    TetrominoDirection::LEFT => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 2, t.y(), t.color());
                    }
                }
            }
            SkewTetromino(t) => {
                match t.direction() {
                    TetrominoDirection::TOP | TetrominoDirection::BOTTOM => {
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                        painter.paint(t.x() + 2, t.y(), t.color());
                    }
                    TetrominoDirection::RIGHT | TetrominoDirection::LEFT => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                        painter.paint(t.x() + 1, t.y() + 2, t.color());

                    }
                }
            }

            ZTetromino(t) => {
                match t.direction() {
                    TetrominoDirection::TOP | TetrominoDirection::BOTTOM => {
                        painter.paint(t.x(), t.y(), t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                        painter.paint(t.x() + 2, t.y() + 1, t.color());
                    }
                    TetrominoDirection::RIGHT | TetrominoDirection::LEFT => {
                        painter.paint(t.x(), t.y() + 1, t.color());
                        painter.paint(t.x(), t.y() + 2, t.color());
                        painter.paint(t.x() + 1, t.y(), t.color());
                        painter.paint(t.x() + 1, t.y() + 1, t.color());
                    }
                }
            }
        }
    }
}
