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
        let content = Layout::default()
            .constraints([Constraint::Length(5)])
            .split(area);
        let block = Block::default().borders(Borders::ALL);

        let canvas = Canvas::default()
            .block(block)
            .paint(|ctx| {
                if let Some(t) = &app.active {
                    ctx.draw(t);
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
