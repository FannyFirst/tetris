use crate::app;
use crate::app::App;
use crate::runtime::tetromino::Tetromino;

pub struct Computer;

pub fn step(app: &mut App) {
    if let Some(t) = &mut app.active {
        t.step_y_down();
    } else {}
}

pub fn move_left(app: &mut App) {
    if let Some(t) = &mut app.active {
        t.step_x_left();
    }
}

pub fn move_right(app: &mut App) {
    if let Some(t) = &mut app.active {
        t.step_x_right();
    }
}

pub fn rotation(app: &mut App) {
    if let Some(t) = &mut app.active {
        t.step_next_direction();
    }
}
